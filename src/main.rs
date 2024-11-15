use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Add a task
    /// on multiple lines
    #[arg(short, long)]
    add: Option<String>,

    /// Remove a task
    #[arg(short, long)]
    remove: Option<String>,

    /// List all tasks
    #[arg(short, long)]
    list: bool,
}

fn main() {
    println!("This is a simple CLI app");

    let args = Args::parse();

    // Add task to list
    let mut tasks = Vec::new();
    match args.add {
        Some(task) => {
            tasks.push(task);
            println!("Task added");
        }
        None => println!("No task added"),
    }
}
