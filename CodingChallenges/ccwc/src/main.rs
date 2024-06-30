use clap::{command, Arg, ArgAction};
use std::{fs};

enum Opts {
    Bytes,
    Lines,
    Words
}

struct Cmd {
    opts: Opts,
    file_name: String
}

fn get_args() -> Cmd {
    let matches = command!()
    .arg(Arg::new("bytes").short('c').long("bytes").action(ArgAction::SetTrue).required(false))
    .arg(Arg::new("lines").short('l').long("lines").action(ArgAction::SetTrue).required(false))
    .arg(Arg::new("words").short('w').long("words").action(ArgAction::SetTrue).required(false))
    .arg(Arg::new("filename"))
    .get_matches();

    let (bytes, lines, words) = (
        matches.get_flag("bytes"),
        matches.get_flag("lines"),
        matches.get_flag("words")
    );

    if !(bytes || lines || words) {
        panic!("At least one flag is required. Use --help to see the valid options");
    }
    let mut opts:Opts = Opts::Bytes;
    
    if lines {
        opts = Opts::Lines;
    } else if words {
        opts = Opts::Words;
    }


    let file_name = String::from(matches.get_one::<String>("filename").expect("Error: File name is required!"));

    return Cmd{opts, file_name};
}


fn count_bytes(contents: &String) -> i32 {
    contents.len() as i32
}

fn count_words(contents: &String) -> i32 {
    contents.split(" ").count() as i32
}

fn count_lines(contents: &String) -> i32 {
    contents.split("\n").count() as i32
}

fn main() {
    let args:Cmd = get_args();
    let file_name = args.file_name;
    let opts = args.opts;
    let contents = fs::read_to_string(&file_name)
        .expect("Should have been able to read the file");

    let required_count = match opts {
        Opts::Bytes => count_bytes(&contents),
        Opts::Words => count_words(&contents),
        Opts::Lines => count_lines(&contents)
    };

    println!("{} {}", required_count,file_name);
}