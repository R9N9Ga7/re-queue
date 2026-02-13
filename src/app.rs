use std::io::{self, Write};

use re_queue::storage_manager::StorageManager;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Command {
    Save,
    Pick,
    MoveNext,
    Exit,
    Help,
    List,
    CreateStorage,
    OpenStorage,
    StorageList,
}

impl Command {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "save" => Some(Command::Save),
            "pick" => Some(Command::Pick),
            "next" => Some(Command::MoveNext),
            "exit" => Some(Command::Exit),
            "help" => Some(Command::Help),
            "list" => Some(Command::List),
            "create-storage" => Some(Command::CreateStorage),
            "open-storage" => Some(Command::OpenStorage),
            "storage-list" => Some(Command::StorageList),
            _ => None,
        }
    }
}

enum Mode {
    AwaitCommand,
    AwaitValue(Command),
}

pub struct App {
    storage_manager: StorageManager
}

impl App {
    pub fn new() -> Self {
        Self {
            storage_manager: StorageManager::new(),
        }
    }

    pub fn run(&mut self) {
        println!("=======");

        let mut mode = Mode::AwaitCommand;

        if !self.storage_manager.has_storages() {
            println!("You don't have any storages, please create one.");
            mode = Mode::AwaitValue(Command::CreateStorage);
        }

        loop {
            match mode {
                Mode::AwaitCommand => {
                    println!("");
                    println!("Current storage: {}", self.storage_manager.get_active_storage_name());
                    print!("Write command (help to list): ");
                    io::stdout().flush().unwrap();

                    let line = Self::read_line_trimmed().unwrap();

                    match Command::parse(&line) {
                        Some(Command::Exit) => break,
                        Some(Command::Help) => {
                            println!("Available commands: save, pick, next, exit, help, list, create-storage, open-storage, storage-list");
                        }
                        Some(Command::Pick) => {
                            let value = self.storage_manager.get_active_storage().pick().unwrap();
                            println!("{value}");
                        }
                        Some(Command::MoveNext) => {
                            self.storage_manager.get_active_storage().move_next().unwrap();
                            println!("<next>");
                        }
                        Some(Command::Save) => {
                            mode = Mode::AwaitValue(Command::Save);
                        }
                        Some(Command::List) => {
                            let records = self.storage_manager.get_active_storage().get_all().unwrap();
                            println!("*******");
                            for record in records {
                                println!("({}): {}", record.meta.get_id(), record.data);
                            }
                            println!("*******");
                        }
                        Some(Command::CreateStorage) => {
                            mode = Mode::AwaitValue(Command::CreateStorage);
                        }
                        Some(Command::OpenStorage) => {
                            mode = Mode::AwaitValue(Command::OpenStorage);
                        }
                        Some(Command::StorageList) => {
                            let list = self.storage_manager.get_list();
                            println!("*******");
                            for storage_name in list {
                                println!("{storage_name}");
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
                    self.storage_manager.get_active_storage().save(value).unwrap();

                    println!("<save>");

                    mode = Mode::AwaitCommand;
                }
                Mode::AwaitValue(Command::CreateStorage) => {
                    print!("Write storage name: ");
                    io::stdout().flush().unwrap();

                    let value = Self::read_line_trimmed_end().unwrap();
                    self.storage_manager.create(&value);
                    self.storage_manager.open(&value);

                    println!("<create-storage>");

                    mode = Mode::AwaitCommand;
                }
                Mode::AwaitValue(Command::OpenStorage) => {
                    print!("Write storage name: ");
                    io::stdout().flush().unwrap();

                    let value = Self::read_line_trimmed_end().unwrap();
                    self.storage_manager.open(&value);

                    println!("<open-storage>");

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
