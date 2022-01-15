use clap::{App, Arg};

// example of using clap

//everything in rust is private by default
// we must make things public by adding the pub keyword
pub fn testing() {
    let matches = App::new("Clap Testing")
        .version("0.1.0")
        .author("Lev")
        .about("Teaches Clap")
        //from here on we are defining our CLI commands
        .arg(
            Arg::new("file")
                .short('f') // i.e,  --f
                .long("file") // i.e -file
                .takes_value(true)
                .help("A cool file"),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .takes_value(true)
                .help("A cool num"),
        )
        //afterwards we get any matches
        .get_matches();

    //unwrapped_or returns a default value
    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("File input is {}", myfile);
    //note, in order to pass arguments to the rust executable use --
    // for example: cargo run -- -f my_file.txt -n 99

    //extracting number
    let my_num = matches.value_of("num");

    //pattern matching!
    match my_num {
        None => println!("I can't guess your number"),
        //using parse's pattern matching
        Some(s) => match s.parse::<i32>() {
            Ok(n) => println!("Your number is {}", n),
            Err(_) => println!("Please insert your number"),
        },
    }
}
