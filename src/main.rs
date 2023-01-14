use colored::*;
use std::env;
use std::fs;

mod analyzer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "{}",
            "# SLIC: data serialization language made just for fun".dimmed()
        );
        println!("{} = {}", "version".blue(), "1.0".green());
        println!("{} = {}", "author".blue(), "\"Thebigbot\"".yellow());
        println!("{} = {}", "license".blue(), "\"NO-LICENSE\"".yellow());
        println!("{} = {}", "are_you_cool".blue(), "true".red());
        return;
    }

    let content = fs::read_to_string(&args[1]).expect("Unable to read file");

    if args.len() == 3 {
        match args[2].as_str() {
            "json" => {
                fs::write("./compiled.json", analyzer::convert_to_json(&content)).expect("Unable to write file");
            }

            "yaml" => {
                fs::write("./compiled.yaml", analyzer::convert_to_yaml(&content)).expect("Unable to write file");
            }
            _ => {
                println!("Options: json or yaml")
            }
        }
    }

    let lines: Vec<&str> = content.lines().collect();

    for line in lines {
        let result = analyzer::analyze_string(line);
        println!("{}: {} = {}", result[0].blue(), result[2].yellow(), result[1].green());
    }
}
