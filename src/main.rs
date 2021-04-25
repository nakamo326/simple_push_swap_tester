use clap::Clap;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::process::{Command, Stdio};
use std::str;
use std::io::{stdout, Cursor};
use std::io::prelude::*;
use termion::color;

#[derive(Clap, Debug)]
#[clap(
    name = "spst",
    version = "0.1.0",
    author = "nakamo326",
    about = "simple push_swap checker\ncheck sorting five times and output average step."
)]
struct Cli {
    /// output push_swap's stderr
    #[clap(short = 'd', long = "debug")]
    debug: bool,
    /// argument size to sort. default is 100.
    #[clap(name = "SIZE")]
    size: Option<usize>,
}

fn main() {
    let args = Cli::parse();
    let mut rng = thread_rng();
    let mut list: Vec<String> = Vec::new();
    let mut steps:[usize; 5] = [0; 5];
    let mut size: usize = 100;

    if let Some(input) = args.size {
        size = input;
    }
    for n in 0..size {
        list.push(n.to_string());
    }

    for n in 1..6 {
        list.shuffle(&mut rng);
        //let argument = format!("{:?}", list).replace(", ", " ").replace("\"", "");
        //println!("argument: {:?}", argument);
        let p_s = Command::new("./push_swap")
                                .args(list.iter())
                                .output()
                                .expect("failed to execute process");

        let answer = p_s.stdout;
        steps[n - 1] = answer.split(|&c| c == '\n' as u8).count() - 1;

        if args.debug == true {
            let debug_print = str::from_utf8(&p_s.stderr).unwrap();
            println!("{}", debug_print);
        }

        let mut checker = Command::new("./checker")
                                .args(list.iter())
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect("failed to execute process");

        let input = checker.stdin.as_mut().unwrap();
        let mut output = Cursor::new(answer);
        std::io::copy(&mut output, input).unwrap();

        let output = checker
        .wait_with_output()
        .expect("failed to wait on child");
        let result = str::from_utf8(&output.stdout).unwrap();
        let eresult = str::from_utf8(&output.stderr).unwrap();

        print!("Test{} step count is {}{}{}, checker output ... "
            , n, color::Fg(color::Blue), steps[n - 1], color::Fg(color::Reset));
        stdout().flush().unwrap();
        if result == "OK\n" {
            println!("{}OK!{}", color::Fg(color::Green), color::Fg(color::Reset));
        } else if result == "KO\n" {
            println!("{}KO{}", color::Fg(color::Red), color::Fg(color::Reset));
        } else if eresult == "Error\n" {
            println!("{}Error{}", color::Fg(color::Red), color::Fg(color::Reset));
        }
    }
    let mut sum:usize = steps.iter().sum();
    sum /= 5;
    println!("step average is {}{}{}"
            , color::Fg(color::Blue), sum, color::Fg(color::Reset));
}
