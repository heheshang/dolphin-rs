use std::path::{Path, PathBuf};


pub fn get_api_config_path() -> PathBuf {
    let value = dotenvy::var("CURRENT_DIR").expect("CURRENT_DIR must be set");

    Path::new(&value).join("api-config")
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_api_config_path() {
        let path = super::get_api_config_path();
        println!("path: {:?}", path);
    }
}
