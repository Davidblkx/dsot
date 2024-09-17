macro_rules! mb_lookup {
    ($name:ident {
        target = $target:ident,
        inc = $($inc:ident$(=$inc_value:expr)?),*
    }) => {
        paste::paste! {
            #[derive(Clone, Debug)]
            pub struct [< $name Lookup >] {
                pub mbid: String,
                pub includes: Vec<String>,
            }

            impl $name {
                pub fn lookup(mbid: &str) -> [< $name Lookup >] {
                    [< $name Lookup >]::new(mbid)
                }
            }

            impl crate::operations::lookup::EntityLookup for [< $name Lookup >] {
                fn target(&self) -> &'static str {
                    stringify!($target)
                }

                fn mbid(&self) -> &str {
                    &self.mbid
                }

                fn includes(&self) -> &Vec<String> {
                    &self.includes
                }
            }

            impl [< $name Lookup >] {
                pub fn new(mbid: &str) -> Self {
                    Self {
                        mbid: mbid.to_string(),
                        includes: Vec::new(),
                    }
                }

                pub async fn execute(&self) -> crate::error::Result<$name> {
                    let json_src: String = crate::operations::lookup::execute_lookup(self).await?;
                    let json: $name = crate::utils::safe_parse_json::parse(json_src)?;
                    Ok(json)
                }

                $(
                    pub fn [<inc_ $inc>](&mut self) -> &mut Self {
                        self.includes.push(stringify_or!($inc $(, $inc_value)?).to_string());
                        self
                    }
                )*

                pub fn inc_tags(&mut self) -> &mut Self {
                    self.includes.push("tags".to_string());
                    self
                }

                pub fn inc_annotation(&mut self) -> &mut Self {
                    self.includes.push("annotation".to_string());
                    self
                }

                pub fn inc_genres(&mut self) -> &mut Self {
                    self.includes.push("genres".to_string());
                    self
                }
            }
        }
    };
}
