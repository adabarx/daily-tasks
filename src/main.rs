#![allow(unused)]
use chrono::{DateTime, Duration, NaiveDate, Utc};
use rand::prelude::Distribution;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs::File, io::Read};
use uuid::Uuid;
use rand::Rng;
use rand::distributions::WeightedIndex;

const CYCLE: usize = 7; // days per week

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    // id: Uuid,
    name: String,
    weight: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct WorkDay {
    date: NaiveDate,
    tasks: Vec<Uuid>,
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

    println!("\nTasks Today:\n");
    for ch in the_chosen.iter() {
        println!("  - {}", ch.name);
    }
    println!();
}
