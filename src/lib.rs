use std::fs::File;
use::std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;

pub fn grep_lite(search_term:&str, context_lines:usize, file:&str)
{
    let reader1:BufReader<File>;
    let reader2:BufReader<File>;
    if file == "-" {
        let stdin = io::stdin();
        let _reader = stdin.lock();
    }
    else{

    }
    let f1 = File::open(file).unwrap();
    let f2 = File::open(file).unwrap();
    reader1 = BufReader::new(f1);
    reader2 = BufReader::new(f2);
    let re = Regex::new(search_term).unwrap();
    //Initialization
    let mut tags :Vec<usize> = Vec::new();
    let mut ctx :Vec<Vec<(usize, String)>> = Vec::new();
    for(i, line_) in reader1.lines().enumerate(){
        let line = line_.unwrap();
        match re.find(&line)

        {
            Some(_) =>{
                tags.push(i);
                let v: Vec<(usize, String)> = Vec::with_capacity(2*context_lines + 1);
                ctx.push(v)
            },
            None => ()
        }
    }

    if tags.len() == 0{
        return;
    }

    //Pass 2

    for (i, line_ ) in reader2.lines().enumerate()
    {
        let line = line_.unwrap();
        for(j, tag) in tags.iter().enumerate()
        {
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if(i >= lower_bound) && (i <= upper_bound)
            {
                let line_as_string = String::from(&line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    //Output
    for local_ctx in ctx.iter()
    {
        for &(i, ref line) in local_ctx.iter(){
            let line_num = i + 1;
            println!("{}: {}", line_num, line)
        }
    }
}