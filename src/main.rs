extern crate regex;
extern crate clap;

use greplite::grep_lite;
use clap::{App, Arg};

fn main() {
    let args = App::new("grep-lite")
    .version("1.0.0")
    .about("searches for patterns in a string")
    .arg(Arg::with_name("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(true))
    .arg(Arg::with_name("filename")
        .help("File to search")
        .takes_value(true)
        .required(true))
    .arg(Arg::with_name("context")
        .help("number of lines to print before and after search")
        .takes_value(true)
        .required(true))
    .get_matches();

    let search_term = args.value_of("pattern").unwrap();
    let context = args.value_of("context").unwrap_or("-");
    let context_lines;
     if context == "-" {
        context_lines = 0
    }
    else{
        context_lines = 1
    }
    let file = args.value_of("input").unwrap_or("-");
    grep_lite(&search_term, context_lines, &file );
}
