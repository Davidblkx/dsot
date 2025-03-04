/// Trait for SQL entities, allows to define the table name, columns and values for a SQL entity.
pub trait SqlEntity {
    /// Returns the name of the table.
    fn table_name() -> &'static str;
    /// Returns the columns of the table.
    fn columns() -> Vec<&'static str>;
    /// Returns the values of the table.
    fn values(&self) -> Vec<String>;
}

/// Sanitizes sql parameters by wrapping the parameter in single quotes and escaping any single quotes in the parameter.
pub fn sanitize_param(param: &str) -> String {
    format!("'{}'", param.replace("'", "''"))
}
