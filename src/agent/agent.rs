use crate::agent::parser;

use super::models::AttackEvent;
use super::parser::AttackParser;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::usize;

const LOG_PATH: &str = "/var/log/auth.log";

fn checker() {
    let file = File::open(LOG_PATH).expect("log file not found");
    let mut reader = BufReader::new(file);

    // real time tracking by jumping to end
    reader.seek(SeekFrom::End(0));
    let parser = AttackParser::new();
    let mut line = String::new();
    loop {
        read_log(&mut reader, &mut line, &parser);
    }
}

fn read_log(reader: &mut BufReader<File>, line: &mut String, parser: &AttackParser) {
    match reader.read_line(line) {
        Ok(0) => {
            // EOF
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        Ok(usize) => {
            if let Some(event) = parser.parse_line(line) {
                println!("target event detected, {}", event.event_type);
            }
            line.clear();
        }
        Err(_) => {
            eprintln!("error reading the file...");
        }
    }
}
