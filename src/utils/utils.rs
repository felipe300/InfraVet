pub fn uses_latest_tag(from_str: &str) -> bool {
    let mut parts = from_str.split_whitespace().skip(1);

    let image_token = match parts.find(|part| !part.starts_with("--")) {
        Some(token) => token,
        None => return false,
    };

    if image_token.contains("@") {
        return false;
    };

    if let Some((_, tag)) = image_token.rsplit_once(":") {
        tag == "latest"
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uses_latest_tag() {
        assert!(uses_latest_tag("FROM rust:latest"));
        assert!(uses_latest_tag("FROM ubuntu:latest"));
        assert!(uses_latest_tag("FROM golang:latest"));
        assert!(uses_latest_tag("FROM node:latest"));
    }

    #[test]
    fn test_does_not_use_latest_tag() {
        assert!(!uses_latest_tag("FROM ubuntu:slim"));
        assert!(!uses_latest_tag("FROM ubuntu:alpine"));
        assert!(!uses_latest_tag("FROM python:3.12-alpine"));
        assert!(!uses_latest_tag("FROM postgres:15-bullseye"));
    }

    #[test]
    fn test_latest_as_part_of_image_name() {
        assert!(!uses_latest_tag("FROM mylatest/image:1.0"));
    }

    #[test]
    fn test_edge_cases_and_malformed_inputs() {
        assert!(!uses_latest_tag(""));
        assert!(!uses_latest_tag("FROM"));
        assert!(!uses_latest_tag("   FROM    "));
    }

    #[test]
    fn test_sha256_digests_is_not_latest() {
        assert!(!uses_latest_tag(
            "FROM ubuntu@sha256:45b23ed16cd59d4019e4623"
        ));

        assert!(!uses_latest_tag(
            "FROM python@sha256:45b23ed16cd59d4019e4623"
        ));
    }

    #[test]
    fn test_from_with_flags() {
        assert!(uses_latest_tag("FROM --platform=linux/amd64 ubuntu:latest"));
        assert!(!uses_latest_tag("FROM --platform=linux/amd64 ubuntu:22.04"));
    }

    #[test]
    fn test_implicit_latest_tag() {
        assert!(uses_latest_tag("FROM ubuntu"));
    }

    #[test]
    fn test_multi_stage_builds() {
        assert!(uses_latest_tag("FROM golang:latest AS builder"));
        assert!(!uses_latest_tag("FROM golang:1.21 AS builder"));
    }
}
