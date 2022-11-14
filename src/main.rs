#![allow(unused)]
use chrono::{DateTime, Duration, NaiveDate, Utc, Local};
use rand::prelude::Distribution;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs;
use std::io::Write;
use std::{fs::File, io::Read};
use uuid::Uuid;
use rand::Rng;
use rand::distributions::WeightedIndex;

const CYCLE: usize = 7; // days per week

// TODO: weigh chances by history
// TODO: integrate with notion

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: Uuid,
    name: String,
    weight: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct WorkDay {
    date: DateTime<Local>,
    tasks: Vec<Uuid>,
}

 impl WorkDay {
     fn new(tasks: Vec<Uuid>) -> Self {
         Self {
             date: chrono::offset::Local::now(),
             tasks,
         }
     }
 }


fn import_tasks() -> Vec<Task> {
    let mut file = File::open("tasks.json")
        .expect("failed to find file");
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let tasks: Vec<Task> = serde_json::from_str(&contents)
        .expect("Serde error");
    tasks
}

fn import_history() -> Vec<WorkDay> {
    let mut file = File::open("history.json")
        .expect("Can't find history file");
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let history: Vec<WorkDay> = serde_json::from_str(&contents)
        .expect("serde error");
    history
}

fn write_to_history_json(chosen: &Vec<Task>, mut history: Vec<WorkDay>) -> Result<()> {
    history.push(WorkDay::new(chosen.iter().map(|ch| ch.id).collect()));
    let j = serde_json::to_string_pretty(&history)
        .expect("serde error");
    fs::write("history.json", j);
    Ok(())
}

fn tasks_last_cycle(history: &Vec<WorkDay>) -> HashMap<Uuid, usize> {
    let date_minus_cycle = chrono::offset::Local::now() - Duration::days(CYCLE as i64);
    let history_last_cycle: Vec<&WorkDay> = history.iter()
        .filter(|day| day.date >= date_minus_cycle)
        .collect();

    let mut task_map: HashMap<Uuid, usize> = HashMap::new();
    for d in history_last_cycle.into_iter() {
        for t in d.tasks.iter() {
            task_map.entry(*t)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    task_map
}

fn the_choosening(mut tasks: Vec<Task>) -> Vec<Task> {
    //
    // The weights represent the number of times that
    // I have to complete each task in any given week.
    //
    // So the number of tasks that I need to do each
    // day becomes calculated as:
    //
    //     (sum of every tasks weight) / 7
    //
    //  we start by getting the weekly avg
    // 
    let weekly_avg: usize = tasks.iter()
        .map(|task| task.weight)
        .sum();
    let weekly_avg = (weekly_avg as f64) / (CYCLE as f64);

    //
    // split the weekly average into an integer and a remainder
    //
    let tasks_today = weekly_avg.floor();
    let remainder = weekly_avg - tasks_today;
    
    //
    // use the remainder to calculate how likely it is
    // to have to do an extra task today.
    //
    let mut rng = rand::thread_rng();
    let tasks_today: usize = if rng.gen_range(0.0..1.0) < remainder {
        tasks_today as usize + 1
    } else {
        tasks_today as usize
    };

    let mut the_chosen: Vec<Task> = Vec::new();
    for _ in 0..tasks_today {
        let weight_index = WeightedIndex::new(tasks.iter().map(|task| task.weight))
            .expect("WeightedIndex error");
        the_chosen.push(tasks.remove(weight_index.sample(&mut rng)));
    }

    the_chosen
}

fn main() {
    let tasks = import_tasks();
    let the_chosen = the_choosening(tasks);

    let mut history = import_history();
    println!("history: {:?}", history);
    println!();
    println!("history_last_cycle: {:?}", tasks_last_cycle(&history));
    write_to_history_json(&the_chosen, history);

    println!("\nTasks Today:\n");
    for ch in the_chosen.iter() {
        println!("  - {}", ch.name);
    }
    println!();
}
