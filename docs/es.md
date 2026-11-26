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
- Configuración opcional mediante TOML para rutas ignoradas y sobrescrituras de reglas.
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

## CI y releases

- **CI** (`.github/workflows/ci.yml`): se ejecuta en push/PR a `main` y `workflow_dispatch`. Verifica formato, ejecuta Clippy con `-D warnings`, ejecuta tests y compila un binario release.
- **Release** (`.github/workflows/release.yml`): se ejecuta en tags `v*`. Valida que el tag coincida con la versión de `infra_vet` en Cargo, luego compila y empaqueta binarios para Linux, Windows y macOS (x86_64/aarch64 donde aplica), genera checksums SHA256 y publica un GitHub Release.

### Configuración

InfraVet carga `infravet.toml` desde la raíz del directorio analizado. Usa `-C` / `--config` para cargar un archivo explícito. Si no se encuentra ningún archivo de configuración, se aplican los valores por defecto (sin rutas ignoradas ni sobrescrituras de reglas).

```toml
default-severity = "WARNING"

[ignore]
paths = ["legacy/", "tmp/"]

[rules]
"DF001" = "ERROR"                              # cambiar severidad
"DF002" = { enabled = false }                  # deshabilitar una regla
"DF003" = { enabled = true, severity = "INFO" }
```

- Las rutas en `ignore.paths` son relativas a la raíz del análisis; las rutas absolutas se rechazan.
- La forma corta (`"DF001" = "ERROR"`) cambia la severidad. La forma detallada también admite `enabled = false` para suprimir una regla por completo.
- `default-severity` es aceptado por el analizador de configuración, pero todavía no se aplica a los hallazgos; la severidad de cada regla proviene de su implementación.

El archivo `infravet.toml` en la raíz del repositorio es una configuración de ejemplo y se aplica a los análisis iniciados desde ese directorio.

---

## Estructura del proyecto

```text
.
├── .github/
│   └── workflows/         # Automatización de CI y publicación
├── Cargo.toml
├── Cargo.lock
├── infravet.toml          # Configuración de ejemplo
├── docs/                  # Catálogo de reglas (Markdown y JSON)
└── src/
    ├── analyzers/         # Analizadores y reglas de infraestructura
    ├── cli.rs             # Definiciones de la interfaz de línea de comandos
    ├── commands/          # Implementación de comandos
    ├── config/            # Carga, valores por defecto y validación de TOML
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
- [x] Reglas configurables (`infravet.toml`: rutas ignoradas, activar/desactivar reglas y sobrescrituras de severidad).
- [x] Integración de pipeline CI (GitHub Actions: fmt, clippy, tests, build).
- [x] Flujo de publicación por etiquetas (builds multiplataforma, checksums, GitHub Release).
- [ ] Aplicar `default-severity` a los hallazgos que no declaran una severidad.
- [ ] Compatibilidad completa con Docker Compose.
- [ ] Compatibilidad con Kubernetes.
- [ ] Compatibilidad con Terraform.
- [ ] Compatibilidad con Ansible.

## Licencia

Este proyecto se encuentra actualmente en desarrollo.
