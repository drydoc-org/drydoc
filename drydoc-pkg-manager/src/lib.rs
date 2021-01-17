use serde::{Serialize, Deserialize};

use std::{collections::{HashMap, HashSet}, io::BufRead, path::{Path, PathBuf}};

use std::error::Error;
use async_trait::async_trait;

use std::io::Read;

use derive_more::{Display, Error};

use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Version {
  pub major: u64,
  pub minor: u64,
  pub patch: u64
}

impl From<semver::Version> for Version {
  fn from(value: semver::Version) -> Self {
    Self {
      major: value.major,
      minor: value.minor,
      patch: value.patch
    }
  }
}

impl From<Version> for semver::Version {
  fn from(value: Version) -> Self {
    Self::new(value.major, value.minor, value.patch)
  }
}

impl std::fmt::Display for Version {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
  }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetTriple {
  pub machine: String,
  pub vendor: String,
  pub os: String
}

impl TargetTriple {
  pub fn this_machine() -> Self {
    let target = env!("TARGET");
    let parts: Vec<&str> = target.split('-').collect();
    Self {
      machine: parts[0].to_string(),
      vendor: parts[1].to_string(),
      os: parts[2 ..].join("-")
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Package {
  pub name: String,
  pub versions: Vec<PackageVersion>
}

#[derive(Serialize, Deserialize)]
pub struct ArtifactReference {
  pub url: String,
  pub sha256: String
}

#[derive(Serialize, Deserialize)]
pub struct Artifact {
  pub entrypoint: String
}

#[derive(Serialize, Deserialize)]
pub struct PackageVersion {
  pub version: Version,
  pub target_artifacts: HashMap<TargetTriple, ArtifactReference>
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
  pub packages: HashSet<String>
}

#[async_trait]
pub trait Fetcher {
  async fn get_repository(&self) -> Result<Repository, Box<dyn Error>>;
  async fn get_package(&self, name: &str) -> Result<Package, Box<dyn Error>>;
  async fn get_artifact(&self, artifact_ref: &ArtifactReference) -> Result<Box<[u8]>, Box<dyn Error>>;
}

pub struct UrlFetcher {
  base: String
}

#[derive(Display, Debug, Error)]
pub enum GetArtifactError {
  ChecksumMismatch
}

#[async_trait]
impl Fetcher for UrlFetcher {
  async fn get_repository(&self) -> Result<Repository, Box<dyn Error>> {
    let res = reqwest::get(format!("{}/repository.json", self.base).as_str())
      .await?
      .error_for_status()?;
    Ok(serde_json::from_str(res.text().await?.as_str())?)
  }

  async fn get_package(&self, name: &str) -> Result<Package, Box<dyn Error>> {
    let res = reqwest::get(format!("{}/{}/package.json", self.base, name).as_str())
      .await?
      .error_for_status()?;
    Ok(serde_json::from_str(res.text().await?.as_str())?)
  }

  async fn get_artifact(&self, artifact_ref: &ArtifactReference) -> Result<Box<[u8]>, Box<dyn Error>> {
    let res = reqwest::get(artifact_ref.url.as_str())
      .await?
      .error_for_status()?;
    let mut hasher = Sha256::default();
    
    let bytes = res.bytes().await?;
    hasher.update(&bytes);
    let result = hasher.finalize();

    if hex::encode(result) != artifact_ref.sha256 {
      return Err(Box::new(GetArtifactError::ChecksumMismatch))
    }
    
    Ok(bytes.to_vec().into_boxed_slice())
  }
}

#[derive(Serialize, Deserialize)]
struct RemoteCache {
  repository: Repository,
  packages: HashMap<String, Package>
}

impl RemoteCache {
  pub fn new() -> Self {
    Self {
      repository: Repository {
        packages: HashSet::new()
      },
      packages: HashMap::new()
    }
  }
}

#[derive(Display, Debug, Error)]
pub enum GetError {
  PackageNotFound,
  VersionNotFound
}

pub struct Manager<F: Fetcher> {
  fetcher: F,
  dir: PathBuf,
  remote_cache: Option<RemoteCache>
}

impl<F: Fetcher> Manager<F> {
  pub fn new<P: AsRef<Path>>(fetcher: F, dir: P) -> Self {
    Self {
      fetcher,
      dir: dir.as_ref().to_path_buf(),
      remote_cache: None
    }
  }

  pub async fn update(&mut self) -> Result<(), Box<dyn Error>> {
    if let Some(_) = self.remote_cache {
      return Ok(());
    }

    let mut remote_cache = RemoteCache::new();

    remote_cache.repository = self.fetcher.get_repository().await?;

    for package in remote_cache.repository.packages.iter() {
      let package = self.fetcher.get_package(package).await?;
      remote_cache.packages.insert(package.name.clone(), package);
    }

    self.remote_cache = Some(remote_cache);

    Ok(())
  }

  pub async fn get(&mut self, name: &str, version_req: &semver::VersionReq) -> Result<(PathBuf, Artifact), Box<dyn Error>> {
    self.update().await?;

    let RemoteCache { repository, packages } = self.remote_cache.as_ref().unwrap();

    let package = packages.get(&name.to_string());

    if let Some(package) = package {
      let mut matching_versions = Vec::new();

      let triple = TargetTriple::this_machine();

      for package_version in package.versions.iter() {
        if !package_version.target_artifacts.contains_key(&triple) {
          continue;
        }

        if version_req.matches(&package_version.version.clone().into()) {  
          matching_versions.push(package_version);
        }
      }

      if matching_versions.is_empty() {
        return Err(Box::new(GetError::VersionNotFound));
      }

      let mut iter = matching_versions.into_iter();
      let mut best_match = iter.next().unwrap();

      for m in iter {
        if m.version > best_match.version {
          best_match = m;
        }
      }

      let mut dir = self.dir.clone();

      dir.push(name);
      dir.push(best_match.version.to_string());

      if !dir.is_dir() {
        let artifact_ref = best_match.target_artifacts.get(&triple).unwrap();
        
        let bytes = self.fetcher.get_artifact(&artifact_ref).await?;

        let stream = std::io::Cursor::new(bytes);
        let mut archive = tar::Archive::new(compress::lz4::Decoder::new(stream));

        std::fs::create_dir_all(&dir)?;
        archive.unpack(&dir)?;
      }

      dir.push("artifact.json");
      let artifact = std::fs::read_to_string(&dir)?;
      let artifact: Artifact = serde_json::from_str(artifact.as_str())?;
      dir.pop();
      
      Ok((dir, artifact))
    } else {
      Err(Box::new(GetError::PackageNotFound))
    }
  }
}
