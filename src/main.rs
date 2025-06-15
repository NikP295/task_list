use std::io;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::env;

fn main() {
    let _example_string:String = String::from("XTask1: do stuff fr&OTask2: do OTHER stuff fr&XTask3: The fitness gram pacer test is a multi-stage aerobics test...");
    
    // open the file where the tasks are stored
    let file_path = get_data_file_path("tasklist.txt");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)  // create if it doesn't exist
        .open(&file_path);

    let mut contents = String::new();
    file.expect("File opening failed! (While reading)").read_to_string(&mut contents).expect("Failed to read data from file!");
    contents = String::from(contents.trim());

    let list_tuple = read_format(contents.clone());
    
    let mut tasks:Vec<String> = list_tuple.0;
    let mut completion:Vec<bool> = list_tuple.1;
    
    let mut choice:String;
    let mut curr_task_num:usize;

    let mut new_line_boone:bool = true;


    // main menu options and user input handling
    loop {
        println!("Tasks: ");
        list_all_tasks(tasks.clone(), completion.clone(), new_line_boone.clone());
        println!("----------------------------------------------------------");
        println!("What do you want to do?");
        println!("'q' = quit, 'c' = switch format option, 'a' = add a task, 'r' = remove a task, 'm' = mark a task as completed or to do");
        choice = user_interaction_io("Your choice: ");
        
        // ADD
        if choice == String::from("a") {
            choice = user_interaction_io("Write the short task description: ");
            
            curr_task_num = tasks.len() + 1;
            choice = format!("Task{}: {}", curr_task_num.to_string(), choice);

            tasks.push(choice);
            completion.push(false);
        }

        // REMOVE
        else if choice == String::from("r") {
            choice = user_interaction_io("Remove task (write the number): ");
            curr_task_num = choice.parse::<usize>().unwrap() - 1;
            
            if curr_task_num >= tasks.len() {
                println!("--------------------------------------------------");
                println!("There is no task number {}, didnt change anything!", curr_task_num + 1);
                println!("--------------------------------------------------");
                
                continue;
            }

            tasks.remove(curr_task_num);
            completion.remove(curr_task_num);
            
            // changes the numbers of the tasks after the removed one to match new numbering
            for x in 0..tasks.len() {
                if x >= curr_task_num {
                    tasks[x].replace_range(0..5, "");
                    tasks[x] = format!("Task{}{}", (x + 1).to_string(), tasks[x]);
                }
            }
        }
        
        // MARK
        else if choice == String::from("m") {
            println!("marking a task!");
            choice = user_interaction_io("Write the task you want to toggle Complete / to do: ");

            if choice == String::from("all") {
                for i in 0..completion.len() {
                    completion[i] = false;
                }
            }
            else {
                curr_task_num = choice.parse::<usize>().unwrap() - 1;
                completion[curr_task_num] = !completion[curr_task_num]
            }
        }

        // display tasks in one or individual lines toggle
        else if choice == String::from("c") {
            new_line_boone = !new_line_boone;
        }

        // QUIT
        else if choice == String::from("q") {
            println!("quiting");
            break;
        }
        
        else {
            println!("unknown option!");
        }
    }
    
    // write the newly made string to the file for the next time the program is opened
    let writable_string:String = write_format(tasks, completion);

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path);


    file.expect("File opening failed! (While writing)").write_all(writable_string.as_bytes()).expect("Failed to either write data to file!");
}


// found online, didnt write all this myself
fn get_data_file_path(filename: &str) -> PathBuf {
    // XDG_DATA_HOME or default to ~/.local/share
    let base_dir = env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut home = dirs::home_dir().expect("Could not get home dir");
            home.push(".local/share");
            home
        });

    let mut path = base_dir;
    path.push("task_list"); // your app name
    std::fs::create_dir_all(&path).expect("Failed to create data directory");
    path.push(filename);

    return path;
}


fn list_all_tasks(the_list:Vec<String>, the_completion:Vec<bool>, print_new_line:bool) {
    for i in 0..the_list.len() {
        if the_completion[i] {
            print!("[X] ");
        }
        else {
            print!("[ ] ");
        }

        print!("{} \t", the_list[i]);

        if print_new_line {
            println!("");
        }

    }
    println!("");
}


fn user_interaction_io(question:&str) -> String {

    println!("{}", question);

    let mut user_typed:String = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_typed).expect("Failed to read user input!");

    user_typed = String::from(user_typed.trim());
    
    return user_typed;
}




fn read_format(input_string:String) -> (Vec<String>, Vec<bool>) {

    let mut list_of_tasks:Vec<String> = Vec::<String>::new();
    let mut list_of_task_completion:Vec<bool> = Vec::<bool>::new();
    
    let mut task:String = String::new();
    
    let mut last_task_forgotten:bool = true;
    let mut first_letter_boone:bool = true;
    
    if input_string == String::from("") {
        // first run
        list_of_tasks.push(String::from("Task1: Write your tasks!"));
        list_of_task_completion.push(false);

        return (list_of_tasks, list_of_task_completion)
    }

    for i in input_string.chars() {
        if i == '&' {
            list_of_tasks.push(task);
            task = String::from("");
            first_letter_boone = true;
        }

        else if first_letter_boone {
            first_letter_boone = false;

            if i == 'X' {
                list_of_task_completion.push(true); 
            }

            else {
                list_of_task_completion.push(false);
            }
        }

        else {
            task.push(i);
        }
    }
    
    if task == String::from("") {
        last_task_forgotten = false;
    }
    
    if last_task_forgotten {
        list_of_tasks.push(task);
    }

    return (list_of_tasks, list_of_task_completion)
}

fn write_format(write_list_tasks: Vec<String>, write_list_tasks_completion: Vec<bool>) -> String {
    let mut res:String = String::new();

    if write_list_tasks.len() != write_list_tasks_completion.len() {
        panic!("Task list does not match completion list in length!");
    }

    let write_length_of_tasks = write_list_tasks.len();

    for i in 0..write_length_of_tasks {
        if write_list_tasks_completion[i]{
            res = format!("{}{}", res, 'X');
        }

        else {
            res = format!("{}{}", res, 'O');
        }

        res.push_str(&write_list_tasks[i]);

        if i != (write_length_of_tasks - 1) {
            res = format!("{}{}", res, '&');
        }
    }

    return res;
}















