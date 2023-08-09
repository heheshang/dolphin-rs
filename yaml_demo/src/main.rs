// fn main() {
//     let f = include_str!("b2c_global_daily_batch_device_mobile_da6_ref.yaml");
//     let yaml = serde_yaml::from_str::<serde_yaml::Value>(f).unwrap();

//     serde_yaml::to_writer(std::io::stdout(), &yaml).unwrap();
// }
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
struct Y {
    adobe_config: AdobeConfig,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]

struct AdobeConfig {
    project_name: Option<String>,
    global_elements: Option<Vec<GlobalElement>>,
    suite_configs: Option<Vec<SuiteConfig>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
struct GlobalElement {
    // Define the properties you need here...
    lasttouchchannel: Option<String>,
    mobiledevicetype: Option<String>,
    evar11: Option<String>,
    mobilemanufacturer: Option<String>,
    mobiledevicename: Option<String>,
    evar73: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
struct SuiteConfig {
    suite_ids: Vec<SuiteID>,
    granularity: Vec<String>,
    params: Vec<Param>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
struct SuiteID {
    suite_id: String,
    country_cd: String,
    cust_type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
struct Param {
    metrics: Vec<String>,
    segments: Vec<String>,
    alias: String,
}
use std::{
    fs::File,
    io::Write,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read YAML file
    // let mut file = File::open(
    //     "/home/ssk/workspace/rust-work/dolphin-rs/yaml_demo/src/\
    //      b2c_global_daily_batch_device_mobile_da6_ref.yaml",
    // )?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // print!("contents: {:?}", contents);
    let contents = include_str!("b2c_global_daily_batch_device_mobile_da6_ref.yaml");
    // print!("contents: {:?}", contents);
    // Deserialize into Rust structure
    // let adobe_config: AdobeConfig = serde_yaml::from_str(&contents)?;

    // Modify structure as needed (Example: Change project_name)
    // adobe_config.project_name = "project_name".to_string();

    let yaml = serde_yaml::from_str::<Y>(contents).unwrap();
    print!("yaml: {:?}", yaml);
    // Serialize to YAML
    let yaml_string = serde_yaml::to_string(&yaml)?;
    print!("yaml_string: {:?}", yaml_string);

    // Write to file
    let mut file =
        File::create("/home/ssk/workspace/rust-work/dolphin-rs/yaml_demo/src/output.yaml")?;
    file.write_all(yaml_string.as_bytes())?;

    Ok(())
}
