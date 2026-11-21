# Rules

This document contains the rules used by `infra_vet` to analyze Dockerfiles.

## Severity levels

| Severity | Description                                       |
| -------- | ------------------------------------------------- |
| ERROR    | A critical issue that should be fixed.            |
| WARNING  | A potential problem or recommended best practice. |
| INFO     | An optional recommendation or improvement.        |

The severities below are the built-in defaults. They can be changed per scan, or a rule can be disabled entirely, with `[rules]` overrides in `infravet.toml` (see the Configuration section in [`en.md`](./en.md) or [`es.md`](./es.md)).

---

# Dockerfile

## DF001 — Missing FROM instruction

**Severity:** ERROR

A Dockerfile must contain at least one `FROM` instruction to define a build stage.

### Example

```dockerfile
RUN echo "Hello"
```

---

## DF002 — Copying entire build context

**Severity:** WARNING

`COPY . .` copies all files from the build context into the image. Consider copying only specific required files or directories to improve layer caching and prevent leaking sensitive local files.

### Example

```dockerfile
COPY . .
```

---

## DF003 — Base image uses latest tag

**Severity:** INFO

Base image uses the `'latest'` tag or lacks an explicit version tag. Consider using a specific version tag or a slim variant for reproducible builds.

### Example

```dockerfile
FROM node:latest
```

---

## DF004 — Missing USER instruction

**Severity:** WARNING

Missing `USER` instruction. Running containers as the `root` user poses security risks. It is recommended to specify a non-root user.

### Example

```dockerfile
FROM alpine:3.18
RUN apk add --no-cache python3
```

---

## DF005 — Unlinked apt-get update

**Severity:** WARNING

`apt-get update` is used without `apt-get install` in the same `RUN` instruction. This can cause cache staleness issues when installing packages in later layers.

### Example

```dockerfile
RUN apt-get update
```

---

## DF006 — Empty Dockerfile

**Severity:** ERROR

The Dockerfile contains no content or only whitespace.

### Example

```dockerfile

```

---

## DF007 — Invalid Dockerfile syntax

**Severity:** ERROR

The Dockerfile contains syntax errors that prevent it from being parsed correctly.

### Example

```dockerfile
INVALID_INSTRUCTION something
```

---

## DF008 — Use COPY instead of ADD

**Severity:** WARNING

`ADD` is used for a local file or directory. `COPY` is more explicit and predictable; reserve `ADD` for archive extraction or remote URLs.

### Example

```dockerfile
FROM alpine:3.18
ADD app.js /app/
```

---

## DF009 — Possible sensitive data in ENV or ARG

**Severity:** ERROR

An `ENV` or `ARG` instruction contains a keyword associated with credentials (for example `PASSWORD`, `TOKEN` or `API_KEY`). Such values remain visible in the image history. Use secret mounts or runtime environment variables instead.

### Example

```dockerfile
FROM alpine:3.18
ENV DB_PASSWORD=secret123
```

---

## DF010 — Uncleaned package manager caches

**Severity:** WARNING

Packages are installed without cleaning the package manager cache, which increases the resulting image size. For apt, remove `/var/lib/apt/lists/*`; for apk, use `--no-cache` or remove the cache directory.

### Example

```dockerfile
FROM ubuntu:22.04
RUN apt-get update && apt-get install -y curl
```

---

## DF011 — Missing HEALTHCHECK instruction

**Severity:** INFO

No `HEALTHCHECK` instruction was found, so Docker cannot automatically monitor the container's health. Add one when the application exposes a meaningful health check.

### Example

```dockerfile
FROM nginx:alpine
```
