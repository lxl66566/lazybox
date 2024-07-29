use clap::{arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("test")
        .about("Run lazybox arg test")
        .arg(arg!(<input> "test input"))
}

pub fn execute(args: &ArgMatches) {
    let test_str = args.get_one::<String>("input").unwrap();
    println!("get input: {}", test_str);
}
