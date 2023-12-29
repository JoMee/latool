use clap::{App, SubCommand};
mod homework;

fn main() {
    let matches = App::new("LaTeX Tool")
        .version("1.0")
        .author("Jonas Meier")
        .about("A small command line tool for LaTeX workflow")
        .subcommand(SubCommand::with_name("homework").alias("hw").about("Generate a blank homework sheet"))
        .get_matches();

    match matches.subcommand_name() {
        Some("homework" | "hw") => homework::generate_homework(),
        _ => {
            println!("{}", matches.usage());

        }
    }
}

