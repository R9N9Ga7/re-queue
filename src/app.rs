use std::io::{self, Write};

use re_queue::storage::Storage;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Command {
    Save,
    Pick,
    MoveNext,
    Exit,
    Help,
    ShowAll,
}

impl Command {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "save" => Some(Command::Save),
            "pick" => Some(Command::Pick),
            "next" => Some(Command::MoveNext),
            "exit" => Some(Command::Exit),
            "help" => Some(Command::Help),
            "all" => Some(Command::ShowAll),
            _ => None,
        }
    }
}

enum Mode {
    AwaitCommand,
    AwaitValue(Command),
}

pub struct App {
    storage: Storage,
}

impl App {
    pub fn new() -> Self {
        Self {
            storage: Storage::new("storage", "meta", "data").unwrap(),
        }
    }

    pub fn run(&mut self) {
        println!("=======");

        let mut mode = Mode::AwaitCommand;
        loop {
            match mode {
                Mode::AwaitCommand => {
                    print!("Write command (help to list): ");
                    io::stdout().flush().unwrap();

                    let line = Self::read_line_trimmed().unwrap();

                    match Command::parse(&line) {
                        Some(Command::Exit) => break,
                        Some(Command::Help) => {
                            println!("Available commands: save, pick, next, exit, help, all");
                        }
                        Some(Command::Pick) => {
                            let value = self.storage.pick().unwrap();
                            println!("{value}");
                        }
                        Some(Command::MoveNext) => {
                            self.storage.move_next().unwrap();
                            println!("<next>");
                        }
                        Some(Command::Save) => {
                            mode = Mode::AwaitValue(Command::Save);
                        }
                        Some(Command::ShowAll) => {
                            let records = self.storage.get_all().unwrap();
                            println!("*******");
                            for record in records {
                                println!("{}", record.data);
                            }
                            println!("*******");
                        }
                        None => {
                            println!("Unknown command: {line}");
                        }
                    }
                }
                Mode::AwaitValue(Command::Save) => {
                    print!("Write value to save: ");
                    io::stdout().flush().unwrap();

                    let value = Self::read_line_trimmed_end().unwrap();
                    self.storage.save(value).unwrap();

                    println!("<save>");

                    mode = Mode::AwaitCommand;
                }
                Mode::AwaitValue(_) => {
                    mode = Mode::AwaitCommand;
                }
            }
        }

        println!("=======");
    }

    fn read_line_trimmed() -> io::Result<String> {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        Ok(s.trim().to_string())
    }

    fn read_line_trimmed_end() -> io::Result<String> {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        Ok(s.trim_end().to_string())
    }
}
