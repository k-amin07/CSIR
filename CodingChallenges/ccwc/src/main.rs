use clap::{command, Arg, ArgAction};
use std::{fs};

enum Opts {
    Bytes,
    Lines,
    Words,
    None
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
    .arg(Arg::new("filename").required(false))
    .get_matches();

    let (bytes, lines, words) = (
        matches.get_flag("bytes"),
        matches.get_flag("lines"),
        matches.get_flag("words")
    );

    let mut opts:Opts = Opts::None;
    
    if lines {
        opts = Opts::Lines;
    } else if words {
        opts = Opts::Words;
    } else if bytes {
        opts = Opts::Bytes;
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

    match opts {
        Opts::Bytes => println!("{} {}", count_bytes(&contents), file_name),
        Opts::Words => println!("{} {}", count_words(&contents), file_name),
        Opts::Lines => println!("{} {}", count_lines(&contents), file_name),
        Opts::None  => println!("{} {} {} {}", 
            count_bytes(&contents), 
            count_lines(&contents),
            count_words(&contents), 
            file_name 
        )
    };
}