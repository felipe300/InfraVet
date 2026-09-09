# Rules

This document contains the rules used by `infra_vet` to analyze Dockerfiles.

## Severity levels

| Severity | Description                                       |
| -------- | ------------------------------------------------- |
| ERROR    | A critical issue that should be fixed.            |
| WARNING  | A potential problem or recommended best practice. |
| INFO     | An optional recommendation or improvement.        |

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
RUN apk add --no-舆 python3
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
