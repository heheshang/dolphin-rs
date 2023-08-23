use serde::{Deserialize, Serialize};
use serde_with::serde_as;


#[derive(Clone, Copy, Debug, PartialEq)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

serde_with::serde_conv!(
    RgbAsArray,
    Rgb,
    |rgb: &Rgb| [rgb.red, rgb.green, rgb.blue],
    |value: [u8; 3]| -> Result<_, std::convert::Infallible> {
        Ok(Rgb {
            red: value[0],
            green: value[1],
            blue: value[2],
        })
    }
);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorCode {
    pub code: i32,
    pub en_msg: String,
    pub cn_msg: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AppStatus {
    SUCCESS,
    InternalServerErrorArgs,
    RequestParamsNotValidError,
}
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Colors {
    #[serde_as(as = "RgbAsArray")]
    one_rgb: Rgb,
    #[serde_as(as = "Vec<RgbAsArray>")]
    rgbs_in_vec: Vec<Rgb>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ColorsWith {
    #[serde(with = "RgbAsArray")]
    rgb_with: Rgb,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        let pink = Rgb {
            red: 255,
            green: 0,
            blue: 255,
        };
        let data = ColorsWith { rgb_with: pink };
        let json = serde_json::json!({
            "rgb_with": [255, 0, 255]
        });

        assert_eq!(json, serde_json::to_value(&data).unwrap());
        assert_eq!(data, serde_json::from_value(json).unwrap());
    }
}


#[warn(dead_code)]
#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub data: Option<T>,
    #[serde(flatten)]
    pub errmsg: ErrorCode,
    #[serde(skip)]
    // #[serde_as(as = "ssss")]
    pub status: AppStatus,
}
impl From<AppStatus> for ErrorCode {
    fn from(status: AppStatus) -> Self {
        match status {
            AppStatus::SUCCESS => ErrorCode::new(0, "success".to_string(), "成功".to_string()),
            AppStatus::InternalServerErrorArgs => ErrorCode::new(
                10001,
                "internal server error args".to_string(),
                "内部服务器错误参数".to_string(),
            ),
            AppStatus::RequestParamsNotValidError => ErrorCode::new(
                10002,
                "request params not valid error".to_string(),
                "请求参数不合法".to_string(),
            ),
        }
    }
}
impl Default for AppStatus {
    fn default() -> Self {
        Self::SUCCESS
    }
}
impl ErrorCode {
    pub fn new(code: i32, en_msg: String, cn_msg: String) -> ErrorCode {
        ErrorCode {
            code,
            en_msg,
            cn_msg,
        }
    }
}
impl Default for ErrorCode {
    fn default() -> Self {
        Self {
            code: 0,
            en_msg: "success".to_string(),
            cn_msg: "成功".to_string(),
        }
    }
}
