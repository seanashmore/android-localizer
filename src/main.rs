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
    let mut is_plural = false;

    write_to_file(&output_file, "field_name, english_translation, new_translation\n");

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                is_translatable = true;
                should_write = false;

                let mut value = String::from("");

                match &name.local_name as &str {
                    "string" => {
                        for a in attributes {
                            match &a.name.local_name as &str {
                                "translatable" => {
                                    is_translatable = a.value.parse().unwrap();
                                },
                                "name" => {
                                    should_write = true;
                                    value = a.value;
                                },
                                _ => {}
                            }
                        }
        
                        if !is_translatable || !should_write {
                            continue;
                        }
        
                        //Write the newline beginning with the resource name
                        write_to_file(&output_file, "\n");
                        write_to_file(&output_file, &value);
                    }
                    "string-array" => {
                        println!("string-array not supported yet")
                    }
                    "plurals" => {
                        is_plural = true;

                        for a in attributes {
                            if a.name.local_name == "name" {
                                value = a.value;
                            }
                        }
                    }
                    "item" => {
                        if is_plural {
                            println!("Found plural item");
                            write_to_file(&output_file, "\nplurals::");
                            for a in attributes {
                                if a.name.local_name == "quantity" {
                                    should_write = true;
                                    write_to_file(&output_file, &a.name.local_name);
                                    write_to_file(&output_file, "::");
                                    write_to_file(&output_file, &a.value);
                                }
                            }

                        }
                    }
                    _ => println!("Unsupported token: {}", &name.local_name)
                }
            }
            Ok(XmlEvent::Characters(ref string)) => {

                if !is_translatable || !should_write {
                    continue;
                }

                println!("printing: {}", &string);

                write_to_file(&output_file, ",\"");
                write_to_file(&output_file, &string);
                write_to_file(&output_file, "\"");
            }
            Ok(XmlEvent::EndElement { name, .. }) => {

                match &name.local_name as &str {
                    //"string" => println!("End string"),
                    "plurals" => {
                        is_plural = false;
                        println!("End plurals")
                    },
                    "resources" => println!("End file"),
                    _ => {}
                }

        
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
