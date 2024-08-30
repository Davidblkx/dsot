use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request<T> {
    pub api_version: u8,
    pub client_type: super::ClientType,
    pub library: super::TargetLibrary,
    pub data: Option<T>,
}

#[derive(Debug)]
pub struct RequestBuilder {
    pub api_version: u8,
    pub client_type: super::ClientType,
    pub library: super::TargetLibrary,
}

impl RequestBuilder {
    pub fn for_client_type(mut self, client_type: super::ClientType) -> Self {
        self.client_type = client_type;
        self
    }

    pub fn for_library(mut self, library: &str) -> Self {
        self.library = super::TargetLibrary::Custom(library.to_string());
        self
    }

    pub fn for_default_library(mut self) -> Self {
        self.library = super::TargetLibrary::Default;
        self
    }

    pub fn with_data<T>(&self, data: T) -> Request<T> {
        Request {
            api_version: self.api_version,
            client_type: self.client_type.clone(),
            library: self.library.clone(),
            data: Some(data),
        }
    }

    pub fn empty<T>(&self) -> Request<T> {
        Request {
            api_version: self.api_version,
            client_type: self.client_type.clone(),
            library: self.library.clone(),
            data: None,
        }
    }
}

impl Default for RequestBuilder {
    fn default() -> Self {
        RequestBuilder {
            api_version: 1,
            client_type: super::ClientType::Embedded,
            library: super::TargetLibrary::Default,
        }
    }
}
