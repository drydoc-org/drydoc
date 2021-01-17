# `drydoc-gen`

`drydoc gen` generates a documentation website based on a configuration file.

## Command Line Arguments

- `--config [config_file]` (`-c`) - Generates documentation based on the given configuration file (default: `drydoc.yaml`).
- `--output [path]` (`-o`) - Output the resulting website to the given path (default: `html`).

## Sample Configuration Files

### Generate C++ documentation

Assuming C++ headers are located in the `include` directory:
```.yaml
---
type: generate
id: home_page
using: clang@^1.0.0
with:
  name: "My API Documentation"
  path: include
```

### Generate Markdown documentation

```.yaml
---
type: generate
id: readme
using: copy@^1.0.0
with:
  name: "Read Me"
  path: README.md
```

### Generate C++ and Markdown documentation

```.yaml
---
type: generate
id: readme
using: copy@^1.0.0
with:
  name: "Read Me"
  path: README.md
children:
  -
    type: generate
    id: api_docs
    using: clang@^1.0.0
    with:
      name: "My API Documentation"
      path: include
```

### Compose Multiple Configuration Files

```.yaml
---
type: generate
id: readme
using: copy@^1.0.0
with:
  name: "Read Me"
  path: README.md
children:
  - type: import
    path: lib/drydoc.yaml
  - type: import
    path: examples/drydoc.yaml
```

## Example Commands

### `drydoc gen`

Generate documentation based on the `drydoc.yaml` configuration file in the working directory, outputing the resulting website to the `html` directory.

### `drydoc gen -o docs`

Generate documentation based on the `drydoc.yaml` configuration file in the working directory, outputing the resulting website to the `docs` directory.

### `drydoc gen -c my_config.yaml`

Generate documentation based on the `my_config.yaml` configuration file in the working directory, outputing the resulting website to the `html` directory.

