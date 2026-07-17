use iroh::protocol::{ProtocolHandler, RouterBuilder};

pub trait DsotProtocolHandler: ProtocolHandler {
    fn get_alpn(&self) -> &[u8];

    fn register_router(self, router: RouterBuilder) -> RouterBuilder;
}

#[macro_export]
macro_rules! dsot_protocol {
    ($target:ident, $alpn:expr) => {
        impl ::crate::network::DsotProtocolHandler for $ident {
            fn register_router(
                self,
                router: ::iroh::protocol::RouterBuilder,
            ) -> ::iroh::protocol::RouterBuilder {
                router.accept($alpn, self)
            }

            fn get_alpn(&self) -> &[u8] {
                $alpn
            }
        }
    };
}
