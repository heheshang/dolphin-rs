use crate::{
    core_error::error::{DisplayErrorInfo, DolphinErrorInfo},
    core_status::app_status::AppStatus,
};
use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub type GrpcResponse<T> = Result<tonic::Response<T>, tonic::Status>;
pub type GrpcRequest<T> = tonic::Request<T>;

#[warn(dead_code)]
#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResult<T> {
    pub data: Option<T>,
    #[serde(skip)]
    pub errmsg: DolphinErrorInfo,
    #[serde(flatten)]
    pub display: DisplayErrorInfo,
    #[serde(skip)]
    pub status: AppStatus,
    #[serde(skip)]
    pub extra: Option<Vec<String>>,
    failed: bool,
    success: bool,
}

impl<T> ApiResult<T> {
    pub fn new(data: Option<T>) -> Self {
        Self {
            data,
            status: AppStatus::SUCCESS,
            errmsg: DolphinErrorInfo::default(),
            display: DolphinErrorInfo::default().into(),
            extra: None,
            failed: false,
            success: true,
        }
    }

    pub fn new_with_err_status(data: Option<T>, status: AppStatus) -> Self {
        Self {
            data,
            status,
            failed: true,
            success: false,
            ..Default::default()
        }
    }

    pub fn new_with_err_extra(
        data: Option<T>,
        status: AppStatus,
        extra: Option<Vec<String>>,
    ) -> Self {
        let error_info: DolphinErrorInfo = status.clone().into();

        let code = error_info.code;
        let cn_msg = match &extra {
            Some(extra) => format_args(&error_info.cn_msg, extra.clone()),
            None => error_info.cn_msg,
        };

        let en_msg = match &extra {
            Some(extra) => format_args(&error_info.en_msg, extra.clone()),
            None => error_info.en_msg,
        };
        let errmsg = DolphinErrorInfo {
            code,
            cn_msg,
            en_msg,
        };

        Self {
            data,
            status,
            failed: true,
            success: false,
            display: errmsg.clone().into(),
            extra,
            errmsg,
        }
    }
}

impl<T> Default for ApiResult<T> {
    fn default() -> Self {
        Self {
            data: None,
            errmsg: DolphinErrorInfo::default(),
            display: DolphinErrorInfo::default().into(),
            extra: None,
            failed: false,
            success: true,
            status: AppStatus::SUCCESS,
        }
    }
}

impl<T> IntoResponse for ApiResult<T>
where T: Serialize
{
    fn into_response(self) -> axum::response::Response {
        let body = Json(self);
        body.into_response()
    }
}

fn format_args(text: &str, args: Vec<String>) -> String {
    let mut new_text = text.to_string();
    let re = regex::Regex::new(r"\{(\d+)").unwrap();
    for cap in re.captures_iter(text) {
        let index = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        if args.len() <= index {
            continue;
        }
        let ss = new_text.replace(&format!("{}{}{}", '{', index, '}'), &args[index]);
        new_text = ss.clone();
    }
    new_text
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn regex_is_work() {
        let text = "copy process definition from {0} to {2} error : {1}";
        let mut new_text = text.to_string();
        let args = vec![String::from("aaa"), String::from("bb"), String::from("cc")];

        let re = regex::Regex::new(r"\{(\d+)").unwrap();
        // let mut result = String::new();

        for cap in re.captures_iter(text) {
            let index = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            println!("{}", index);
            if args.len() <= index {
                continue;
            }
            let ss = new_text.replace(&format!("{}{}{}", '{', index, '}'), &args[index]);
            new_text = ss.clone();
            // println!("{}", ss);
            // result.push_str(&args[index]);
        }

        println!("{}", new_text);
    }
}
