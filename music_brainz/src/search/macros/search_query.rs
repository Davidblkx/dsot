macro_rules! search_query {
    ($name:ident [$target:expr] {$($prop:ident: $comment:expr),*}) => {
        paste::paste! {
            #[derive(Clone, Debug)]
            pub struct [< $name Query>] {
                pub value: String,
                pub limit: u32,
                pub offset: u32,
            }

            impl [< $name Query>] {
                pub fn for_query(value: String) -> Self {
                    Self {
                        value,
                        limit: 25,
                        offset: 0,
                    }
                }
            }

            impl crate::search::SearchQuery for [< $name Query>] {
                fn target(&self) -> &'static str {
                    $target
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
                    [< $name Query>]::for_query(self.parts.join(""))
                }

                // Escape special characters not allowed in lucene query
                fn escape_value(&self, value: &str) -> String {
                    value
                        .replace("+", "\\+")
                        .replace("-", "\\-")
                        .replace("&&", "\\&&")
                        .replace("||", "\\||")
                        .replace("!", "\\!")
                        .replace("(", "\\(")
                        .replace(")", "\\)")
                        .replace("{", "\\{")
                        .replace("}", "\\}")
                        .replace("[", "\\[")
                        .replace("]", "\\]")
                        .replace("^", "\\^")
                        .replace("\"", "\\\"")
                        .replace("~", "\\~")
                        .replace("*", "\\*")
                        .replace("?", "\\?")
                        .replace(":", "\\:")
                        .replace("\\", "\\\\")
                        .replace("/", "\\/")
                }

                $(
                    #[doc = $comment]
                    pub fn $prop(&mut self, $prop: &str) -> &mut Self {
                        let prop_value = self.escape_value($prop);
                        let value = format!("{}:{}", stringify!($prop), prop_value);
                        self.parts.push(value);
                        self
                    }
                )*
            }
        }
    };
}
