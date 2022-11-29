mod daily_tasks;

use crate::daily_tasks::*;

use anyhow::Result;
use serde_json;
use std::{
    fs,
    fs::File,
    io::Read,
};

// TODO: integrate with notion

fn import_json_file<T>(s: &str) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned
{
    let mut file = File::open(s)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let rv: Vec::<T> = serde_json::from_str(&contents)?;

    Ok(rv)
}

fn write_to_history_json(chosen: &Vec<Task>, mut history: Vec<WorkDay>) -> Result<()> {
    history.push(WorkDay::new(chosen.iter().map(|ch| ch.id).collect()));
    fs::write("history.json", serde_json::to_string_pretty(&history)?)?;

    Ok(())
}

fn main() -> Result<()> {
    let tasks: Vec<Task> = import_json_file("tasks.json")?;

    let history: Vec<WorkDay> = import_json_file("history.json")?;

    let last_cycle = tasks_last_cycle(&history);

    let num_today = num_tasks_today(&tasks, &last_cycle);

    let weights = generate_weights(&tasks, last_cycle);

    let the_chosen = the_choosening(tasks, weights, num_today);

    write_to_history_json(&the_chosen, history)?;

    println!("\nTasks Today:\n");
    for ch in the_chosen.iter() {
        println!("  - {}", ch.name);
    }
    println!();

    Ok(())
}
