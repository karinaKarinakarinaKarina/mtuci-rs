use std::io;

struct DoItem {
    id: u32, 
    name: String,
    completed: bool
}

fn main() {
    let mut tasks_list: Vec<DoItem> = Vec::new();

    loop {
        println!("Whatcha doing bro?");
        println!("1. Add a task bro");
        println!("2. Mate just completed the task");
        println!("3. Display all my business");
        println!("4. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("cant read the line");

        let choice = choice.trim().parse::<u32>().expect("cant read your input bro");

        match choice {
            1 => {
                println!("Enter the name of the to-do item:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();

                let id = tasks_list.len() as u32 + 1;

                let item = DoItem {
                    id,
                    name,
                    completed: false,
                };

                tasks_list.push(item);
            },
            2 => {
                println!("Enter the ID of the to-do item you want to complete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<u32>().expect("Invalid input");

                let item = tasks_list.iter_mut().find(|i| i.id == id).unwrap();
                complete_task(item);
            },
            3 => {
                display_items(&tasks_list);
            },
            4 => {
                println!("Goodbye! brooooo see ya again");
                return;
            },
            _ => {
                println!("Invalid choice");
            },
        }
    }
}


fn complete_task(task: &mut DoItem) {
    task.completed = true;
}


fn display_items(tasks: &Vec<DoItem>) {
    for task in tasks {
        println!("{} - {} ({})", task.id, task.name, task.completed);
    }
}