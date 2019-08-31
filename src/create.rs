use std::{fs, str};

use crate::path_exists;

pub fn create_persistent_directories(path: &str, key_dir: &str, number_of_agents: &usize, dna_id: &str){
    let current_keys: Vec<_> = fs::read_dir(key_dir).unwrap().map(|res| res.unwrap().path()).collect();
    let persistant_path = format!("{}/{}/storage", path, dna_id);

    let persistent_directories: Vec<_> = fs::read_dir(&persistant_path).unwrap().map(|res| res.unwrap().path()).collect();
    println!("Current number of persistent directories for agents: {}\n", persistent_directories.len());

    if persistent_directories.len() < *number_of_agents{
        for current_key in current_keys{
            let pub_address_split = current_key.to_str().unwrap().split("/").collect::<Vec<&str>>();
            let pub_address = pub_address_split[pub_address_split.len()-1];
            if persistent_directories.contains(&current_key) == false{
                println!("Creating directory at: {} for agent with pub address {}", persistant_path, pub_address);
                fs::create_dir(format!("{}/{}", persistant_path, pub_address)).unwrap();
            };
        };
    };
}

pub fn create_base_persistent_directory(path: &str, dna_id: &str) -> Result<(), ()>{
    let base_path = format!("{}/{}/", path, dna_id);
    let storage_path = format!("{}storage", base_path);

    if path_exists(&base_path) == false{
        fs::create_dir(&base_path).unwrap();
        fs::create_dir(&storage_path).unwrap();
        return Ok(())
    };

    if path_exists(&storage_path) == false{
        fs::create_dir(&storage_path).unwrap();
    };
    Ok(())
}

pub fn create_core_paths(path: &str) -> Result<(), ()> {
    if path_exists(path) == false{
        fs::create_dir(path).unwrap();
    };

    let persistence_dir = format!("{}/persistence", path);
    let n3h_dir = format!("{}/n3h", path);

    if path_exists(&persistence_dir) == false {
        fs::create_dir(persistence_dir).unwrap();
    };

    if path_exists(&n3h_dir) == false {
        fs::create_dir(n3h_dir).unwrap();
    };

    Ok(())
}