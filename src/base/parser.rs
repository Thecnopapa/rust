
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
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
struct DictItem {
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
struct Dict {
    key: String,
    val: Vec<DictItem>
}

#[derive(Debug)]
#[derive(PartialEq)]
enum ParsedKey {
    ListKey(List),
    DictKey(Dict),
    None
}

#[derive(Debug)]
enum Literal {
    Delimiter(String, char),
    Implicit,
    None
}


pub fn parse(filepath:String, include:Vec<String>, exclude:Vec<String>){
    println!("Parsing file: {filepath}");
    println!("INCLUDE:{:?}", include);
    println!("EXCLUDE:{:?}", exclude);
    let mut status = Status::Blank;
    let mut literal = Literal::None;
    let re = Regex::new(r"  *").unwrap();
    let mut n = 0;

    let mut result: Vec<ParsedKey> = vec![];
    let mut current: ParsedKey = ParsedKey::None;

    let mut dataname: String = "1234".to_string();

    if let Ok(lines) = read_lines(filepath) {
        for line in lines.map_while(Result::ok) {
            println!("\n{n} ({:?})", status);
            n+=1;
            if n > 150 {break}
            let l0: Vec<&str> = line.split(" ").collect();
            let l0 = l0[0];

            println!("LITERAL: {literal:?}");
            println!("LITERAL: {literal:?}");
            match literal {
                Literal::Delimiter(_, _) => {
                    println!("Inside literal: {line}");
                }, 
                Literal::Implicit => {
                    println!("Inside implicit literal: {line}");
                }, 
                _ => {
                    let line = re.replace_all(&line, " ");

                    //println!("{:?}", line);
                    let l: Vec<&str> = line.split(" ").collect();
                    //l.pop();
                    println!("{:?}", l);

                    if l.len() == 0 {
                        continue
                    }
                    
                    if l[0].starts_with("data_") {
                        let ll0: Vec<&str> = l.clone()[0].split("_").collect();
                        dataname = ll0[1].to_string();
                        println!("DATANAME:{}", dataname);
                        continue
                    }


                    //println!(">>{}", l[0]);
                    //println!("{}", l[0] == "loop_");
                    match l[0] {
                        "#" => {
                            status = Status::Blank;
                            match current {
                                ParsedKey::DictKey(_) | ParsedKey::ListKey(_) => {
                                    result.push(current);
                                },
                                _ => {}
                            }
                            current = ParsedKey::None;
                            literal = Literal::None;
                            println!("status --> {status:?}");
                        },

                        "loop_" => {
                            status = Status::LoopHeader;
                            println!("status --> {status:?}");
                            continue

                        },
                        _ => {
                            match l[0].chars().next() {
                                Some(c) => {
                                    if c != '_'  {
                                        if status == Status::LoopHeader {
                                            status = Status::Loop;
                                            println!("status --> {status:?}");
                                        } 
                                    } else if status == Status::Blank {
                                        status = Status::Dict;
                                        println!("status --> {status:?}");
                                    }
                                },
                                None => {todo!()}
                            }
                            
                        }
                    }

                    match status {
                        Status::Dict => {
                            if l.len() <= 2{
                                literal = Literal::Implicit;
                            } 
                        },
                        _ => {}
                    }
                }
            }




            //println!("Current = {current:?}");
            match status {
                Status::Blank => {continue},
                Status::Loop => {continue},
                Status::LoopHeader |  Status::Dict => {
                    let h: Vec<&str>= l0.split(".").collect();
                    let k =  h[0];
                    match literal {
                        Literal::None => {
                            match current {
                                ParsedKey::None => {
                                    println!("Building new element");
                                    println!("Key = {k}");
                                    match status {
                                        Status::LoopHeader => current = ParsedKey::ListKey(List{key: k.to_string(), val: vec![]}),
                                        Status::Dict => current = ParsedKey::DictKey(Dict{key: k.to_string(), val: vec![]}),
                                        _ => {}
                                    }
                                    println!("Current = {current:?}");
                                },
                                _ => {}
                            }
                        },
                        Literal::Implicit => {
                            literal = Literal::Delimiter(String::from(""), line.chars().next().unwrap());
                        },
                        Literal::Delimiter(_, _) => {

                        }

                    }
                    
                    let mut v = String::from("");


                    for c in line.chars() {
                        match literal{
                            Literal::Delimiter(ref mut vv, cc) => {
                                if c == cc {
                                    println!("LITERAL end");
                                    break
                                }
                                vv.push(c.clone())
                            },
                            _ => {
                                v.push(c.clone());
                            }
                        }
                    }
                    
                    
                    match status {
                        Status::Dict => {
                            match current {
                                ParsedKey::DictKey(ref mut d) => {
                                    match literal {
                                        Literal::None | Literal::Implicit => {
                                            // let h: Vec<&str>= l0.split(".").collect();
                                            // let k =  h[0];
                                            let s = h[1];
                                            d.val.push(DictItem{key: k.to_string(), subkey:s.to_string(), val: v.clone()});
                                        },
                                        _ => {}

                                    }
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    } 
                }
            }
                //     match status {
                //         Status::LoopHeader => {
                //             println!("In loop header = {current:?}");
                            
                //         },
                //         Status::Loop => {

                //         },
                //         Status::Dict => {

                //         //match current.val.push(DictItem{key: k.to_string(),, subkey:s.to_string(),, val: v.to_string(),});

                //         },
                //         _ => {}
                //     }
                // }
        }
    }
    println!("DATANAME:{}", dataname);
    println!("RESULT:\n{:?}", result);
}