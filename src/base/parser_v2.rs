use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::iter::zip;
use rust_decimal::Decimal;


// Misc functions ######################################################################################################
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(&filename)?;
    Ok(io::BufReader::new(file).lines())
}


// Real parser starts here #############################################################################################

const DELIMITERS:[char; 3] = ['\'', ';', '"'];


//#[derive(Debug)]
enum Status{
    Ignored,
    Loop(bool),
    LoopHeader,
    Dict,
    Blank,
}

//#[derive(Debug)]
enum Literal{
    Inline(char),
    Multiline(char),
    None
}

//#[derive(Debug)]
pub struct Item {
    _n: i32,
    pub values: Vec<String>
}

//#[derive(Debug)]
pub struct Field{
    pub mode: String,
    pub key: String,
    pub subkeys: Vec<String>,
    pub subitems: Vec<Item>
}

//#[derive(Debug)]
pub struct Data{
    pub dataname: String,
    pub fields: Vec<Field>
}

//#[derive(Debug)]
enum Pending{
    NewField,
    NewKey,
    None
}

//#[derive(Debug)]
enum Current{
    Field(Field),
    Item(Item),
    None
}


pub fn parse(filepath:String, include:Vec<String>, exclude:Vec<String>, verbose:bool) -> Data {
    let mut include = include.clone();
    let mut had_include = false;
    if include.len() > 0 {
        had_include = true;
    }
    if verbose {
        println!("Parsing file: {filepath}");
        if include.len() > 0 {
            println!("INCLUDE:{:?}", include); // Keys to include
        }
        if exclude.len() > 0 {
            println!("EXCLUDE:{:?}", exclude); // Keys to exclude
        }
    }
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
    let mut literal = Literal::None; // Current literal status

    if let Ok(lines) = read_lines(&filepath) {
        for line in lines.map_while(Result::ok) {
            n += 1;
            // if n >230 {
            //     break
            // }
            // Loop through every line in file
            //println!("");
            if verbose {
                println!("[{n}] {:?}", line);
            }
            let l = re.replace_all(&line, " "); // l is line with compressed spaces (and trimmed)
            let l = l.trim();
            //println!("l: {:?}", l);
            let line_comps: Vec<&str> = l.split(" ").collect(); // line_comps is vector splid by spaces
            //println!("line_comps: {:?}", line_comps);
            let l0 = line_comps[0]; // l0 is first word
            //println!("l0: {:?}", l0);

            let c0 = l0.chars().next(); // c0 is first char (Some()/None)




            // Detect end of block and empty lines
            match literal{
                Literal::Multiline(_) =>{} // Not applied if inside multiline literal
                _ => {
                    match c0 {
                        Some(c) => {
                            match c {
                                // Block end
                                '#' => {
                                    //println!("End of block!");

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
                                    if had_include {
                                        if include.len() == 0 {
                                            break
                                        }
                                    }
                                    pending = Pending::None;
                                    status = Status::Blank;
                                    literal = Literal::None;
                                    current_field = Current::None;
                                    current_item = Current::None;
                                    current_n = 0;
                                    continue
                                },
                                // Line with key
                                '_' => {
                                    //println!("Key detected!");
                                    match status {
                                        Status::Blank | Status::Ignored => {
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
                                            //println!("Loop detected!");
                                            status = Status::LoopHeader;
                                            continue
                                        },
                                        _ => {
                                            if l0.split("_").collect::<Vec<&str>>()[0] == "data" {
                                                // Data name found
                                                //println!("Dataname found!");
                                                data.dataname = l0.split("_").collect::<Vec<&str>>()[1].to_string();
                                                continue
                                            }
                                        }
                                    }
                                    match status {
                                        Status::LoopHeader => {
                                            // Start loop
                                            status = Status::Loop(false);
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        },
                        // Blank line (outside literal)
                        None => {
                            //println!("Empty line!");
                            continue
                        }
                    }
                }

            }
            //println!("STATUS: {:?}", status);

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
                    let mut allowed = true;
                    //println!("{:?}", had_include );
                    if had_include {
                        allowed = false;
                        for (i, ival) in include.iter().enumerate(){
                            if ival == &k {
                                allowed = true;
                                include.remove(i);
                                break
                            }
                        }
                    }
                    if allowed{
                        if exclude.len() > 0{
                            if exclude.contains(&k) {
                                allowed = false;
                            }
                        }
                    }

                    if !allowed {
                        //println!("IGNORED field: {:?}", k);
                        status = Status::Ignored;
                        pending = Pending::None;
                        continue
                    }
                    //println!("ALLOWED field: {:?}", k);

                    //println!("NEW field: {:?}", k);
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
                    //println!("NEW subkey: {:?}", subk);
                    match current_field {
                        Current::Field(ref mut f) => {
                            f.subkeys.push(subk);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
            // match current_field{
            //     Current::Field(ref f) => {println!("CURRENT FIELD: {:?} {:?}", f.key, f.subkeys)},
            //     _ => {}
            // }

            pending = Pending::None; // Reset pending


            // Create subitems
            match status {
                // If dict add new empty subitem
                Status::Dict => {
                    match current_item {
                        Current::None => {
                            //println!("New dict item");
                            current_item = Current::Item(Item{_n:0, values:vec![]});
                        },
                        _ => {}
                    }
                },
                // If loop add new subitem with values
                Status::Loop(mut splitloop)=> {
                    //println!("New loop item");
                    let mut vals = vec![];
                    let mut val = String::from("");
                    let mut add_to_last = false;


                    for c in line.chars() {
                        //print!("{c:?}");
                        match c {
                            ' ' => {
                                match literal {
                                    Literal::None => {
                                        if val.len() > 0 {
                                            //println!("\nNEW VAL: {:?}", val);
                                            vals.push(val.clone());
                                            val = String::from("");
                                            continue
                                        }
                                    },
                                    _ => {
                                        val.push(' ');
                                    }
                                }

                            },
                            cc => {
                                if DELIMITERS.contains(&cc) {
                                    match literal {
                                        Literal::None => {
                                            //println!("\nInline literal start (loop): {c}");
                                            literal = Literal::Inline(cc.clone());
                                            continue
                                        },
                                        Literal::Inline(dd) => {
                                            //println!("\nInline literal (loop): {cc:?} {dd:?}");
                                            if dd == cc {
                                                //println!("Inline literal end (loop): {c}");
                                                literal = Literal::None;
                                                continue
                                            }
                                        },
                                        Literal::Multiline(dd) => {
                                            //println!("\nMultiline literal (loop): {cc:?} {dd:?}");
                                            if dd == cc {
                                                //println!("Multiline literal end (loop): {c}");
                                                literal = Literal::None;
                                                add_to_last = true;
                                                continue
                                            }
                                        }
                                    }

                                }
                                val.push(cc);
                            }
                        }

                    }
                    if val.len() > 0 {
                        //println!("\nNEW VAL (last): {:?}", val);
                        vals.push(val.clone());
                    }
                    match literal {
                        Literal::Inline(dd) => {
                            //println!("\nInline literal unfinished (loop) -> Multiline literal");
                            literal = Literal::Multiline(dd);
                        },
                        Literal::Multiline(_) => {
                            //println!("\nInside Multiline literal");
                            add_to_last = true;
                            splitloop = true
                        }
                        _ => {}
                    }
                    // Update/add item (loop)
                    match current_field {
                        Current::Field(ref mut f) => {
                            //println!("SUBK: {:?}", f.subkeys);
                            //println!("VALS: {:?}", &vals);
                            if splitloop {
                                // last has missing vals -> push to last item

                                let last_n = &f.subitems.len() - 1;
                                let last = &mut f.subitems[last_n];

                                // If there was a multiline literal then extend last value from last item
                                //println!("{:?}/{:?}",add_to_last,  vals.len() > 0);
                                if add_to_last && vals.len() > 0{
                                    let first_val = &vals[0];
                                    let last_val_n = last.values.len() - 1;
                                    let last_val = last.values[last_val_n].clone();
                                    //println!("Extending to last item [{}]: {:?} + {:?}", last_val_n, last_val, first_val);
                                    last.values[last_val_n] = last_val + first_val;
                                    vals = vals[1..].to_vec(); // remove first val
                                }

                                //let mut last_values = last.values;
                                //println!("Pushing to last item: {:?}", vals);
                                last.values.extend(vals);
                                //println!("NEW values: {:?}", last.values);
                                if last.values.len() == f.subkeys.len() {
                                    status = Status::Loop(false);
                                } else if last.values.len() > f.subkeys.len() {
                                    todo!("Too many values in loop item: N subkeys: {:?} / N vals: {:?}", f.subkeys.len(), last.values.len());
                                }

                            } else {
                                // add new item
                                if vals.len() > 0{
                                    //println!("Pushing new item: {:?}", vals);
                                    f.subitems.push(Item{_n:current_n, values:vals});
                                }
                            }
                            let last_n = &f.subitems.len() - 1;
                            //println!("FINAL VALUES {:?}", &f.subitems[last_n].values);
                            let last_n_vals = f.subitems[last_n].values.len();
                            if last_n_vals < f.subkeys.len() {
                                // if missing vals declare splitloop
                                //println!("Key value missmatch: N subkeys: {:?} / N vals: {:?}", f.subkeys.len(), last_n_vals);
                                status = Status::Loop(true);
                            } else if last_n_vals > f.subkeys.len() {
                                todo!("Too many values in loop item: N subkeys: {:?} / N vals: {:?}", f.subkeys.len(), last_n_vals);
                            }

                        },
                        _ => {}
                    }
                    current_n += 1;
                },
                _ => {}
            }



            // Update subitem (dict)
            match current_item {
                Current::Item(ref mut i) => {
                    //println!("CURRENT ITEM: {i:?}");
                    match status {
                        // If dict update current item
                        Status::Dict => {
                            let mut val = String::from("");
                            let l1 = &line.split(" ").collect::<Vec<&str>>()[1..];
                            let l1 = l1.join(" ");
                            let mut l1 = l1.trim();

                            let mut add_to_last = false;
                            // If dict line does not start with _, then parse entire line
                            match c0 {
                                Some(cc) => {
                                    //println!("C0: {cc:?}");
                                    match cc{
                                        '_' => {},
                                        _ => {
                                            l1 = &line;
                                            //println!("No key -> add_to_last");
                                            add_to_last = true;
                                        }
                                    }
                                },
                                _ => {}
                            }

                            //println!("LINE: {l1:?}");
                            for c in l1.chars() {
                                //print!("{c:?}");
                                if DELIMITERS.contains(&c) {
                                    match literal {
                                        Literal::None => {
                                            //println!("Inline literal start (dict): {c}");
                                            literal = Literal::Inline(c.clone());
                                            continue
                                        },
                                        Literal::Inline(dd) | Literal::Multiline(dd) => {
                                            //println!("Inline literal (dict): {c:?} {dd:?}");
                                            if dd == c {
                                                //println!("Inline literal end (dict): {c}");
                                                literal = Literal::None;
                                                continue
                                            }
                                        }
                                    }
                                }
                                val.push(c);
                            }
                            //println!("VAL: {val:?}");

                            match literal {
                                Literal::Inline(dd) => {
                                    //println!("Inline literal unfinished (dict) -> Multiline literal");
                                    literal = Literal::Multiline(dd);
                                },
                                Literal::Multiline(_) => {
                                    //println!("Multiline -> add_to_last");
                                    add_to_last = true;
                                },
                                _ => {}
                            }

                            if add_to_last {
                                //println!("Adding to last");
                                let last_n = i.values.len()-1;
                                let last = i.values[last_n].clone();
                                i.values[last_n] = last + &val;

                            } else {
                                i.values.push(val);
                            }



                        },
                        _ => {}
                    }
                    //println!("UPDATED ITEM: {i:?}, {}", i.values.len());
                },
                _ => {}
            }
        }
    }
    //println!("\nDATA: {:?}", data);
    //println!("\nFILEPATH: {}", &filepath);
    return data; // Return parsed data
}





pub fn data_to_json(data:Data, indent:i32) -> String{
    let mut json_string = String::from("{");

    for field in data.fields{
        if indent > 0{ json_string += "\n"; for _ in 0..indent {json_string += " ";} }
        json_string += &format!("{:?}", &field.key);
        json_string += ": [";

        match field.mode.as_str() {
            "dict"|"loop" => {
                for item in field.subitems{
                    if indent > 0{ json_string += "\n"; for _ in 0..indent*2 {json_string += " ";} }
                    json_string += "{";
                    for (k, v) in zip(field.subkeys.iter(), item.values.iter()){
                        if indent > 0{ json_string += "\n"; for _ in 0..indent*3 {json_string += " ";} }
                        match v.chars().next() {
                            Some(cc) => { match cc { '.' | '?' => {json_string += &format!("{:?}: null,", &k);continue}, _ => {}} },
                            None => {json_string += &format!("{:?}: null,", &k);continue}
                        }
                        let chars:Vec<_>= v.chars().collect();
                        if chars.contains(&'E') | chars.contains(&'e') {json_string += &format!("{:?}: {:?},", &k, &v);continue}

                        match v.parse::<i32>() { Ok(vv) => {json_string += &format!("{:?}: {},", &k, vv);continue}, _ => {}}
                        match v.parse::<bool>() { Ok(vv) => {json_string += &format!("{:?}: {},", &k, vv);continue}, _ => {}}
                        match v.parse::<f32>() { Ok(_) => {json_string += &format!("{:?}: {},", &k, &v);continue}, _ => {}}
                        json_string += &format!("{:?}: {:?},", &k, &v);
                    }
                    json_string.pop();
                    if indent > 0{ json_string += "\n"; for _ in 0..indent*2 {json_string += " ";} }
                    json_string += "},";
                }
                json_string.pop();
            },
            _ => {}
        }
        if indent > 0{ json_string += "\n"; for _ in 0..indent {json_string += " ";} }
        json_string += "],";
    }
    json_string.pop();
    if indent > 0{ json_string += "\n";}
    json_string.push('}');

    return json_string

}


use std::env;
//use std::io::{self, Write};
use write_json;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() > 1, "No file provided");

    let filepath = &args[1];
    let data = parse(filepath.to_string(), vec![], vec![], false);
    let mut buf = String::new();
    let mut obj = write_json::object(&mut buf);
    for field in data.fields{
        let mut a = obj.array(&field.key);
        match field.mode.as_str() {
            "dict"|"loop" => {
                for item in field.subitems{
                    let mut o = a.object();
                    for (k, v) in zip(field.subkeys.iter(), item.values.iter()){
                        o.string(k, v);
                    }
                }
            },
            _ => {}
        }
    }

    //println!("{:?}", &obj);


    //io::stdout().write_all(b"hello world")?;
    Ok(())
}




// #####################################################################################################################