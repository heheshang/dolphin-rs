use axum::{
    response::{IntoResponse, Response},
    Json,
};

use crate::base::result::ApiResult;

use super::app_status::AppStatus;

pub struct AppError(pub AppStatus);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self.0 {
            AppStatus::SUCCESS => Json(ApiResult::new(Some(()))).into_response(),
            AppStatus::InternalServerErrorArgs => Json(ApiResult::new_with_err_status(
                Some(()),
                AppStatus::InternalServerErrorArgs,
            ))
            .into_response(),
            AppStatus::RequestParamsNotValidError  => todo!(),
        }
    }
}

impl<E> From<E> for AppError
where E: Into<AppStatus>
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
