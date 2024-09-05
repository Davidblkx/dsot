/// Escape a value for use in a Lucene query.
pub fn escape_value(value: &str) -> String {
    value
        .replace("+", "\\+")
        .replace("-", "\\-")
        .replace("&&", "\\&&")
        .replace("||", "\\||")
        .replace("!", "\\!")
        .replace("(", "\\(")
        .replace(")", "\\)")
        .replace("{", "\\{")
        .replace("}", "\\}")
        .replace("[", "\\[")
        .replace("]", "\\]")
        .replace("^", "\\^")
        .replace("\"", "\\\"")
        .replace("~", "\\~")
        .replace("*", "\\*")
        .replace("?", "\\?")
        .replace(":", "\\:")
        .replace("\\", "\\\\")
        .replace("/", "\\/")
}
