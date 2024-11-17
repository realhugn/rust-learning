use std::env;
use std::fs::File;
use std::io::{self, Read};

struct CCWC {
    bytes: usize,
    lines: usize,
    words: usize,
    chars: usize,
}

impl CCWC {
    fn new() -> Self {
        CCWC {
            bytes: 0,
            lines: 0,
            words: 0,
            chars: 0,
        }
    }

    // Count the number of bytes, lines, words, and characters from a reader
    fn count_from_reader<R: Read>(&mut self, reader: R) {
        let mut buf_reader = io::BufReader::new(reader);
        let mut buffer = String::new();
        while let Ok(bytes_read) = buf_reader.read_to_string(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            self.bytes += bytes_read;
            self.lines += buffer.lines().count();
            self.words += buffer.split_whitespace().count();
            self.chars += buffer.chars().count();
            buffer.clear();
        }
    }
    
    // -c: print the byte counts
    // -l: print the newline counts
    // -w: print the word counts
    // -m: print the character counts
    // default: print the newline, word, and byte counts
    fn print_counts(&self, filename: &str, options: &[String]) {
        if options.contains(&"-c".to_string()) {
            println!("{:>8} {}", self.bytes, filename);
        } else if options.contains(&"-l".to_string()) {
            println!("{:>8} {}", self.lines, filename);
        } else if options.contains(&"-w".to_string()) {
            println!("{:>8} {}", self.words, filename);
        } else if options.contains(&"-m".to_string()) {
            println!("{:>8} {}", self.chars, filename);
        } else {
            println!("{:>8} {:>8} {:>8} {}", self.lines, self.words, self.bytes, filename);
        }
    }
}

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
