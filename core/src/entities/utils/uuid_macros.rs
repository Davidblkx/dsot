macro_rules! impl_uuid_field {
    ($name:ident, $field:ident) => {
        paste::paste! {
            impl $name {
                pub fn [<get_ $field _uuid>](&self) -> crate::error::Result<uuid::Uuid> {
                    crate::entities::utils::parse_uuid(&self.$field)
                }

                pub fn [<set_ $field _uuid>](&mut self, uuid: &uuid::Uuid) {
                    self.$field = crate::entities::utils::uuid_to_vec(uuid);
                }
            }
        }
    };
}

macro_rules! impl_opt_uuid_field {
    ($name:ident, $field:ident) => {
        paste::paste! {
            impl $name {
                pub fn [<get_ $field _uuid>](&self) -> crate::error::Result<Option<uuid::Uuid>> {
                    if let Some(uuid) = &self.$field {
                        let uuid = crate::entities::utils::parse_uuid(uuid)?;
                        Ok(Some(uuid))
                    } else {
                        Ok(None)
                    }
                }

                pub fn [<set_ $field _uuid>](&mut self, uuid: Option<uuid::Uuid>) {
                    self.$field = uuid.map(|u| crate::entities::utils::uuid_to_vec(&u));
                }
            }
        }
    };
}
