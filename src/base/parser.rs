
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;


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

#[derive(Debug)]
#[derive(PartialEq)]
enum Status{
    Loop,
    LoopHeader,
    Dict,
    Blank,
}
#[derive(Debug)]
#[derive(PartialEq)]
struct Dict {
    key: String,
    subkey: String,
    val: String
}

#[derive(Debug)]
#[derive(PartialEq)]
struct ListItem {
    key: String,
    subkey: String,
    n: i32,
    val: Vec<String>,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct List {
    key: String,
    val: Vec<ListItem>
}

#[derive(Debug)]
#[derive(PartialEq)]
enum ParsedKey {
    ListKey(List),
    DictKey(Dict),
    None
}


pub fn parse(filepath:String, include:Vec<String>, exclude:Vec<String>){
    println!("Parsing file: {filepath}");
    let mut status = Status::Blank;
    let mut in_literal = false;
    let re = Regex::new(r"  *").unwrap();
    let mut n = 0;

    let mut result: Vec<ParsedKey> = vec![];
    let mut current: ParsedKey = ParsedKey::None;

    if let Ok(lines) = read_lines(filepath) {
        for line in lines.map_while(Result::ok) {
            println!("\n{n} ({:?})", status);
            n+=1;
            if n > 100 {break}
            let line = re.replace_all(&line, " ");

            //println!("{:?}", line);
            let mut l: Vec<&str> = line.split(" ").collect();
            //l.pop();
            println!("{:?}", l);

            if l.len() == 0 {
                continue
            }
            println!(">>{}", l[0]);
            //println!("{}", l[0] == "loop_");
            match l[0] {
                "#" => {
                    status = Status::Blank;
                    current = ParsedKey::None;
                    in_literal = false;
                    println!("status --> {status:?}");
                },

                "loop_" => {
                    status = Status::LoopHeader;
                    println!("status --> {status:?}");
                    continue

                },
                _ => {
                    if status == Status::LoopHeader {
                        match l[0].chars().next() {
                            Some(c) => if c != '_'  {
                                status = Status::Loop;
                                },
                            None => {todo!()}
                        }
                    }
                },
            }

            println!("Current = {current:?}");
            match status {
                Status::Blank => {continue},
                Status::LoopHeader => {
                    println!("In loop header = {current:?}");
                    match current {
                        ParsedKey::None => {
                            let h: Vec<&str>= l[0].split(".").collect();
                            let k = h[0];
                            println!("Key = {k}");
                            current = ParsedKey::ListKey(List{key: k.to_string(), val: vec![]});
                            println!("Current = {current:?}");
                        },
                        _ => {}
                    }

                },
                Status::Loop => {

                },
                Status::Dict => {

                },
            }
        }
    }
    println!("RESULT:\n{:?}", result);
}