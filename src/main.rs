use std::io::{self, Write, Read, BufRead};
use std::fs::{File, OpenOptions};

const FILE_NAME: &str = "tasks.txt";

fn main() {

    let mut tasks = load_tasks();

    println!("Welcome to To-Do List App!");

    loop{
        println!("\nTo-Do List:");
        view_tasks(&tasks);
        println!("\nOptions: (1) Add Task  (2) Remove Task  (3) Exit");

        print!("Enter a choice: ");

        io::stdout().flush().unwrap();
        
        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => add_task(&mut tasks),
            "2" => remove_task(&mut tasks),
            "3" => {
                println!("Exiting....");
                break;
            },
            _ => println!("Invalid Choice!!")
        }
    }
}

fn save_tasks(tasks: &Vec<String>){
    let mut file = File::create(FILE_NAME).expect("Unable to craete file.");

    for task in tasks{
        writeln!(file, "{}", task).expect("Unable to write to file!");
    }
}

fn load_tasks() -> Vec<String>{
    let mut tasks = Vec::new();

    if let Ok(file) = File::open(FILE_NAME){
        let reader = io::BufReader::new(file);

        for line in reader.lines(){
            if let Ok(task) = line{
                tasks.push(task);
            }
        }
    }
    tasks
}

fn add_task(tasks: &mut Vec<String>){
    print!("\nEnter a Task: ");

    io::stdout().flush().unwrap();

    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Failed to read task.");

    let task = task.trim().to_string();

    if task.is_empty(){
        println!("Task cannot be empty!");
    } else{
        tasks.push(task);
        save_tasks(tasks);
        println!("Task Added Successfully!");
    }
}

fn view_tasks(tasks: &Vec<String>){
    if tasks.is_empty(){
        println!("No tasks available!");
    } else{
        println!("\nYour Tasks:");
        for (index, task) in tasks.iter().enumerate(){
            println!("{}. {}", index+1, task);
        }
    }
}

fn remove_task(tasks: &mut Vec<String>){
    if tasks.is_empty(){
        println!("No tasks to remove.");
    }

    view_tasks(tasks);
    print!("Enter the task number to remove: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim().parse::<usize>(){
        Ok(index) if index > 0 && index <= tasks.len() => {
            tasks.remove(index - 1);
            save_tasks(tasks);
            println!("Task {} removed successfully!", index);
        }
        _ => println!("Invalid task number!"),
    }
}