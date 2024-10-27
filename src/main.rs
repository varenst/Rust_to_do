use std::io::{self};

fn intro() {
    println!("# To-do application in Rust");
    println!("Available commands are:");
    println!("1. add <task> - Add a new task");
    println!("2. list - List all tasks");
    println!("3. remove <task_index> - Remove one or more tasks by their indices (space-separated)");
    println!("4. done <task_index> - Mark task as done");
    println!("5. undone <task_index> - Mark task as undone");
    println!("6. flush - Remove all tasks");
    println!("7. percentage - Show percentage of tasks done");
    println!("8. help - shows all available commands");
    println!("Type 'exit' to close the application.");
}

fn help() {
    println!("Available commands are:");
    println!("1. add <task> - Add a new task");
    println!("2. list - List all tasks");
    println!("3. remove <task_index> - Remove one or more tasks by their indices (space-separated)");
    println!("4. done <task_index> - Mark task as done");
    println!("5. undone <task_index> - Mark task as undone");
    println!("6. flush - Remove all tasks");
    println!("7. percentage - Show percentage of tasks done");
}

fn add_task(parts: &mut std::str::SplitWhitespace, tasks: &mut Vec<String>) {
    match parts.next() {
        Some(first) => {
            let mut task = String::new();

            task.push_str(first);
            
            while let Some(part) = parts.next() {
                task.push(' ');
                task.push_str(part);
            }
            
            println!("Added task: {}", task);

            tasks.push(task);
        },
        _ => println!("Please specify a task to add."),
    }
}

fn flush(tasks: &mut Vec<String>) {
    tasks.clear();
    println!("All tasks have been removed.");
}


fn done(parts: &mut std::str::SplitWhitespace, tasks: &mut Vec<String>) {
    let indices: Vec<usize> = parts
        .filter_map(|index_str| index_str.parse::<usize>().ok().map(|i| i - 1))
        .filter(|&i| i < tasks.len())
        .collect();

    for index in indices {
        if let Some(task) = tasks.get_mut(index) {
            *task = format!("{} [done]", task.trim_end());
            println!("Marked task as done: {}", task);
        }
    }
}

fn undone(parts: &mut std::str::SplitWhitespace, tasks: &mut Vec<String>) {
    let indices: Vec<usize> = parts
        .filter_map(|index_str| index_str.parse::<usize>().ok().map(|i| i - 1))
        .filter(|&i| i < tasks.len())
        .collect();

    for index in indices {
        if let Some(task) = tasks.get_mut(index) {
            *task = task.replace(" [done]", "");
            println!("Marked task as undone: {}", task);
        }
    }
}

fn percentage_done(tasks: &Vec<String>) {
    let total_tasks = tasks.len();
    let done_tasks = tasks.iter().filter(|task| task.ends_with(" [done]")).count();

    if total_tasks == 0 {
        println!("No tasks available to calculate percentage.");
    } else {
        let percentage = (done_tasks as f64 / total_tasks as f64) * 100.0;
        println!("Percentage of tasks done: {:.2}%", percentage);
    }
}


fn remove_task(parts: &mut std::str::SplitWhitespace, tasks: &mut Vec<String>) {
    let mut indices: Vec<usize> = Vec::new();

    for index_str in parts {
        match index_str.parse::<usize>() {
            Ok(index) if index > 0 && index <= tasks.len() => {
                indices.push(index - 1);
            },
            Ok(_) => println!("Invalid index: {} (out of range)", index_str),
            Err(_) => println!("Invalid index: {} (not a number)", index_str),
        }
    }

    if indices.is_empty() {
        println!("No valid tasks were specified for removal.");
        
        return;
    }

    indices.sort_unstable();
    indices.reverse();

    let mut removed_tasks: Vec<String> = Vec::new();

    for index in &indices {
        match tasks.get(*index) {
            Some(removed_task) => {
                removed_tasks.push(removed_task.clone());
            },
            None => println!("No task found at index: {}", index + 1),
        }
    }

    for index in indices {
        let removed_task = tasks.remove(index);
        println!("Removed task: {}", removed_task);
    }
}

fn list_tasks(tasks: &Vec<String>) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        println!("Current tasks:");
        for (index, task) in tasks.iter().enumerate() {
            println!("{}: {}", index + 1, task);
        }
    }
}                   

fn matchcommands(input : &str,tasks: &mut Vec<String>) {
    let lowercase_input: String = input.trim().to_lowercase();
    let mut parts = lowercase_input.split_whitespace();
    
    match parts.next() {
        Some("exit") => {
            print!("Exiting the application");
            std::process::exit(0);
        }
        Some("list") => list_tasks(tasks),
        Some("add") => add_task(&mut parts, tasks),
        Some("remove") => remove_task(&mut parts, tasks),
        Some("done") => done(&mut parts, tasks),
        Some("undone") => undone(&mut parts, tasks),
        Some("percentage") => percentage_done(tasks),
        Some("flush") => flush(tasks),
        Some("help") => help(), 
        Some(command) => println!("Command wasn't provided or doesn't exist: {}", command),
        None => println!("No command was inputted"),
    }
}

fn main() {
    intro();

    let mut tasks: Vec<String> = Vec::new();

    loop {
        println!("Ente a command: ");
        
        let mut line = String::new();

        io::stdin().read_line(&mut line).expect("Failed to read line");

        matchcommands(&line, &mut tasks);
    }
}