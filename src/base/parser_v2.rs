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
}


pub fn parse(filepath:String, include:Vec<String>, exclude:Vec<String>){
    println!("Parsing file: {filepath}");
    println!("INCLUDE:{:?}", include); // Keys to include
    println!("EXCLUDE:{:?}", exclude); // Keys to exclude
    // Keys = [k for k in keys if (k in include or len(include) == 0) and k not in exclude]

    let re = Regex::new(r"  *").unwrap(); // Pattern to merge spaces


    let mut dataname: String = "0UNK".to_string(); // Default PDB code
    let mut n = 0; // Line counter
    let mut status = Status::Blank; // Current parser status
    if let Ok(lines) = read_lines(filepath) {
        for line in lines.map_while(Result::ok) {
            // Loop through every line in file










        }
    }
}
// #####################################################################################################################