#[async_trait::async_trait]
pub trait AsyncFrom<T> {
  async fn from(t: T) -> Self;
}