# InfraVet

**InfraVet** is a CLI tool for DevOps engineers designed to scan, analyze, and validate infrastructure configuration files before they reach production or CI/CD pipelines.

---

## Features & Status

InfraVet is in active development, focusing on **Dockerfile discovery, parsing, and rule-based validation**.

### Supported Validation

- **Syntax Validation**: Ensures Dockerfiles are parsable and syntactically sound.
- **Structural & Best-Practice Analysis**: Identifies security risks, bad practices, and sub-optimal layer patterns.

---

## Analysis Rules

InfraVet enforces linting rules categorized by severity (`ERROR`, `WARNING`, `INFO`).

For the complete list of rules, descriptions, and examples, see [`rules.md`](rules.md).

| Rule ID   | Name                                | Severity  | Description                                                                          |
| --------- | ----------------------------------- | --------- | ------------------------------------------------------------------------------------ |
| **DF001** | Missing `FROM` instruction          | `ERROR`   | A Dockerfile must define at least one build stage.                                   |
| **DF002** | Copying entire context (`COPY . .`) | `WARNING` | May expose sensitive files and degrade build cache efficiency.                       |
| **DF003** | Use of `latest` tag                 | `INFO`    | Base image uses the `latest` tag instead of an explicit version.                     |
| **DF004** | Missing `USER` instruction          | `WARNING` | Container runs as root, introducing security risks.                                  |
| **DF005** | Unlinked `apt-get update`           | `WARNING` | `apt-get update` without `apt-get install` in the same layer causes cache staleness. |
| **DF006** | Empty Dockerfile                    | `ERROR`   | File contains no instructions or content.                                            |
| **DF007** | Invalid Dockerfile syntax           | `ERROR`   | Failed to parse Dockerfile structure.                                                |

---

## Usage

### Search & Analyze Dockerfiles

Recursively scans the current directory and executes automated static analysis on all matched files:

```bash
cargo run -- search
```

## Project Structure

```bash
├── Cargo.lock
├── Cargo.toml
├── docs
│   └── rules.md
├── README.md
├── src
    ├── analyzer
    │   ├── dockerfile.rs
    │   ├── issue.rs
    │   └── mod.rs
    ├── cli.rs
    ├── commands
    │   ├── mod.rs
    │   └── search.rs
    ├── main.rs
    ├── models.rs
    └── utils
        ├── mod.rs
        ├── output.rs
        └── utils.rs
```

- **CLI** (`cli.rs, commands/`): Command-line argument parsing and execution flow.
- **Analyzer & Rules**: Parsing Dockerfiles and evaluating best-practice rules.

## Roadmap

**Dockerfile**

- [x] Recursive Dockerfile discovery
- [x] Dockerfile syntax parsing
- [x] Structural & security rule validation (`DF001 - DF007`)
- [x] Issue & error reporting output
- [ ] Automated integration & unit tests
- [ ] Flexible configuration support (`e.g., .infravet.toml`)

**Future Technologies**

- [ ] Docker Compose
- [ ] Kubernetes Manifests
- [ ] Terraform Files
- [ ] YAML files (GitHub Action, GitLab)
