use std::env;
use std::fs::File;
use std::io::{self, Read};
use wc::ccwc::CCWC;

struct App {
    ccwc: CCWC,
    args: Vec<String>,
}

impl App {
    fn new(args: Vec<String>) -> Self {
        App {
            ccwc: CCWC::new(),
            args,
        }
    }

    fn run(&mut self) {
        if self.args.len() <= 1 {
            self.ccwc.count_from_reader(io::stdin());
            self.ccwc.print_counts("", &self.args);
        } else {
            let options: Vec<String> = self.args[1..].iter().filter(|arg| arg.starts_with('-')).cloned().collect();
            let filename = {
                let filename_opt = self.args[1..].iter().find(|arg| !arg.starts_with('-'));
                filename_opt.cloned().unwrap_or_else(|| "".to_string())
            };

            if filename.is_empty() {
                self.ccwc.count_from_reader(io::stdin());
            } else {
                let file = File::open(&filename).expect("Could not open file");
                self.ccwc.count_from_reader(file);
            }
            self.ccwc.print_counts(&filename, &options);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut app = App::new(args);
    app.run();
}
