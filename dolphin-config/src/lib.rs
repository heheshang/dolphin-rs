use std::path::{Path, PathBuf};
pub mod api_config;
pub mod dao_config;


pub fn get_api_config_path() -> PathBuf {
    let value = dotenvy::var("CURRENT_DIR").expect("CURRENT_DIR must be set");

    Path::new(&value).join("api-config")
}


pub fn get_dao_config_path() -> PathBuf {
    let value = dotenvy::var("CURRENT_DIR").expect("CURRENT_DIR must be set");

    Path::new(&value).join("dao-config")
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_api_config_path() {
        let path = super::get_api_config_path();
        println!("path: {:?}", path);
    }
}
