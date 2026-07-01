use crate::agent::parser;
use crate::messages::bus::EventBus;
use crate::messages::event::Event;

use super::models::AttackEvent;
use super::parser::AttackParser;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::usize;

const LOG_PATH: &str = "/var/log/auth.log";

pub fn checker(bus: EventBus) {
    let file = File::open(LOG_PATH).expect("log file not found");
    let mut reader = BufReader::new(file);

    reader.seek(SeekFrom::End(0));
    let parser = AttackParser::new();
    let mut line = String::new();

    loop {
        read_log(&mut reader, &mut line, &parser, &bus);
    }
}

fn read_log(
    reader: &mut BufReader<File>,
    line: &mut String,
    parser: &AttackParser,
    bus: &EventBus,
) {
    match reader.read_line(line) {
        Ok(0) => {
            // EOF
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        Ok(usize) => {
            if let Some(event) = parser.parse_line(line) {
                bus.publish(Event::Attacked(event));
            }
            line.clear();
        }
        Err(_) => {
            eprintln!("error reading the file...");
        }
    }
}
