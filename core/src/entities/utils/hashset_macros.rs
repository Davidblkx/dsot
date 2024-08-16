macro_rules! impl_hashset_str_field {
    ($name:ident, $field:ident) => {
        paste::paste! {
            impl $name {
                pub fn [<get_ $field>](&self) -> crate::error::Result<std::collections::HashSet<String>> {
                    if self.$field.is_empty() {
                        return Ok(std::collections::HashSet::new());
                    }

                    let owned = self.$field.clone();
                    let set = crate::entities::utils::HashSetString::from_binary(owned)?;
                    Ok(set.values)
                }

                pub fn [<set_ $field>](&mut self, value: std::collections::HashSet<String>) -> crate::error::Result<()> {
                    let set = crate::entities::utils::HashSetString::new(value);
                    self.$field = set.to_binary()?;
                    Ok(())
                }

                pub fn [<add_ $field>](&mut self, value: String) -> crate::error::Result<bool> {
                    let mut set = self.[<get_ $field>]()?;
                    let added = set.insert(value);
                    self.[<set_ $field>](set)?;
                    Ok(added)
                }
            }
        }
    };
}
