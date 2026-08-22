macro_rules! declare_repository_unit {
    ($name:ident {
        $(
            $($is_async:tt)? fn $fn_name:ident(&self$(, $fn_param:ident: $fn_param_type:ty)*) -> $ret:ty; $default:expr;
        )*
    }) => {
        ::paste::paste! {
            #[async_trait::async_trait]
            pub trait [<$name Repository>]: Send + Sync + ::std::fmt::Debug {
                $($($is_async)? fn $fn_name(&self$(, $fn_param: $fn_param_type)*) -> $ret;)*
            }

            #[derive(Debug)]
            pub struct [<Default $name Repo>];

            #[async_trait::async_trait]
            impl [<$name Repository>] for [<Default $name Repo>] {
                $($($is_async)? fn $fn_name(&self$(, [<_ $fn_param>]: $fn_param_type)*) -> $ret {
                    $default
                })*
            }
        }
    };
}
