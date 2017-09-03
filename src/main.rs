extern crate libc;
extern crate rustyline;
extern crate num;

mod ast;
mod parser;

use std::io;
use std::io::BufRead;

use rustyline::error::ReadlineError;
use parser::expr;

fn main() {
    ::std::process::exit(real_main());
}

fn real_main() -> i32 {
    let in_isatty = unsafe { libc::isatty(libc::STDIN_FILENO as i32) } != 0;
    let out_isatty = unsafe { libc::isatty(libc::STDOUT_FILENO as i32) } != 0;

    let prompt = "> ";

    // rustyline input & output
    let mut rl = rustyline::Editor::<()>::new();

    // pipe input, since reading rustyline changes stdout
    let mut pipe_reader = io::BufReader::new(io::stdin()).lines();

    if in_isatty && out_isatty {
        println!("Interactive Roller REPL started");
    }

    // return value
    let return_status = loop {
        // read a line
        let line_res = if out_isatty {
            rl.readline(prompt)
        } else {
            pipe_reader.next()
                .map_or(
                    Err(ReadlineError::Eof),
                    |res| res.map_err(&ReadlineError::from),
                )
        };

        match line_res {
            Ok(ref input) if input.trim().is_empty() => continue, // empty input

            Ok(input) => {
                // ok input
                if out_isatty {
                    rl.add_history_entry(&input.trim_right());
                } else {
                    // print prompt and input into pipe output
                    // we might want to give a flag to supress this
                    println!("{}{}", prompt, input);
                }

                let input = input.trim();
                let parsed_res = expr::parse_expr(input);

                println!("Result is: {:?}", parsed_res);
            },

            // TODO: maybe check ReadlineError::WindowResize when on windows

            Err(ReadlineError::Interrupted) => {
                // received interrupt (ctrl+C)
                eprintln!("Interrupt received");
                break 0;
            },

            Err(ReadlineError::Eof) => {
                // received EOF (ctrl+D, or end of pipe input)
                if in_isatty {
                    eprintln!("End-of-file received");
                }
                break 0;
            },

            Err(e) => {
                // other error, maybe IO error
                // in some cases we might want to continue, but we don't want 
                // any infinite loops
                eprintln!("Encountered error: {:?}", e);
                break 1;
            },
        }
    };
    
    if in_isatty {
        println!("Exiting");
    }

    return_status
}
