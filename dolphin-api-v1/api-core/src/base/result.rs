use crate::core_error::app_status::{AppStatus, ErrorCode};
use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[warn(dead_code)]
#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResult<T> {
    pub data: Option<T>,
    #[serde(flatten)]
    pub errmsg: ErrorCode,
    // #[serde(flatten)]
    // #[serde_as(as = "ssss")]
    #[serde(skip)]
    pub status: AppStatus,
}

impl<T> ApiResult<T> {
    pub fn new(data: Option<T>) -> Self {
        Self {
            data,
            status: AppStatus::SUCCESS,
            errmsg: ErrorCode::default(),
        }
    }

    pub fn new_with_err_status(data: Option<T>, status: AppStatus) -> Self {
        Self {
            data,
            status,
            ..Default::default()
        }
    }
}


impl Default for AppStatus {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl<T> Default for ApiResult<T> {
    fn default() -> Self {
        Self {
            data: None,
            errmsg: ErrorCode::default(),
            status: AppStatus::SUCCESS,
        }
    }
}

impl<T> IntoResponse for ApiResult<T>
where T: Serialize
{
    fn into_response(self) -> axum::response::Response {
        let err_msg = self.status.clone().into();
        let mut body = Json(self);
        body.0.errmsg = err_msg;
        // body.0.status = self.status;
        body.into_response()
    }
}
