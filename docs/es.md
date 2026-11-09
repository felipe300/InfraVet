# InfraVet

**InfraVet** es una herramienta de línea de comandos (CLI) orientada a profesionales de DevOps y Platform Engineering. Permite descubrir, analizar y validar archivos de configuración de infraestructura antes de que lleguen a producción o se incorporen a pipelines de CI/CD.

## Funcionalidades y estado

Actualmente, InfraVet permite **descubrir, interpretar y analizar estáticamente archivos Dockerfile**, aplicando reglas en las siguientes categorías:

- **Sintaxis y estructura** (`DF001`, `DF006`, `DF007`)
- **Buenas prácticas de seguridad** (`DF002`, `DF004`, `DF009`)
- **Construcción y rendimiento** (`DF005`, `DF010`)
- **Reproducibilidad y calidad** (`DF003`, `DF008`, `DF011`)

El proceso de análisis contempla:

- Descubrimiento recursivo de archivos de infraestructura.
- Análisis de archivos en paralelo mediante **Rayon**.
- Consolidación centralizada de los resultados del análisis.
- Seguimiento de archivos encontrados, analizados correctamente y fallidos.
- Clasificación de hallazgos según su severidad: `ERROR`, `WARNING` e `INFO`.
- Generación de informes en distintos formatos.

Para consultar el catálogo completo de reglas, sus descripciones y ejemplos, revisa [`docs/rules.md`](./rules.md).

---

## Uso

### Analizar archivos Dockerfile

Analiza recursivamente el directorio actual y muestra un resumen general en la consola:

```bash
cargo run -- scan
```

El resumen presenta el estado del análisis, las métricas de archivos procesados y la cantidad de hallazgos encontrados por severidad.

Los estados posibles del análisis son:

- `SCAN COMPLETED`: todos los archivos encontrados fueron analizados correctamente.
- `PARTIAL SCAN`: algunos archivos no pudieron analizarse.
- `SCAN FAILED`: ningún archivo encontrado pudo analizarse correctamente.
- `NO FILES FOUND`: no se encontraron archivos compatibles con los objetivos del análisis.

Un análisis completado no significa necesariamente que la infraestructura esté libre de problemas: el resultado también puede contener hallazgos de severidad `ERROR`.

### Formatos de salida compatibles y exportación

InfraVet admite los siguientes formatos:

- `cli`: resumen general para la consola.
- `json`: informe estructurado en JSON.
- `markdown`: informe en Markdown.
- `html`: informe en HTML.
- `csv`: informe tabular.
- `junit`: informe compatible con herramientas de integración continua.
- `sarif`: formato estandarizado para herramientas de análisis de código y seguridad.

Puedes exportar los resultados directamente a un archivo mediante la opción `-o` / `--output`:

```bash
# Exportar el informe en formato JSON
cargo run -- scan -f json -o report.json

# Exportar el informe en formato SARIF
cargo run -- scan -f sarif -o report.sarif

# Exportar el informe en formato CSV
cargo run -- scan -f csv -o report.csv

# Exportar el informe en formato HTML
cargo run -- scan -f html -o report.html
```

El formato `cli` está orientado a proporcionar una visión general del análisis, mientras que los formatos de exportación permiten consultar y procesar los hallazgos individuales.

### Listar las reglas disponibles

Para mostrar el catálogo de reglas implementadas:

```bash
cargo run -- rules
```

---

## Estructura del proyecto

```text
.
├── Cargo.toml
├── Cargo.lock
├── docs/                 # Catálogo de reglas (Markdown y JSON)
└── src/
    ├── analyzers/         # Analizadores y reglas de infraestructura
    ├── cli.rs             # Definiciones de la interfaz de línea de comandos
    ├── commands/          # Implementación de comandos
    ├── core/              # Abstracciones principales de reglas y hallazgos
    ├── main.rs            # Punto de entrada de la aplicación
    ├── models.rs          # Modelos y estructuras de datos compartidos
    ├── reports/
    │   ├── formatters/    # Generadores de informes
    │   ├── output.rs      # Utilidades de presentación y salida
    │   └── scan.rs        # Coordinación del proceso de análisis
    └── utils/             # Funciones auxiliares
```

## Hoja de ruta

- [x] Descubrimiento recursivo de archivos de infraestructura.
- [x] Análisis estático de Dockerfile.
- [x] Implementación de reglas Dockerfile (`DF001` - `DF011`).
- [x] Procesamiento paralelo de archivos mediante Rayon.
- [x] Consolidación de resultados y métricas del análisis.
- [x] Clasificación de hallazgos por severidad.
- [x] Múltiples formatos de informe (`cli`, `json`, `markdown`, `html`, `csv`, `junit`, `sarif`).
- [x] Exportación de informes a archivos (`-o` / `--output`).
- [ ] Compatibilidad completa con Docker Compose.
- [ ] Compatibilidad con Kubernetes.
- [ ] Compatibilidad con Terraform.
- [ ] Compatibilidad con Ansible.
- [ ] Reglas configurables.
- [ ] Integración con pipelines de CI/CD.

## Licencia

Este proyecto se encuentra actualmente en desarrollo.
