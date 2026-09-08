pub fn uses_latest_tag(from_str: &str) -> bool {
    from_str
        .split_whitespace()
        .skip(1)
        .any(|part| part.ends_with("latest"))
}
