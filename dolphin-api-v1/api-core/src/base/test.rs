use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::core_error::app_status::{AppStatus, ErrorCode};

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

    use serde_json::json;
    #[test]
    fn name() {
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

serde_with::serde_conv!(
    statusAsMsg,
    AppStatus,
    |status: &AppStatus| [status],
    |value: AppStatus| -> Result<_, std::convert::Infallible> { Ok(value.into()) }
);
