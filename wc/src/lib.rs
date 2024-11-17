
pub mod ccwc {
    use std::io::Read;

    pub struct CCWC {
        pub bytes: usize,
        pub lines: usize,
        pub words: usize,
        pub chars: usize,
    }

    impl CCWC {
        pub fn new() -> Self {
            CCWC {
                bytes: 0,
                lines: 0,
                words: 0,
                chars: 0,
            }
        }

        // Count the number of bytes, lines, words, and characters from a reader
        pub fn count_from_reader<R: Read>(&mut self, reader: R) {
            let mut buf_reader = std::io::BufReader::new(reader);
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
        pub fn print_counts(&self, filename: &str, options: &[String]) {
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
}