use std::io::{self, Write, BufRead};
use std::fs::{File};

const FILE_NAME: &str = "tasks.txt";

#[derive(Debug, Clone)]
struct Task {
    description: String,
    completed: bool, 
}

fn main() {

    let mut tasks = load_tasks();

    println!("Welcome to To-Do List App!");

    loop{
        println!("\nTo-Do List:");
        view_tasks(&tasks);
        println!("\nOptions: (1) Add Task (2) Mark Complete (3) Remove Task  (4) Exit");

        print!("Enter a choice: ");

        io::stdout().flush().unwrap();
        
        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => add_task(&mut tasks),
            "2" => mark_task_complete(&mut tasks),
            "3" => remove_task(&mut tasks),
            "4" => {
                println!("Exiting....");
                break;
            },
            _ => println!("Invalid Choice!!")
        }
    }
}

fn save_tasks(tasks: &Vec<Task>){
    let mut file = File::create(FILE_NAME).expect("Unable to craete file.");

    for task in tasks{
        writeln!(file, "{};{}", task.description, task.completed).expect("Unable to write to file!");
    }
}

fn load_tasks() -> Vec<Task>{
    let mut tasks = Vec::new();

    if let Ok(file) = File::open(FILE_NAME){
        let reader = io::BufReader::new(file);

        for line in reader.lines(){
            if let Ok(entry) = line{
                let parts: Vec<&str> = entry.split(';').collect();
                if parts.len() == 2 {
                    let description = parts[0].to_string();
                    let completed = parts[1] == "true";
                    tasks.push(Task {description, completed});
                }
            }
        }
    }
    tasks
}

fn add_task(tasks: &mut Vec<Task>){
    print!("\nEnter a Task: ");

    io::stdout().flush().unwrap();

    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Failed to read task.");

    tasks.push(Task {
        description: task.trim().to_string(),
        completed: false,
    });

    save_tasks(tasks);
    println!("Task Added Successfully!");
}

fn view_tasks(tasks: &Vec<Task>){
    if tasks.is_empty(){
        println!("No tasks available!");
        return;
    } else{
        for (i, task) in tasks.iter().enumerate() {
            let status = if task.completed { "✓" } else { " " }; // Shows check mark for completed tasks
            println!("{}. [{}] {}", i + 1, status, task.description);
        }
    }
}

fn remove_task(tasks: &mut Vec<Task>){
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

fn mark_task_complete(tasks: &mut Vec<Task>){
    print!("Enter task number to mark as complete: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input!");

    match input.trim().parse::<usize>(){
        Ok(index) if index > 0 && index <= tasks.len() => {
            tasks[index - 1].completed = !tasks[index - 1].completed;
            save_tasks(tasks);
            println!("Task {} marked completed!", index);
        }
        _ => println!("Invalid task number!"),
    }
}