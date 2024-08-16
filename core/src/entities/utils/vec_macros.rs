macro_rules! impl_vec_u64_field {
    ($name:ident, $field:ident) => {
        paste::paste! {
            impl $name {
                pub fn [<get_ $field>](&self) -> crate::error::Result<Vec<u64>> {
                    if self.$field.is_empty() {
                        return Ok(Vec::new());
                    }

                    let owned = self.$field.clone();
                    let set = crate::entities::utils::VecU64::from_binary(owned)?;
                    Ok(set.values)
                }

                pub fn [<set_ $field>](&mut self, value: Vec<u64>) -> crate::error::Result<()> {
                    let set = crate::entities::utils::VecU64::new(value);
                    self.$field = set.to_binary()?;
                    Ok(())
                }

                pub fn [<add_ $field>](&mut self, value: u64) -> crate::error::Result<()> {
                    let mut set = self.[<get_ $field>]()?;
                    set.push(value);
                    self.[<set_ $field>](set)
                }
            }
        }
    };
}
