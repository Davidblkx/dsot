#[macro_export]
macro_rules! macro_util_stringify_or {
    ($value_ident:ident) => {
        stringify!($value_ident)
    };
    ($value_ident:ident, $value_expr: expr) => {
        $value_expr
    };
}

#[macro_export]
macro_rules! macro_util_value_or {
    ($v1:tt) => {
        $v1
    };
    ($v1: tt, $v2: tt) => {
        $v2
    };
}

#[macro_export]
macro_rules! mu_stringify_last {
    ($($value:tt)*, $last:tt) => {
        stringify!($last)
    };
    ($last:tt) => {
        stringify!($last)
    };
}
