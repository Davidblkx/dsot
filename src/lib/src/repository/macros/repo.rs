macro_rules! async_expr {
    ($x:expr,) => {
        $x
    };
    ($x:expr, $is_async:tt) => {
        $x.await
    };
}

macro_rules! declare_repository {
    ($(
        $name:ident {
            $(
                $($is_async:tt)? fn $fn_name:ident(&self$(, $fn_param:ident: $fn_param_type:ty)*) -> $ret:ty; $default:expr;
            )*
        }
    ),*) => {
        use ::std::sync::Arc;
        use ::tokio::sync::{RwLock, RwLockReadGuard};

        ::paste::paste! {
            $(
                #[async_trait::async_trait]
                pub trait [<$name Repository>]: Send + Sync + ::std::fmt::Debug {
                    $($($is_async)? fn $fn_name(&self$(, $fn_param: $fn_param_type)*) -> $ret;)*
                }
            )*

            pub trait Repository: Sync + Send + ::std::fmt::Debug {
                $(
                    fn [< $name:lower >](&self) -> &dyn [<$name Repository>];
                )*
            }

            $(
                #[derive(Debug, Default, Clone, Copy)]
                pub struct [<Default $name Repo>];

                #[async_trait::async_trait]
                impl [<$name Repository>] for [<Default $name Repo>] {
                    $($($is_async)? fn $fn_name(&self$(, [<_ $fn_param>]: $fn_param_type)*) -> $ret {
                        $default
                    })*
                }
            )*

            /// Repository with no connection used as a placeholder
            #[derive(Debug, Default, Clone, Copy)]
            pub struct DefaultRepository {
                $(
                    pub [< $name:lower >]: [<Default $name Repo>],
                )*
            }

            impl Repository for DefaultRepository {
                $(
                    fn [< $name:lower >](&self) -> &dyn [<$name Repository>] {
                        &self.[< $name:lower >]
                    }
                )*
            }

            #[derive(Debug, Clone)]
            pub struct DsotRepository {
                repo: Arc<RwLock<Box<dyn Repository>>>,
            }

            impl DsotRepository {
                pub fn new(repo: impl Repository + 'static) -> Self {
                    Self {
                        repo: Arc::new(RwLock::new(Box::new(repo))),
                    }
                }

                pub async fn set_repo(&self, repo: impl Repository + 'static) {
                    let repo = Box::new(repo);
                    let mut writer = self.repo.write().await;
                    *writer = repo;
                }

                pub async fn read(&self) -> RwLockReadGuard<'_, Box<dyn Repository>> {
                    self.repo.read().await
                }
            }

            $(
                #[async_trait::async_trait]
                impl [<$name Repository>] for DsotRepository {
                    $(async fn $fn_name(&self$(, $fn_param: $fn_param_type)*) -> $ret {
                        async_expr!(self.repo.read().await.[<$name:lower>]().$fn_name($($fn_param),*), $($is_async)?)
                    })*
                }
            )*
        }
    };
}
