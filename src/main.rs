mod instructions;
mod cpu;



use clap::{App, load_yaml};
use cpu::CPU;

fn parse_args() -> String {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    match matches.value_of("IMAGE") {
        Some(s) => s.to_owned(),
        None => panic!("No image was specified"),
    }
}


fn main() {
    let file = parse_args();
    let mut processor = CPU::initiate();
    processor.load_instructions(file).unwrap();
    loop {
        processor.fetch_instruction();
        processor.execute();
        if processor.is_finished {break};
    }
    processor.terminate();
}