use lazy_static::lazy_static;
use regex::Regex;
use std::{env, fs, io};

fn scan_curr_dir<'a>() -> Result<(), io::Error> {
    let current_dir = env::current_dir()?;

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        print!("{:?} ", entry.file_name());
        let file_name = entry.file_name();
        if let Ok(_) = entry.file_type() {
            if let Some(file_name_str) = file_name.to_str() {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"a-(?P<n>\d*).txt").unwrap();
                }
                let parsed = RE.replace_all(file_name_str, "b-$n.txt");
                println!("{:?}", parsed);
                match fs::rename(file_name.clone(), parsed.to_string()) {
                    Ok(_) => (),
                    Err(err) => return Err(err),
                }
            }
        }
    }

    Ok(())
}

fn main() {
    match scan_curr_dir() {
        Ok(_) => return,
        Err(err) => eprintln!("{:?}", err),
    }
}
