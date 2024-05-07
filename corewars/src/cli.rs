use std::{
    error::Error,
    fs,
    io::{self, Read},
    path::PathBuf,
};

use clap::Parser;
use lazy_static::lazy_static;

use corewars_parser as parser;
use corewars_sim::Core;

lazy_static! {
    static ref IO_SENTINEL: PathBuf = PathBuf::from("-");
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab")]
/// Parse, assemble, and save Redcode files
struct CliOptions {
    /// The corewars subcommand to perform
    #[clap(subcommand)]
    command: Command,

    /// Print additional details while running
    // TODO(#26) hook this up to a log level
    #[clap(long, short)]
    verbose: bool,

    /// Input file; use "-" to read from stdin
    input_file: PathBuf,
}

#[derive(Debug, Parser)]
enum Command {
    /// Save/print a program in "load file" format
    #[clap(name = "dump")]
    Dump {
        /// Output file; defaults to stdout ("-")
        #[clap(long, short, default_value = "-")]
        output_file: PathBuf,

        /// Whether labels, expressions, macros, etc. should be resolved and
        /// expanded in the output
        #[clap(long, short = 'E')]
        no_expand: bool,
    },

    /// Run a warrior to completion
    #[clap(name = "run")]
    Run {
        /// The max number of cycles to run. Defaults to
        #[clap(long, short)]
        max_cycles: Option<usize>,
    },
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli_options = CliOptions::parse();

    let mut input = String::new();

    if cli_options.input_file == *IO_SENTINEL {
        io::stdin().read_to_string(&mut input)?;
    } else {
        input = fs::read_to_string(cli_options.input_file)?;
    }

    let parsed_core = match parser::parse(input.as_str()) {
        parser::Result::Ok(warrior, warnings) => {
            print_warnings(&warnings);
            Ok(warrior)
        }
        parser::Result::Err(err, warnings) => {
            print_warnings(&warnings);
            Err(err)
        }
    }?;

    match cli_options.command {
        Command::Dump {
            output_file,
            no_expand,
        } => {
            if no_expand {
                unimplemented!()
            }

            if output_file == *IO_SENTINEL {
                println!("{parsed_core}");
            } else {
                fs::write(output_file, format!("{parsed_core}\n"))?;
            };
        }
        Command::Run { max_cycles } => {
            let mut core = Core::default();
            core.load_warrior(&parsed_core)?;

            match core.run(max_cycles) {
                Ok(_) => println!(
                    "Warrior stopped after {}max of {} cycles",
                    if max_cycles.is_some() {
                        "specified "
                    } else {
                        ""
                    },
                    core.steps_taken()
                ),
                Err(err) => println!("Warrior failed after {} steps: {err}", core.steps_taken()),
            }

            if cli_options.verbose {
                println!("Core after execution:\n{core}");
            }
        }
    };

    Ok(())
}

fn print_warnings(warnings: &[parser::Warning]) {
    for warning in warnings.iter() {
        eprintln!("Warning: {warning}");
    }
}
