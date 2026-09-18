# InfraVet

**InfraVet** is a CLI tool for DevOps engineers designed to scan, analyze, and validate infrastructure configuration files before they reach production or CI/CD pipelines.

## Features & Status

InfraVet is currently focused on **Dockerfile discovery, parsing, and rule-based static analysis**.

### Supported Validation

- **Syntax Validation:** Detects empty Dockerfiles, missing `FROM` instructions, and invalid Dockerfile syntax.
- **Security Analysis:** Identifies potentially unsafe practices such as running containers as root or embedding sensitive data in `ENV` and `ARG` instructions.
- **Best-Practice Analysis:** Detects Dockerfile patterns that can be improved for clarity, maintainability, and predictability.
- **Build & Performance Analysis:** Identifies package-management patterns that may lead to stale indexes or unnecessarily large image layers.
- **Reproducibility Analysis:** Detects practices such as using mutable base-image tags that can make builds less predictable.

---

## Analysis Rules

InfraVet enforces linting rules categorized by severity:

| Severity    | Description                                                                                                     |
| ----------- | --------------------------------------------------------------------------------------------------------------- |
| **ERROR**   | Issues that may prevent a Dockerfile from being correctly built or may represent significant security concerns. |
| **WARNING** | Potential security, build, performance, or best-practice issues.                                                |
| **INFO**    | Recommendations that can improve reproducibility or operational reliability.                                    |

For the complete list of rules, descriptions, recommendations, and examples, see [`docs/rules.md`](docs/rules.md).

### Rule Catalog

| Rule ID   | Name                                      | Severity | Category        | Description                                                                                              |
| --------- | ----------------------------------------- | -------- | --------------- | -------------------------------------------------------------------------------------------------------- |
| **DF001** | Missing `FROM` instruction                | ERROR    | Syntax          | A Dockerfile must contain at least one `FROM` instruction to define a build stage.                       |
| **DF002** | Copying entire build context              | WARNING  | Security        | Detects `COPY . .`, which may copy unnecessary or sensitive files into the image.                        |
| **DF003** | Base image uses `latest` tag              | INFO     | Reproducibility | Detects base images using the mutable `latest` tag.                                                      |
| **DF004** | Missing `USER` instruction                | WARNING  | Security        | Detects stages without a `USER` instruction, which may cause the container to run as root.               |
| **DF005** | Unlinked `apt-get update`                 | WARNING  | Build           | Detects `apt-get update` without a corresponding `apt-get install` in the same `RUN` instruction.        |
| **DF006** | Empty Dockerfile                          | ERROR    | Syntax          | Detects Dockerfiles containing no instructions or only whitespace.                                       |
| **DF007** | Invalid Dockerfile syntax                 | ERROR    | Syntax          | Detects Dockerfile syntax or instruction errors that prevent correct parsing.                            |
| **DF008** | Use `COPY` instead of `ADD`               | WARNING  | Best Practice   | Detects `ADD` usage for local files or directories where `COPY` is more explicit.                        |
| **DF009** | Possible sensitive data in `ENV` or `ARG` | ERROR    | Security        | Detects keywords commonly associated with credentials or sensitive data in `ENV` and `ARG` instructions. |
| **DF010** | Uncleaned package manager caches          | WARNING  | Performance     | Detects package installation patterns that may leave unnecessary package-manager caches in the image.    |
| **DF011** | Missing `HEALTHCHECK` instruction         | INFO     | Best Practice   | Detects Dockerfiles without a `HEALTHCHECK` instruction when one may be useful.                          |

---

## Usage

### Scan Dockerfiles

InfraVet recursively scans the current directory for Dockerfiles and runs the available static-analysis rules:

```bash
cargo run -- scan
```

You can also build and run the binary directly:

```bash
cargo build --release
./target/release/infravet scan
```

### List Available Rules

To display the rules supported by InfraVet:

```bash
cargo run -- rules
```

---

## Project Structure

```bash
.
├── Cargo.lock
├── Cargo.toml
├── docs
│   ├── rules.json
│   └── rules.md
├── README.md
└── src
    ├── analyzers
    │   ├── dockerfile
    │   │   ├── context.rs
    │   │   ├── mod.rs
    │   │   ├── parser.rs
    │   │   └── rules
    │   │       ├── df001.rs
    │   │       ├── df002.rs
    │   │       ├── df003.rs
    │   │       ├── df004.rs
    │   │       ├── df005.rs
    │   │       ├── df006.rs
    │   │       ├── df007.rs
    │   │       ├── df008.rs
    │   │       ├── df009.rs
    │   │       ├── df010.rs
    │   │       ├── df011.rs
    │   │       ├── execution_rules.rs
    │   │       ├── instruction_rules.rs
    │   │       └── mod.rs
    │   └── mod.rs
    ├── cli.rs
    ├── commands
    │   ├── mod.rs
    │   ├── rules.rs
    │   └── scan.rs
    ├── core
    │   ├── issue.rs
    │   ├── mod.rs
    │   └── rule.rs
    ├── main.rs
    ├── models.rs
    └── utils
        ├── mod.rs
        ├── output.rs
        └── utils.rs
```

---

## Main Components

- **CLI (`cli.rs`):** Defines the command-line interface and arguments.
- **Commands (`commands/`):** Implements CLI commands such as `scan` and `rules`.
- **Dockerfile Analyzer (`analyzers/dockerfile/`):** Handles Dockerfile discovery, parsing, context construction, and rule execution.
- **Rules (`analyzers/dockerfile/rules/`):** Contains the individual Dockerfile validation rules.
- **Core (`core/`):** Provides the core abstractions for rules and reported issues.
- **Models (`models.rs`):** Contains shared application data structures.
- **Utils (`utils/`):** Provides output formatting and shared utility functions.
- **Documentation (`docs/`):** Contains the rule catalog in Markdown and JSON formats.

---

## Rule Architecture

Dockerfile analysis is organized around **independent rules**. Each rule evaluates information extracted by the Dockerfile parser and may report one or more issues.

The general analysis flow is:

```text
Dockerfile
    │
    ▼
Discovery
    │
    ▼
Parser
    │
    ▼
Dockerfile Context
    │
    ▼
Rule Evaluation
    │
    ├── DF001
    ├── DF002
    ├── DF003
    ├── ...
    └── DF011
    │
    ▼
Issues
    │
    ▼
CLI Output
```

This structure allows new Dockerfile rules to be added independently without changing the overall analysis pipeline.

---

## Roadmap

### Dockerfile

- [x] Recursive Dockerfile discovery
- [x] Dockerfile parsing
- [x] Structural and syntax validation (`DF001`, `DF006`, `DF007`)
- [x] Security rule validation (`DF002`, `DF004`, `DF009`)
- [x] Build and performance validation (`DF005`, `DF010`)
- [x] Best-practice validation (`DF008`, `DF011`)
- [x] Reproducibility validation (`DF003`)
- [x] Issue reporting
- [ ] Automated unit tests
- [ ] Automated integration tests
- [ ] Configurable rule severity
- [ ] Rule enable/disable configuration
- [ ] Flexible configuration support (e.g. `.infravet.toml`)
- [ ] CI/CD integration
- [ ] Multiple output formats (JSON, SARIF, etc.)

### Future Technologies

- [ ] Docker Compose
- [ ] Kubernetes manifests
- [ ] Terraform files
- [ ] YAML files (GitHub Actions, GitLab CI/CD)
- [ ] Additional infrastructure configuration formats

---

## License

This project is currently under development.
