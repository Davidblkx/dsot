macro_rules! impl_repository_shell {
    ($target:ident {
        users: $users_type:ty,
        devices: $devices_type:ty,
        inbox: $inbox_type:ty,
    }) => {
        #[derive(Debug)]
        pub struct $target {
            users: $users_type,
            devices: $devices_type,
            inbox: $inbox_type,
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
            ) -> $crate::error::Result<bool> {
                self.devices.add_device(device).await
            }
            async fn remove_device(&self, id: iroh::EndpointId) -> $crate::error::Result<()> {
                self.devices.remove_device(id).await
            }
        }

        #[::async_trait::async_trait]
        impl $crate::repository::InboxRepository for $target {
            async fn load_inbox(
                &self,
                filter: &$crate::state::inbox::InboxFilter,
            ) -> $crate::error::Result<Vec<$crate::state::inbox::InboxItemValue>> {
                self.inbox.load_inbox(filter).await
            }
        }

        impl $crate::repository::Repository for $target {}
    };
}
