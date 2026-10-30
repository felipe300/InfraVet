# InfraVet

**InfraVet** is a CLI tool for DevOps engineers to scan, analyze, and validate infrastructure configuration files before they reach production or CI/CD pipelines.

## Features & Status

InfraVet currently supports **Dockerfile discovery, parsing, and static analysis**, enforcing rules across:

- **Syntax & Structure** (`DF001`, `DF006`, `DF007`)
- **Security Best Practices** (`DF002`, `DF004`, `DF009`)
- **Build & Performance** (`DF005`, `DF010`)
- **Reproducibility & Quality** (`DF003`, `DF008`, `DF011`)

For a complete catalog of rules, descriptions, and examples, see [`docs/rules.md`](docs/rules.md).

---

## Usage

### Scan Dockerfiles

Scan the current directory recursively and print the report to the console:

```bash
cargo run -- scan
```

### Supported Output Formats & Exporting

You can select different output formats (`cli`, `json`, `markdown`, `html`, `csv`, `junit`, `sarif`) and export the results directly to a file using the `-o` / `--output` flag:

```bash
# Export as JSON to a specific file
cargo run -- scan -f json -o report.json

# Export as SARIF for security analysis pipelines
cargo run -- scan -f sarif -o report.sarif

# Export as CSV
cargo run -- scan -f csv -o report.csv
```

### List Available Rules

To display all supported rules:

```bash
cargo run -- rules
```

## Project Structure

```bash
.
├── Cargo.toml
├── docs/             # Rule catalog (Markdown & JSON)
└── src/
    ├── analyzers/    # Infrastructure parsers and rules
    ├── cli.rs        # Command-line interface definitions
    ├── commands/     # Command implementations (scan, rules)
    ├── core/         # Core rules and issues abstractions
    ├── models.rs     # Shared models and data structures
    ├── reports/      # Output formatters (CLI, JSON, SARIF, CSV, etc.)
    └── utils/        # Helpers and output styling
```

## Roadmap

- [x] Recursive file discovery and parsing
- [x] Dockerfile static analysis rules (`DF001` - `DF011`)
- [x] Multiple report formats (`cli`, `json`, `markdown`, `html`, `csv`, `junit`, `sarif`)
- [x] File export support (`-o` / `--output`)
- [ ] Docker Compose & Kubernetes support
- [ ] Terraform & Ansible support
- [ ] Configurable rules and CI/CD integration

## License

This project is currently under development.
