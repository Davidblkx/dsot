use serde::{Deserialize, Serialize};

use super::Request;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseError {
    pub domain: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub api_version: u8,
    pub client_type: super::ClientType,
    pub library: super::TargetLibrary,
    pub data: Result<T, ResponseError>
}

pub struct ResponseBuilder {
    pub api_version: u8,
}

impl ResponseBuilder {
    pub fn assert_support<T>(&self, req: &Request<T>) -> crate::error::Result<()> {
        if req.api_version != self.api_version {
            Err(crate::error::DsotError::UnsupportedClientVersion(req.api_version))
        } else {
            Ok(())
        }
    }

    pub fn with_data<TReq, TRes>(&self, req: &Request<TReq>, data: Result<TRes, ResponseError>) -> Response<TRes> {
        Response {
            api_version: self.api_version,
            client_type: req.client_type.clone(),
            library: req.library.clone(),
            data,
        }
    }
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        ResponseBuilder {
            api_version: 1,
        }
    }
}
