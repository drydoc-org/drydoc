# drydoc
Drydoc generates beautiful and accessible documentation websites for projects. It intends to be general purpose and allows many different documentation resources (e.g., C++, markdown, videos, audio, PDFs, etc.) to exist in the same interface. Documentation is organized into a tree of pages. Pages may contain arbitrary content; the client determines how to render pages.

The generated website does not require any external resources, and thus may be viewed without an internet
connection (assuming all contained resources also do not require the internet). It can be hosted with a simple filesystem
HTTP server (suitable for use with GitHub Pages, Apache servers, etc).

Drydoc is in the early stages of development and may not be suitable for your documentation needs. If you're interested
in contributing to a next-generation documentation system, we'd love your help!

## Configuration
A `drydoc.yaml` file is used to configure drydoc. Drydoc configurations can be hierarchical, allowing
large organizational documentation pages or single project pages in a modular way.

## Compiling
Drydoc will compile and run on all major operating systems, including Linux, macOS, and Windows 10.

Dependencies:
  - [Node.js v14 or later](https://nodejs.org/en/)
  - [Yarn](https://yarnpkg.com/)
  - [Rust](https://rustup.rs/)
  - [Clang v10 or later](https://clang.llvm.org/)

### Compiling the Server
```.sh
cd /path/to/drydoc
cargo build
```

### Compiling the Client
```.sh
cd /path/to/drydoc/client
yarn run build
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

## Contributing
Drydoc intends to be a comprehensive documentation solution capable of supplanting current industry-standard
documentation tools like doxygen. If you believe in our vision and have some spare cycles, we'd love your help!
See the Github Issues for ideas on how to jump in to development. There's a lot to do!

### Key Project Goals
  - Compatibility with common documentation hosting methods (e.g., GitHub Pages).
  - Rich client built on a modern software stack.
  - Modular generator architecture for programming languages and other file types that enables a vast ecosystem.
  - Leverage existing standards (e.g., doxygen-style C++ comments, markdown, etc.) to make transitioning as easy as possible.

## License
Drydoc is released under the terms of the BSD 3-Clause license.