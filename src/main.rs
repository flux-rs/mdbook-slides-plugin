use clap::{Arg, Command};
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_slides::SlidesPreprocessor;
use std::io;
use std::process;

fn main() {
    let matches = Command::new("mdbook-slides")
        .about("An mdbook preprocessor that wraps sections in slide divs")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .get_matches();

    let preprocessor = SlidesPreprocessor::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        let renderer = sub_args
            .get_one::<String>("renderer")
            .expect("Required argument");
        let supported = preprocessor.supports_renderer(renderer);

        if supported {
            process::exit(0);
        } else {
            process::exit(1);
        }
    }

    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let processed_book = preprocessor.run(&ctx, book).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    serde_json::to_writer(io::stdout(), &processed_book).unwrap();
}
