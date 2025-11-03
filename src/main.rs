mod task;
mod storage;
mod commands;

use std::{u8};

use clap::{Parser, Subcommand};
use storage::{load_tasks};

extern crate colored;


use todo_cil::priority_color;

use crate::commands::{add, done,delete,list};

#[derive(Parser)]
#[command(name = "TODO")]
#[command(about = "This is a simple command-line task manager built with Rust")]
//--help


struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String,priority:Option<u8> },
    List,
    Done { id: u32 },
    Delete { id: u32 },
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { description ,priority} => {
            add(&mut tasks,description,priority);
        }


        Commands::List=>{
            list(&mut tasks);
        }


        Commands::Done { id } => {
            done(&mut tasks, id);
        }

        Commands::Delete { id } => {
            delete(&mut tasks, id);
        }
    }

}




