macro_rules! impl_network_device_extension {
    ($id:ident, $access:ident) => {
        pub struct $id<'a> {
            device: &'a $crate::network::NetworkDevice,
            validator: $crate::network::protocols::server::TokenValidator,
        }

        impl $crate::network::devices::NetworkDevice {
            pub fn $access(
                &self,
                validator: impl $crate::network::protocols::server::NetworkValidator,
            ) -> $id<'_> {
                $id {
                    device: self,
                    validator: validator.get_validator(),
                }
            }
        }

        impl<'a> $id<'a> {
            async fn connect(
                &self,
            ) -> $crate::error::Result<$crate::network::sink::NetworkChannel> {
                let connection = self.device.connect_alpn(ALPN).await?;
                self.validator.start_handshake(connection).await
            }
        }
    };
}

macro_rules! exec_request {
    ($target:ident, $request:expr) => {{
        let mut channel = $target.connect().await?;

        let result = channel.request($request).await;

        channel.force_close().await;

        result
    }};
}
