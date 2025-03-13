#[macro_export]
macro_rules! stringify_or {
    ($value_ident:ident) => {
        stringify!($value_ident)
    };
    ($value_ident:ident, $value_expr: expr) => {
        $value_expr
    };
}

#[macro_export]
macro_rules! use_last_of {
    ($value1:ident) => {
        $value1
    };
    ($value1:ident, $value2: expr) => {
        $value2
    };
    ($value1:ident, $value2: expr, $value3: expr) => {
        $value3
    };
}
