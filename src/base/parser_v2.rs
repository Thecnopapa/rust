use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;


// Misc functions ######################################################################################################
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read(filepath:String){
    println!("Parsing file: {filepath}");
    if let Ok(lines) = read_lines(filepath) {
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }
}


// Real parser starts here #############################################################################################

#[derive(Debug)]
#[derive(PartialEq)]
enum Status{
    Loop,
    LoopHeader,
    Dict,
    Blank,
    Literal,
}

#[derive(Debug)]
struct Item {
    n: i32,
    values: Vec<String>
}

#[derive(Debug)]
struct Field{
    mode: String,
    key: String,
    subkeys: Vec<String>,
    subitems: Vec<Item>
}

#[derive(Debug)]
pub struct Data{
    dataname: String,
    fields: Vec<Field>
}

#[derive(Debug)]
enum Pending{
    NewField,
    NewKey,
    None
}

#[derive(Debug)]
enum Current{
    Field(Field),
    Item(Item),
    None
}


pub fn parse(filepath:String, include:Vec<String>, exclude:Vec<String>) -> Data {
    println!("Parsing file: {filepath}");
    println!("INCLUDE:{:?}", include); // Keys to include
    println!("EXCLUDE:{:?}", exclude); // Keys to exclude
    // Keys = [k for k in keys if (k in include or len(include) == 0) and k not in exclude]

    let re = Regex::new(r"  *").unwrap(); // Pattern to merge spaces

    let mut data = Data{ // This will be the function output
        dataname:"0UNK".to_string(), // Default PDB code
        fields: vec![] // Initialise empty data
    };

    let mut current_field = Current::None;
    let mut current_item = Current::None;
    let mut current_n = 0;
    let mut pending = Pending::None;

    let mut n = -1; // Line counter
    let mut status = Status::Blank; // Current parser status
    if let Ok(lines) = read_lines(filepath) {
        for line in lines.map_while(Result::ok) {
            n += 1;
            if n >33 {
                break
            }
            // Loop through every line in file
            println!("\n[{n}] {:?}", line);
            let l = re.replace_all(&line, " "); // l is line with compressed spaces (and trimmed)
            let l = l.trim();
            println!("l: {:?}", l);
            let line_comps: Vec<&str> = l.split(" ").collect(); // line_comps is vector splid by spaces
            println!("line_comps: {:?}", line_comps);
            let l0 = line_comps[0]; // l0 is first word
            println!("l0: {:?}", l0);

            let c0 = l0.chars().next(); // c0 is first char (Some()/None)




            // Detect end of block and empty lines
            match status{
                Status::Literal =>{} // Not applied if inside literal
                _ => {
                    match c0 {
                        Some(c) => {
                            match c {
                                // Block end
                                '#' => {
                                    println!("End of block!");
                                    match data.fields.last() {
                                        Some(last) => {
                                            println!("LAST FIELD: {:?}", &last);
                                            println!("LAST SUBKEYS: {:?}", &last.subkeys);
                                            println!("LAST N SUBITEMS: {:?}", &last.subitems.len());
                                            for val in &last.subitems {
                                                println!("{:?}: {:?}", val.n, val.values);
                                            }
                                        },
                                        None => {}
                                    }
                                    match current_field {
                                        Current::Field(mut f) => {
                                            match current_item {
                                                Current::Item(i) => {
                                                    f.subitems.push(i);
                                                },
                                                _ => {}
                                            }
                                            data.fields.push(f);
                                        },
                                        _ => {}
                                    }

                                    pending = Pending::None;
                                    status = Status::Blank;
                                    current_field = Current::None;
                                    current_item = Current::None;
                                    current_n = 0;
                                    continue
                                },
                                // Line with key
                                '_' => {
                                    println!("Key detected!");
                                    match status {
                                        Status::Blank => {
                                            // New field
                                            pending = Pending::NewField;
                                            status = Status::Dict;
                                        },
                                        Status::Dict => {
                                            // New key
                                            pending = Pending::NewKey;
                                        },
                                        Status::LoopHeader => {
                                            // New loop key
                                            pending = Pending::NewKey;
                                        },
                                        _ => {}
                                    }
                                },
                                _ => {
                                    match l0 {
                                        "loop_" => {
                                            // Start loop header
                                            println!("Loop detected!");
                                            status = Status::LoopHeader;
                                            continue
                                        },
                                        _ => {
                                            if l0.split("_").collect::<Vec<&str>>()[0] == "data" {
                                                // Data name found
                                                println!("Dataname found!");
                                                data.dataname = l0.split("_").collect::<Vec<&str>>()[1].to_string();
                                                continue
                                            }
                                        }
                                    }
                                    match status {
                                        Status::LoopHeader => {
                                            // Start loop
                                            status = Status::Loop;
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        },
                        // Blank line (outside literal)
                        None => {
                            println!("Empty line!");
                            continue
                        }
                    }
                }

            }
            println!("STATUS: {:?}", status);

            // If new key found, and not new field created(eg. first loopheader), then upgrade to newfield
            match pending {
                Pending::NewKey => {
                    match current_field {
                        Current::None => {
                            pending = Pending::NewField
                        },
                        _ => {}
                    }
                },
                _ => {}
            }

            // Create new field
            match pending {
                Pending::NewField => {
                    let k = l0.split(".").collect::<Vec<&str>>()[0].to_string();
                    println!("NEW field: {:?}", k);
                    match status {
                        Status::Dict => {current_field = Current::Field(Field{mode:String::from("dict"), key:k, subkeys:vec![], subitems:vec![]});},
                        Status::LoopHeader => {current_field = Current::Field(Field{mode:String::from("loop"), key:k, subkeys:vec![], subitems:vec![]});},
                        _ => {}
                    }

                },
                _ => {}
            }

            // Add new subkey to current field
            match pending {
                Pending::NewField | Pending::NewKey=> {
                    let subk = l0.split(".").collect::<Vec<&str>>()[1].to_string();
                    println!("NEW subkey: {:?}", subk);
                    match current_field {
                        Current::Field(ref mut f) => {
                            f.subkeys.push(subk);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
            println!("CURRENT FIELD: {:?}", current_field);
            pending = Pending::None; // Reset pending


            // Create subitems
            match status {
                // If dict add new empty subitem
                Status::Dict => {
                    match current_item {
                        Current::None => {
                            println!("New dict item");
                            current_item = Current::Item(Item{n:0, values:vec![]});
                        },
                        _ => {}
                    }
                },
                // If loop add new subitem with values
                Status::Loop => {
                    println!("New loop item");
                    let mut vals = vec![];
                    for lc in line_comps {
                        vals.push(lc.to_string());
                    }
                    match current_field {
                        Current::Field(ref mut f) => {
                            f.subitems.push(Item{n:current_n, values:vals});
                        },
                        _ => {}
                    }
                    current_n += 1;
                },
                _ => {}
            }



            // Update subitem
            match current_item {
                Current::Item(ref mut i) => {
                    println!("CURRENT ITEM: {i:?}");
                    match status {
                        // If dict update current item
                        Status::Dict => {
                            let mut val = String::from("");

                            let l1 = &line.split(" ").collect::<Vec<&str>>()[1..];
                            let l1 = l1.join(" ");
                            let l1 = l1.trim();
                            println!("LINE: {l1:?}");

                            for c in l1.chars() {
                                print!("{c:?}");
                                val.push(c);
                            }
                            println!("\nVAL: {val:?}");
                            i.values.push(val);

                        },
                        _ => {}
                    }
                    println!("CURRENT ITEM: {i:?}, {}", i.values.len());
                },
                _ => {}
            }
            // match data.fields.last() {
            //     Some(last) => {
            //         println!("LAST FIELD: {:?}", &last);
            //         println!("LAST SUBKEYS: {:?}", &last.subkeys);
            //         println!("LAST N SUBITEMS: {:?}", &last.subitems.len());
            //         for val in &last.subitems {
            //             println!("{:?}: {:?}", val.n, val.values);
            //         }
            //     },
            //     None => {}
            // }









        }
    }
    println!("\nDATA: {:?}", data);
    return data; // Return parsed data
}
// #####################################################################################################################