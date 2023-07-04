mod dialog_box;

use dialog_box::display;
use std::{fs::File, io::Read};
use serde_json::Value;

fn get_save_path() -> String {
let appdata_path = match std::env::var("APPDATA") {
        Ok(path) => path,
        Err(_) => {
            display::error("Pathing Error", "Could not find APPDATA");
            panic!();
        }
    };

format!("{}\\Godot\\app_userdata\\Cruelty Squad\\savegame.save", appdata_path)    
}

fn parse_save_data(mut save_file: File) -> Value {
    let mut save_data_string: String = String::new();
    match save_file.read_to_string(&mut save_data_string) {
        Ok(_) => (),
        Err(_) => { 
            display::error("Parser Error", "Could not Read File");
            panic!();
        }
    }
    
    match serde_json::from_str(save_data_string.as_str()) {
        Ok(data) => data,
        Err(_) => {
            display::error("Paser Error", "Save File Corrupted");
            panic!();
        }
    }
}

fn main() {
    let mut save_data_file: File = match File::open(get_save_path()) { 
        Ok(file) => file,
        Err(_) => {
            display::error("File Error", "Could not open Save File to Read");
            panic!();
        }
    };
    let mut save_data: Value = parse_save_data(save_data_file);

    if save_data["soul"] == true {
        display::information("wtf?", "You already have a Soul");
        return;
    }
    
   if let Some(field) = save_data.get_mut("soul") {
        if let Some(bool_value) = field.as_bool() {
            *field = Value::Bool(!bool_value);
        }
    }

    save_data_file = match File::create(get_save_path()) {
        Ok(file) => file,
        Err(_) => {
            display::error("File Error", "Could not open save file to overwrite");
            panic!()
        }
    };

    match serde_json::to_writer(save_data_file, &save_data) {
        Ok(_) => (),
        Err(_) => {
            display::error("File Error", "could not write to save file: THIS COULD MEAN THAT THE SAVE IS WAS DELETED");
            panic!();
        }
    }

    display::information("back to the sluge", "Soul Restored");
}
