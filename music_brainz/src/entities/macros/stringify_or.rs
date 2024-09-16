macro_rules! stringify_or {
    ($value_ident:ident) => {
        stringify!($value_ident)
    };
    ($value_ident:ident, $value_expr: expr) => {
        $value_expr
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stringify_or() {
        assert_eq!(stringify_or!(value), "value");
        assert_eq!(stringify_or!(value, "test"), "test");
    }
}
