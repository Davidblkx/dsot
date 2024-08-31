macro_rules! query_prop_str {
    ($name:ident, $prop:ident) => {
        paste::paste! {
            impl $name {
                pub fn [< $prop >](&mut self, $prop: &str) -> &mut Self {
                    let value = format!("{}:{}", stringify!($prop), $prop);
                    self.parts.push(value);
                    self
                }
            }
        }
    };
}
