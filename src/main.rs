use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

// Task priority enumeration
#[derive(Serialize, Deserialize)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    // This method returns a string variant of enum variant
    // In this case i can use just a debug token in format, but
    // this method was created for more flexibility
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_owned(),
            Priority::Medium => "Medium".to_owned(),
            Priority::High => "High".to_owned()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    description: String,
    priority: Priority,
    add_time: DateTime<Local>,
}

impl Task {
    // This method creates new Task object from given parameters
    fn new(name: String, description: String, priority: Priority) -> Self {
        Self { name, description, priority, add_time: Local::now() }
    }

    // This method creates new Task object from user input data
    fn new_from_console() -> Option<Self> {
        let name = match ConsoleManager::input("Enter new task name: ") {
            Ok(name) => name,
            Err(err) => {
                println!("Error getting user input: {}", err);
                return None;
            }
        };
        let description = match ConsoleManager::input("Enter new task description: ") {
            Ok(description) => description,
            Err(err) => {
                println!("Error getting user input: {}", err);
                return None;
            }
        };
        let priority = match ConsoleManager::input("Enter new task priority: ") {
            Ok(priority) => match priority.to_lowercase().as_str() {
                "low" => Priority::Low,
                "medium" => Priority::Medium,
                "high" => Priority::High,
                _ => {
                    println!("Invalid priority, setting to \"Low\"");
                    Priority::Low
                }
            }
            Err(err) => {
                println!("Error getting user input: {}", err);
                return None;
            }
        };

        Some(Self::new(name, description, priority))
    }

    // This method simply prints task information
    fn print_task(&self) {
        println!(
            "{} | {} | {}\n\"{}\"\n",
            self.name,
            self.priority.to_string(),
            self.add_time.format("%d-%m-%Y %H:%M:%S"),
            self.description
        )
    }
}

struct TasksManager {
    // List of the added tasks
    tasks: Vec<Task>,
}

impl TasksManager {
    // This method simply creates new TasksManager object with empty vector field
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    // This method prints every added task info
    fn print_tasks(&self) {
        for task in &self.tasks {
            task.print_task();
        }
    }

    // This method adds given task to the tasks vector
    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    // This method searches for the task with given name in the added tasks vector
    fn find_task(&self, name: &str) -> Option<usize> {
        self.tasks.iter().position(|task| task.name == name)
    }

    // This method deletes task with the given task name from the added tasks vector
    fn remove_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            self.tasks.remove(index);
            Ok(format!("Task \"{}\" removed successfully", name))
        } else {
            Err(format!("Task with name \"{}\" doesn't exist", name))
        }
    }

    // This method searches for task with given name and updates its fields
    fn edit_task(&mut self, name: &str, updated_task: Task) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            match self.tasks.get_mut(index) {
                None => Err("Error fetching task".to_owned()),
                Some(task) => {
                    task.name = updated_task.name;
                    task.description = updated_task.description;
                    task.priority = updated_task.priority;

                    Ok(format!("Task \"{}\" updated successfully", name))
                }
            }
        } else {
            Err(format!("Task with name \"{}\" doesn't exist", name))
        }
    }

    // This method stores tasks list to the file in json format
    fn store_to_file(&self, filename: &str) -> Result<String, String> {
        if !Path::new(filename).exists() {
            let file = match File::create(filename) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error creating file: {}", err))
            };

            match serde_json::to_writer(&file, &self.tasks) {
                Ok(_) => Ok("Data stored successfully".to_owned()),
                Err(err) => Err(format!("Error saving data: {}", err))
            }
        } else {
            Err("File \"{filename}\" already exists".to_owned())
        }
    }

    // This method reads tasks list from the file (data must be in json format)
    fn read_from_file(&mut self, filename: &str) -> Result<String, String> {
        if Path::new(filename).exists() {
            let file = match File::open(filename) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error creating file: {}", err))
            };

            let reader = BufReader::new(file);

            self.tasks = match serde_json::from_reader(reader) {
                Ok(data) => data,
                Err(err) => {
                    return Err(format!("Error reading file: {}", err));
                }
            };

            Ok("Data read successfully".to_owned())
        } else {
            Err(format!("File \"{}\" doesn't exist", filename))
        }
    }
}

struct ConsoleManager {
    tasks_manager: TasksManager,
    menu_options: Vec<String>,
}

impl ConsoleManager {
    // This method simply creates new ConsoleManager object and fills options vector
    fn new() -> Self {
        Self {
            tasks_manager: TasksManager::new(),
            menu_options: vec![
                "Add task".to_owned(),
                "Find task".to_owned(),
                "Edit task".to_owned(),
                "Remove task".to_owned(),
                "Print tasks".to_owned(),
                "Store tasks to file".to_owned(),
                "Read tasks from file".to_owned(),
            ],
        }
    }

    // THis method prints menu of console manager
    fn print_menu(&self) {
        for (index, menu_option) in self.menu_options.iter().enumerate() {
            println!("{}. {}", index + 1, menu_option);
        }
    }

    // This function needed to make getting user input more convenient
    // query parameter is a text we printing to console when asking user for input
    fn input(query: &str) -> std::io::Result<String> {
        print!("{}", query);
        std::io::stdout().flush()?;

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }

    // This method gets command index from user
    // and executes needed operation with tasks list
    fn process_command(&mut self) {
        match Self::input("\nEnter command index: ") {
            Ok(command) => {
                match command.as_str() {
                    "1" => {
                        if let Some(task) = Task::new_from_console() {
                            self.tasks_manager.add_task(task);
                        }
                    }

                    "2" => {
                        let name = match Self::input("Enter task name to find: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };

                        match self.tasks_manager.find_task(name.as_str()) {
                            None => println!("Task with name \"{}\" doesn't exist", name),
                            Some(index) => {
                                println!("Task found!");

                                match self.tasks_manager.tasks.get(index) {
                                    None => println!("Error fetching task"),
                                    Some(task) => task.print_task()
                                }
                            }
                        }
                    }

                    "3" => {
                        let name = match Self::input("Enter task name to edit: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };

                        if let Some(task) = Task::new_from_console() {
                            match self.tasks_manager.edit_task(name.as_str(), task) {
                                Ok(msg) => println!("{}", msg),
                                Err(msg) => println!("{}", msg),
                            }
                        }
                    }

                    "4" => {
                        let name = match Self::input("Enter task name to remove: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };

                        match self.tasks_manager.remove_task(name.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg),
                        }
                    }

                    "5" => {
                        self.tasks_manager.print_tasks();
                    }

                    "6" => {
                        let filename = match Self::input("Enter file name to store data in: ") {
                            Ok(filename) => filename,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };

                        match self.tasks_manager.store_to_file(filename.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg),
                        }
                    }

                    "7" => {
                        let filename = match Self::input("Enter file name to read data from: ") {
                            Ok(filename) => filename,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };

                        match self.tasks_manager.read_from_file(filename.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg),
                        }
                    }

                    _ => println!("I don't understand this command :(")
                }
            }
            Err(err) => println!("Error getting user input: {}", err)
        }
    }
}

// Program entry point
fn main() {
    let mut manager = ConsoleManager::new();
    manager.print_menu();

    loop {
        manager.process_command();
    }
}
