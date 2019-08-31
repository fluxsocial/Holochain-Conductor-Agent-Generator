use std::{fs, str};

use crate::{get_current_config, AGENT_STRING, INSTANCE_STRING, INTERFACE_STRING};

pub fn write_agent(path: &str, key_dir: &str, dna_ids: &Vec<&str>) -> Result<(), &'static str>{
    let pub_address_split = key_dir.split("/").collect::<Vec<&str>>();
    let pub_address = pub_address_split[pub_address_split.len()-1];

    let agent_string = format!(AGENT_STRING!(), pub_address, pub_address, key_dir, pub_address);
    let current_config = get_current_config();
    let new_config = format!("{}\n{}\n", current_config, agent_string);
    fs::write("./config.toml", new_config).expect("Unable to write file");

    for dna_id in dna_ids {
        let instance_string = format!(INSTANCE_STRING!(), pub_address, dna_id, dna_id, pub_address, path, dna_id, pub_address);
        let current_config = get_current_config();
        let new_config = format!("{}\n{}\n", current_config, instance_string);
        fs::write("./config.toml", new_config).expect("Unable to write file");
    };
    
    Ok(())
}

pub fn write_interface(key_dir: &str, dna_ids: &Vec<&str>) -> Result<(), &'static str> {
    let pub_address_split = key_dir.split("/").collect::<Vec<&str>>();
    let pub_address = pub_address_split[pub_address_split.len()-1];

    for dna_id in dna_ids{
        let interface_string = format!(INTERFACE_STRING!(), dna_id, pub_address);
        let current_config = get_current_config();
        let new_config = format!("{}\n{}", current_config, interface_string);
        fs::write("./config.toml", new_config).expect("Unable to write file");
    };
    Ok(())
}
