// Kind of an hack to have hardcoded names here, but we expect the declare_repository! macro to only be used once with this values
macro_rules! impl_repository {
    ($target:ident {
        device: $device:ty,
        inbox: $inbox:ty,
        user: $user:ty,
    }) => {
        #[derive(Debug)]
        pub struct $target {
            pub device: $device,
            pub inbox: $inbox,
            pub user: $user,
        }

        impl $crate::repository::Repository for $target {
            fn device(&self) -> &dyn $crate::repository::DeviceRepository {
                &self.device
            }

            fn inbox(&self) -> &dyn $crate::repository::InboxRepository {
                &self.inbox
            }

            fn user(&self) -> &dyn $crate::repository::UserRepository {
                &self.user
            }
        }
    };
}
