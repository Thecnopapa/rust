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
    key: String,
    subkeys: Vec<String>,
    subitems: Vec<Item>
}

#[derive(Debug)]
pub struct Data{
    dataname: String,
    fields: Vec<Field>
}


pub fn parse(filepath:String, include:Vec<String>, exclude:Vec<String>) -> Data{
    println!("Parsing file: {filepath}");
    println!("INCLUDE:{:?}", include); // Keys to include
    println!("EXCLUDE:{:?}", exclude); // Keys to exclude
    // Keys = [k for k in keys if (k in include or len(include) == 0) and k not in exclude]

    let re = Regex::new(r"  *").unwrap(); // Pattern to merge spaces

    let data = Data{ // This will be the function output
        dataname:"0UNK".to_string(), // Default PDB code
        fields: vec![] // Initialise empty data
    };

    let mut n = -1; // Line counter
    let mut status = Status::Blank; // Current parser status
    if let Ok(lines) = read_lines(filepath) {
        for line in lines.map_while(Result::ok) {
            n += 1;
            if n >150 {
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
                                    // TODO: Reset block vars
                                    status = Status::Blank;
                                    continue
                                },
                                // Line with key
                                '_' => {
                                    println!("Key detected!");
                                },
                                _ => {}
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
















        }
    }
    println!("DATA: {:?}", data);
    return data; // Return parsed data
}
// #####################################################################################################################