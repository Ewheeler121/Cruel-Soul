use std::{fs::File, io::Read};
use serde_json::Value;

fn get_save_data() -> Result<std::fs::File, std::io::Error> {
    let appdata_path = match std::env::var("APPDATA") {
        Ok(path) => path,
        Err(_) => {
            panic!("Could not find APPDATA");
        }
    };

    let save_data_path = format!("{}\\Godot\\app_userdata\\Cruelty Squad\\savegame.save", appdata_path);
    File::open(save_data_path)
}

fn parse_save_data(mut save_file: File) -> Value {
    let mut save_data_string: String = String::new();
    save_file.read_to_string(&mut save_data_string).expect("Could not open file");

    match serde_json::from_str(save_data_string.as_str()) {
        Ok(data) => data,
        Err(_) => {
            panic!("Save File Corrupted!!!");
        }
    }
}

fn set_save_data() -> Result<std::fs::File, std::io::Error> {
   let appdata_path = match std::env::var("APPDATA") {
        Ok(path) => path,
        Err(_) => {
            panic!("Could not find APPDATA");
        }
    };

    let save_data_path = format!("{}\\Godot\\app_userdata\\Cruelty Squad\\savegame.save", appdata_path);
    File::create(save_data_path)
    
}

fn main() {
    let mut save_data_file: File = get_save_data().expect("Could not find save data");
    let mut save_data: Value = parse_save_data(save_data_file);

    if save_data["soul"] == true {
        println!("you already have a soul");
        return;
    }
    
   if let Some(field) = save_data.get_mut("soul") {
        if let Some(bool_value) = field.as_bool() {
            *field = Value::Bool(!bool_value);
        }
    }

    save_data_file = set_save_data().expect("Could not open save file to overwrite");
    serde_json::to_writer(save_data_file, &save_data).expect("could not write to save file: COULD MEAN THAT THE SAVE IS NOW CORRUPTED");
}
