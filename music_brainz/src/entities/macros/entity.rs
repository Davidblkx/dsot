macro_rules! entity {
    (
        $name:ident {
            inc: [$($inc:ident$(=$($inc_value:expr)?)?),*],
            search [$search_result:ident]: {$($prop_search:ident: $comment_search:expr),*},
            schema [$schema_name:ident]: {$(
                [$comment:expr]
                $prop:ident $($json_prop:literal)?: $prop_type:ty
            ),*}
        }
    ) => {
        paste::paste! {
            #[derive(Clone, Debug, serde::Deserialize)]
            pub struct $name {
                #[doc = "The MusicBrainz Identifier (MBID) of the entity."]
                pub id: String,

                $(
                    $(#[serde(alias = $json_prop)])?
                    #[doc = $comment]
                    pub $prop: $prop_type,
                )*
            }

            #[derive(Clone, Debug, serde::Deserialize)]
            pub struct [< $name QueryResult>] {
                pub created: String,
                pub count: u32,
                pub offset: u32,
                pub $search_result: Vec<$name>,
            }

            #[derive(Clone, Debug)]
            pub struct [< $name Query>] {
                pub value: String,
                pub limit: u32,
                pub offset: u32,
            }

            impl [< $name Query>] {
                pub fn for_query(value: &str) -> Self {
                    Self {
                        value: value.to_string(),
                        limit: 25,
                        offset: 0,
                    }
                }

                pub async fn execute(&self) -> crate::error::Result<[< $name QueryResult>]> {
                    let json_src: String = crate::operations::search::execute_search(self).await?;
                    let json: [< $name QueryResult>] = serde_json::from_str(&json_src)?;
                    Ok(json)
                }
            }

            impl crate::operations::search::SearchQuery for [< $name Query>] {
                fn target(&self) -> &'static str {
                    stringify!($schema_name)
                }

                fn query_value(&self) -> &str {
                    &self.value
                }

                fn limit(&self) -> u32 {
                    self.limit
                }

                fn offset(&self) -> u32 {
                    self.offset
                }
            }

            #[derive(Clone, Debug)]
            pub struct [< $name Query Builder>] {
                parts: Vec<String>,
            }

            impl [< $name Query Builder>] {
                pub fn new() -> Self {
                    Self {
                        parts: Vec::new(),
                    }
                }

                pub fn begin_group(&mut self) -> &mut Self {
                    self.parts.push("(".to_string());
                    self
                }

                pub fn end_group(&mut self) -> &mut Self {
                    self.parts.push(")".to_string());
                    self
                }

                pub fn and(&mut self) -> &mut Self {
                    self.parts.push(" AND ".to_string());
                    self
                }

                pub fn or(&mut self) -> &mut Self {
                    self.parts.push(" OR ".to_string());
                    self
                }

                pub fn not(&mut self) -> &mut Self {
                    self.parts.push(" NOT ".to_string());
                    self
                }

                pub fn build(&self) -> [< $name Query>] {
                    [< $name Query>]::for_query(&self.parts.join(""))
                }

                $(
                    #[doc = $comment]
                    pub fn $prop(&mut self, $prop: &str) -> &mut Self {
                        let prop_value = crate::utils::lucene::escape_value($prop);
                        let value = format!("{}:{}", stringify!($prop), prop_value);
                        self.parts.push(value);
                        self
                    }
                )*
            }
        }
    };
}
