# InfraVet

InfraVet is a command-line interface (CLI) tool designed for DevOps and Platform Engineering professionals. It allows you to discover, analyze, and validate infrastructure configuration files before they reach production or are integrated into CI/CD pipelines.

## Features and Status

Currently, InfraVet enables the discovery, interpretation, and static analysis of Dockerfile files, applying rules across the following categories:

- **Syntax and structure** (`DF001`, `DF006`, `DF007`)
- **Security best practices** (`DF002`, `DF004`, `DF009`)
- **Build and performance** (`DF005`, `DF010`)
- **Reproducibility and quality** (`DF003`, `DF008`, `DF011`)

The analysis process includes:

- Recursive discovery of infrastructure files.
- Parallel file analysis using Rayon.
- Centralized consolidation of analysis results.
- Tracking of files found, successfully analyzed, and failed.
- Classification of findings by severity: `ERROR`, `WARNING`, and `INFO`.
- Report generation in multiple formats.

To consult the complete rule catalog, descriptions, and examples, check `docs/rules.md`.

---

## Usage

### Analyze Dockerfile Files

Recursively analyze the current directory and display a general summary in the console:

```bash
cargo run -- scan
```

The summary presents the analysis status, processed file metrics, and the number of findings discovered by severity.

Possible analysis statuses are:

- **SCAN COMPLETED**: All files found were successfully analyzed.
- **PARTIAL SCAN**: Some files could not be analyzed.
- **SCAN FAILED**: No files found could be successfully analyzed.
- **NO FILES FOUND**: No files compatible with the analysis targets were found.

A completed analysis does not necessarily mean the infrastructure is problem-free: the result may still contain findings with `ERROR` severity.

### Supported Output Formats and Exporting

InfraVet supports the following formats:

- `cli`: General summary for the console.
- `json`: Structured report in JSON.
- `markdown`: Report in Markdown.
- `html`: Report in HTML.
- `csv`: Tabular report.
- `junit`: Report compatible with continuous integration tools.
- `sarif`: Standardized format for code analysis and security tools.

You can export results directly to a file using the `-o` / `--output` option:

```bash
# Export the report in JSON format
cargo run -- scan -f json -o report.json

# Export the report in SARIF format
cargo run -- scan -f sarif -o report.sarif

# Export the report in CSV format
cargo run -- scan -f csv -o report.csv

# Export the report in HTML format
cargo run -- scan -f html -o report.html
```

The `cli` format is oriented toward providing a general overview of the analysis, while the export formats allow querying and processing individual findings.

### List Available Rules

To display the catalog of implemented rules:

```bash
cargo run -- rules
```

---

## Project Structure

```text
.
├── Cargo.toml
├── Cargo.lock
├── docs/                 # Rule catalog (Markdown and JSON)
└── src/
    ├── analyzers/         # Infrastructure analyzers and rules
    ├── cli.rs             # Command-line interface definitions
    ├── commands/          # Command implementations
    ├── core/              # Core abstractions for rules and findings
    ├── main.rs            # Application entry point
    ├── models.rs          # Shared data models and structures
    ├── reports/
    │   ├── formatters/    # Report generators
    │   ├── output.rs      # Presentation and output utilities
    │   └── scan.rs        # Analysis process coordination
    └── utils/             # Helper functions
```

---

## Roadmap

- [x] Recursive discovery of infrastructure files.
- [x] Static analysis of Dockerfile.
- [x] Implementation of Dockerfile rules (DF001 - DF011).
- [x] Parallel file processing using Rayon.
- [x] Consolidation of results and analysis metrics.
- [x] Classification of findings by severity.
- [x] Multiple report formats (`cli`, `json`, `markdown`, `html`, `csv`, `junit`, `sarif`).
- [x] Report exporting to files (`-o` / `--output`).
- [ ] Full Docker Compose compatibility.
- [ ] Kubernetes compatibility.
- [ ] Terraform compatibility.
- [ ] Ansible compatibility.
- [ ] Configurable rules.
- [ ] CI/CD pipeline integration.

---

## License

This project is currently under development.
