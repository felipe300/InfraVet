# InfraVet

**InfraVet** is a CLI tool for DevOps engineers designed to scan, analyze, and validate infrastructure configuration files.

The long-term goal is to provide a single tool capable of discovering and validating configuration files commonly used in DevOps and infrastructure projects, including:

- Dockerfiles
- Docker Compose files
- Kubernetes manifests
- Terraform files
- Helm charts
- Other infrastructure configuration files

## Current Status

InfraVet is currently in **early development** and focuses exclusively on **Dockerfiles**.

The first stage of the project is to build a reliable file discovery system capable of recursively finding Dockerfiles inside a project directory.

Once file discovery is working, the project will evolve toward Dockerfile parsing and validation.

The validation process will eventually be divided into two main levels:

1. **Syntax validation**
2. **Structural and best-practice validation**

---

## Dockerfile Support

The first version of InfraVet focuses on discovering files named:

```text
Dockerfile
```

InfraVet will recursively scan a directory and identify Dockerfiles located throughout the project.

For example:

```bash
my-project/
├── Dockerfile
├── backend/
│   ├── Dockerfile
│   └── src/
├── frontend/
│   ├── Dockerfile
│   └── src/
└── infrastructure/
    └── Dockerfile
```

InfraVet should be able to discover:

```bash
./Dockerfile
./backend/Dockerfile
./frontend/Dockerfile
./infrastructure/Dockerfile
```

### Current CLI

The initial CLI command is intended to provide Dockerfile discovery:

```bash
infravet search --filename Dockerfile
```

The command should recursively search the specified project directory and return the paths of discovered Dockerfiles.

---

## Dockerfile Validation

Once Dockerfile discovery is implemented, InfraVet will introduce Dockerfile validation.

Validation will be divided into multiple levels.

### 1. Syntax Validation

The first validation level will determine whether a Dockerfile is syntactically valid.

For example, InfraVet could detect:

- Invalid Dockerfile instructions
- Malformed instructions
- Invalid instruction arguments
- Other syntax-related errors

Example output:

```bash
Dockerfile
✓ Syntax valid
```

Or:

```bash
Dockerfile
✗ Syntax error
  Line 5: invalid instruction
```

---

### 2. Structural Validation

A syntactically valid Dockerfile is not necessarily well structured or efficient.

The second validation level will analyze the structure of the Dockerfile and identify potential problems or questionable practices.

Potential checks include:

- Missing `FROM` instruction
- Incorrect instruction ordering
- Unnecessary image layers
- Use of the `latest` tag
- Inefficient package installation
- Missing `.dockerignore`
- Running containers as `root`
- Poor layer caching
- Unnecessary files copied into the image
- Other Dockerfile best-practice violations

The goal is not only to determine whether a Dockerfile **works**, but also whether it follows reasonable practices for production environments.

---

## Planned CLI

The CLI will evolve as new functionality is implemented.

The initial command:

```bash
infravet search --filename Dockerfile
```

could eventually evolve into:

```bash
infravet check .
```

The `check` command would automatically discover and validate Dockerfiles throughout the project.

#### Example

```bash
Scanning project...

Dockerfiles found:

✓ ./Dockerfile
✓ ./backend/Dockerfile
✓ ./frontend/Dockerfile

Validation:

./Dockerfile
✓ Syntax
✓ Structure

./backend/Dockerfile
✓ Syntax
⚠ Uses latest image tag

./frontend/Dockerfile
✗ Syntax error
  Line 8: invalid instruction

Results:
1 error
1 warning
```

---

## Project Structure

InfraVet is currently being developed with a simple modular structure:

```bash
src/
├── main.rs
├── cli.rs
└── commands/
    ├── mod.rs
    └── search.rs
```

As new functionality is introduced, the project structure will evolve.

The goal is to keep the main components separated:

- **CLI** — command-line interface and argument handling
- **File discovery** — locating infrastructure files
- **Parsing** — reading and interpreting configuration files
- **Validation** — checking syntax, structure, and best practices
- **Reporting** — presenting errors, warnings, and results

This separation should make it easier to support additional DevOps technologies in the future.

---

## Roadmap

### Dockerfile

- [x] Create the initial CLI structure
- [ ] Search recursively for Dockerfiles
- [ ] Return discovered Dockerfile paths
- [ ] Parse Dockerfiles
- [ ] Validate Dockerfile syntax
- [ ] Validate Dockerfile structure
- [ ] Add Dockerfile best-practice rules
- [ ] Improve error and warning reporting
- [ ] Add automated tests
- [ ] Improve CLI output

### Future Support

Once Dockerfile functionality is stable, support for additional infrastructure technologies is planned:

- Docker Compose
- Kubernetes
- Terraform
- Helm
- Other DevOps and infrastructure configuration files

---

## Goal

InfraVet aims to become a practical CLI tool for identifying and validating infrastructure configuration before it reaches production or CI/CD pipelines.

The project is currently focused on building the core architecture around **Dockerfile discovery, parsing, and validation**.

Once that foundation is stable, InfraVet can expand to other DevOps technologies while maintaining the same general workflow:

```text
Discover → Parse → Validate → Report
```
