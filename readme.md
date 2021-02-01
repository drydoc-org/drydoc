# drydoc

Drydoc is a modular documentation generator for projects of all kinds and sizes. It intends to be general purpose and allows many different documentation resources (e.g., C++, JS, markdown, videos, audio, PDFs, etc.) to exist in the same interface. Documentation is organized into a tree of pages. Pages may contain arbitrary content; the client determines how to render pages.

The generated website does not require any external resources and thus may be viewed without an internet
connection (assuming all contained resources also do not require the internet). It may be hosted with a simple
filesystem HTTP server (suitable for use with GitHub Pages, Apache servers, etc) or with `drydoc serve`. Due to
newer cross-origin restrictions in browsers, however, the documentation may not work when viewed directly from a `file://` URI.  

Drydoc is in the early stages of development and may not be suitable for your documentation needs. If you're interested
in contributing to a next-generation documentation system, we'd love your help!

## Configuration
A `drydoc.yaml` file is used to configure drydoc. Drydoc configurations can be hierarchical, allowing
large organization-level documentation pages or single project pages in a modular way.

## Compiling
Drydoc will compile and run on all major operating systems, including Linux, macOS, and Windows 10.

Dependencies:
  - [Node.js v14 or later](https://nodejs.org/en/)
  - [Yarn](https://yarnpkg.com/)
  - [Rust](https://rustup.rs/)

### Compiling

#### Backend
```.sh
cd /path/to/drydoc
cargo build
```

#### Frontend
```.sh
cd /path/to/drydoc/client
yarn run build
```

## Example Configuration Files
Drydoc is configured with a YAML file in your project's root called `drydoc.yaml`.

### C/C++ Documentation
Assuming headers are located in `include`:
```.yaml
type: generate
id: my_project
using: clang@1.0
with:
  name: "My Project"
  path: include
```

### Markdown Book
```.yaml
type: generate
id: book
using: copy@1.0
with:
  name: "My Book"
  path: index.md
children:
  - type: generate
    id: chapter_one
    using: copy@1.0
    with:
      name: "Chapter One"
      path: chapter_one.md
  - type: generate
    id: chapter_two
    using: copy@1.0
    with:
      name: "Chapter Two"
      path: chapter_two.md
  - type: import
    path: appendix/drydoc.yaml
```

## Running
Assuming `drydoc/target/debug` is in the system path:

```.sh
cd /path/to/project
# Assuming drydoc.yaml is in the project root. Use --help for options.
drydoc gen

# Documentation, by default, is output into a directory called "html".
# Serve the generated documentation. Use --help for options.
drydoc serve

# Now navigate to localhost:8888 in your browser!
```

## Packages
Drydoc provides a package manager for managing installed generator backends and renderer frontends. These are installed
automatically when encountered in a `drydoc.yaml` configuration file. To read more about package management, including
how to use it while developing new packages, see `crates/drydoc-pkg-manager/readme.md`.

## Contributing
Drydoc intends to be a comprehensive documentation solution capable of supplanting current industry-standard
documentation tools like doxygen. If you believe in our vision and have some spare cycles, we'd love your help!
See the Github Issues for ideas on how to jump in to development. There's a lot to do!

### Key Project Goals
  - Modular frontend and backend backed by a package manager to enable an ecosystem of documentation renderers and generators.
  - Compatibility with common documentation hosting methods (e.g., GitHub Pages).
  - Rich frontend built on a modern software stack.
  - Leverage existing standards (e.g., doxygen-style C++ comments, markdown, etc.) where possible to make transitioning easy.

## License
Drydoc is released under the terms of the BSD 3-Clause license.