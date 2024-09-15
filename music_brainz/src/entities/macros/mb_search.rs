macro_rules! mb_search {
    ($name:ident {
        name = $schema_name:ident,
        search_name = $search_result:ident,
        props = {$($prop_search:ident: $comment_search:expr),*}
    }) => {
        paste::paste! {
            #[derive(Clone, Debug, serde::Deserialize)]
            pub struct [< $name SearchResult>] {
                pub created: String,
                pub count: u32,
                pub offset: u32,
                pub $search_result: Vec<$name>,
            }

            #[derive(Clone, Debug)]
            pub struct [< $name Search>] {
                pub query: String,
                pub limit: u32,
                pub offset: u32,
            }

            impl crate::operations::search::SearchQuery for [< $name Search>] {
                fn target(&self) -> &'static str {
                    stringify!($schema_name)
                }

                fn query_value(&self) -> &str {
                    &self.query
                }

                fn limit(&self) -> u32 {
                    self.limit
                }

                fn offset(&self) -> u32 {
                    self.offset
                }
            }

            impl [< $name Search>] {
                pub fn for_query(value: &str) -> Self {
                    Self {
                        query: value.to_string(),
                        limit: 25,
                        offset: 0,
                    }
                }

                pub async fn execute(&self) -> crate::error::Result<[< $name SearchResult>]> {
                    let json_src: String = crate::operations::search::execute_search(self).await?;
                    let json: [< $name SearchResult>] = serde_json::from_str(&json_src)?;
                    Ok(json)
                }
            }

            #[derive(Clone, Debug)]
            pub struct [< $name Search Builder>] {
                parts: Vec<String>,
            }

            impl $name {
                pub fn search() -> [< $name Search Builder>] {
                    [< $name Search Builder>]::new()
                }
            }

            impl [< $name Search Builder>] {
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

                pub fn build(&self) -> [< $name Search>] {
                    [< $name Search>]::for_query(&self.parts.join(""))
                }

                $(
                    #[doc = $comment_search]
                    pub fn $prop_search(&mut self, $prop_search: &str) -> &mut Self {
                        let prop_value = crate::utils::lucene::escape_value($prop_search);
                        let value = format!("{}:{}", stringify!($prop_search), prop_value);
                        self.parts.push(value);
                        self
                    }
                )*
            }
        }
    };
}
