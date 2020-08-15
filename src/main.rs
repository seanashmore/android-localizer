extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use xml::reader::{EventReader, XmlEvent};

fn write_to_file(mut output_file: &File, value: &str) {
    output_file.write(value.as_bytes()).expect("Could not write to file");
}

fn main() {
    let file = File::open("strings-ru.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    let output_path = Path::new("output.csv");
    let display = output_path.display();

    let output_file = match File::create(&output_path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut is_translatable = true;
    let mut should_write = false;

    write_to_file(&output_file, "field_name, english_translation, new_translation\n");

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                is_translatable = true;
                should_write = false;

                let mut value = String::from("");

                let token_type = &name.local_name[..];

                match token_type {
                    "string" => {
                        for a in attributes {
                            if a.name.local_name == "translatable" {
                                is_translatable = false;
                            }
                            if a.name.local_name == "name" {
                                should_write = true;
                                value = a.value;
                            }
                        }
        
                        if !is_translatable || !should_write {
                            continue;
                        }
        
                        //Write the newline beginning with the resource name
                        write_to_file(&output_file, "\n");
                        write_to_file(&output_file, &value);
                    }
                    _ => {}
                }
            }
            Ok(XmlEvent::Characters(ref string)) => {

                if !is_translatable || !should_write {
                    continue;
                }

                write_to_file(&output_file, ",\"");
                write_to_file(&output_file, &string);
                write_to_file(&output_file, "\"");
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
