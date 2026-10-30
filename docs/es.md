# InfraVet

**InfraVet** es una herramienta de línea de comandos (CLI) dirigida a profesionales de DevOps que permite escanear, analizar y validar archivos de configuración de infraestructura antes de que lleguen a producción o se incorporen a pipelines de CI/CD.

## Funcionalidades y estado

Actualmente, InfraVet permite **descubrir, interpretar y analizar estáticamente archivos Dockerfile**, aplicando reglas en las siguientes categorías:

- **Sintaxis y estructura** (`DF001`, `DF006`, `DF007`)
- **Buenas prácticas de seguridad** (`DF002`, `DF004`, `DF009`)
- **Construcción y rendimiento** (`DF005`, `DF010`)
- **Reproducibilidad y calidad** (`DF003`, `DF008`, `DF011`)

Para consultar el catálogo completo de reglas, sus descripciones y ejemplos, revisa [`docs/rules.md`](docs/rules.md).

---

## Uso

### Analizar archivos Dockerfile

Analiza recursivamente el directorio actual y muestra el informe en la consola:

```bash
cargo run -- scan
```

### Formatos de salida compatibles y exportación

Puedes seleccionar distintos formatos de salida (`cli`, `json`, `markdown`, `html`, `csv`, `junit`, `sarif`) y exportar los resultados directamente a un archivo mediante la opción `-o` / `--output`:

```bash
# Exportar el informe en formato JSON a un archivo específico
cargo run -- scan -f json -o report.json

# Exportar el informe en formato SARIF para pipelines de análisis de seguridad
cargo run -- scan -f sarif -o report.sarif

# Exportar el informe en formato CSV
cargo run -- scan -f csv -o report.csv
```

### Listar las reglas disponibles

Para mostrar todas las reglas compatibles:

```bash
cargo run -- rules
```

## Estructura del proyecto

```text
.
├── Cargo.toml
├── docs/             # Catálogo de reglas (Markdown y JSON)
└── src/
    ├── analyzers/    # Analizadores y reglas de infraestructura
    ├── cli.rs        # Definiciones de la interfaz de línea de comandos
    ├── commands/     # Implementación de comandos (scan, rules)
    ├── core/         # Abstracciones principales de reglas y hallazgos
    ├── models.rs     # Modelos y estructuras de datos compartidos
    ├── reports/      # Generadores de informes (CLI, JSON, SARIF, CSV, etc.)
    └── utils/        # Funciones auxiliares y estilos de salida
```

## Hoja de ruta

- [x] Descubrimiento recursivo de archivos y análisis de su contenido
- [x] Reglas de análisis estático para Dockerfile (`DF001` - `DF011`)
- [x] Múltiples formatos de informe (`cli`, `json`, `markdown`, `html`, `csv`, `junit`, `sarif`)
- [x] Exportación de informes a archivos (`-o` / `--output`)
- [ ] Compatibilidad con Docker Compose y Kubernetes
- [ ] Compatibilidad con Terraform y Ansible
- [ ] Reglas configurables e integración con CI/CD

## Licencia

Este proyecto se encuentra actualmente en desarrollo.
