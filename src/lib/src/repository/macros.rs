macro_rules! impl_repository_shell {
    ($target:ident {
        users: $users_type:ty,
        devices: $devices_type:ty,
    }) => {
        #[derive(Debug)]
        pub struct $target {
            users: $users_type,
            devices: $devices_type,
        }

        #[::async_trait::async_trait]
        impl $crate::repository::UserRepository for $target {
            async fn list_users(&self) -> $crate::error::Result<Vec<String>> {
                self.users.list_users().await
            }
        }

        #[::async_trait::async_trait]
        impl $crate::repository::DeviceRepository for $target {
            async fn list_devices(
                &self,
            ) -> $crate::error::Result<Vec<$crate::state::devices::RemoteDevice>> {
                self.devices.list_devices().await
            }

            async fn add_device(
                &self,
                device: $crate::state::devices::RemoteDevice,
            ) -> $crate::error::Result<()> {
                self.devices.add_device(device).await
            }
            async fn remove_device(&self, id: iroh::EndpointId) -> $crate::error::Result<()> {
                self.devices.remove_device(id).await
            }
        }

        impl $crate::repository::Repository for $target {}
    };
}
