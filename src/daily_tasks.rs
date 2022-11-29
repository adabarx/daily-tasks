use std::collections::HashMap;
use chrono::{Local, DateTime, Duration, Timelike};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use rand::{
    prelude::Distribution,
    distributions::WeightedIndex,
    Rng,
};

const CYCLE: usize = 7;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub weight: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkDay {
    pub date: DateTime<Local>,
    pub tasks: Vec<Uuid>,
}

impl WorkDay {
    pub fn new(tasks: Vec<Uuid>) -> Self {
        Self {
            date: Local::now(),
            tasks,
        }
    }
}
    

pub fn tasks_last_cycle(history: &Vec<WorkDay>) -> HashMap<Uuid, usize> {
    let date_minus_cycle = Local::now() - (Duration::seconds((CYCLE as i64 * 24 * 60 * 60) +
                                                             (Local::now().hour() as i64 * 60 * 60) +
                                                             (Local::now().minute() as i64 * 60) +
                                                             (Local::now().second() as i64))); // midnight one cycle ago
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

pub fn num_tasks_today(tasks: &Vec<Task>, last_cycle: &HashMap<Uuid, usize>) -> usize {
    let target_avg: f64 = tasks.iter()
        .map(|task| task.weight)
        .sum();
    let current_avg: f64 = last_cycle.values()
        .map(|v| *v as f64)
        .sum();

    let target_avg = target_avg / CYCLE as f64;
    let current_avg = current_avg / CYCLE as f64;

    let  mult = if current_avg == 0.0 {
        target_avg * target_avg
    } else {
        target_avg / current_avg
    };

    if target_avg * mult > tasks.len() as f64 {
        return tasks.len();
    }

    let mut rng = rand::thread_rng();
    let mut rv = target_avg * mult;
    let remainder = rv - rv.floor();
    if rng.gen_range(0.0..1.0) > remainder {
        rv += 1.0;
    }
    rv.floor() as usize
}

pub fn generate_weights(tasks: &Vec<Task>, last_cycle: HashMap<Uuid, usize>) -> HashMap<Uuid, usize> {
    tasks.iter()
        .map(|task| {
            let base = match last_cycle.get(&task.id) {
                Some(weight) => task.weight - (task.weight - *weight as f64),
                None => task.weight * 2.0,
            };
            let mult = match last_cycle.get(&task.id) {
                Some(weight) => task.weight / *weight as f64,
                None => task.weight * 2.0,
            };
            (task.id, (base * mult).round() as usize)
        })
        .collect()
}

pub fn the_choosening(mut tasks: Vec<Task>, weights: HashMap<Uuid, usize>, num_tasks: usize) -> Vec<Task> {
    let mut rng = rand::thread_rng();

    let mut the_chosen: Vec<Task> = Vec::new();
    for _ in 0..num_tasks {
        let weight_index = WeightedIndex::new(tasks.iter().map(|task| weights[&task.id]))
            .expect("WeightedIndex error");
        the_chosen.push(tasks.remove(weight_index.sample(&mut rng)));
    }
    the_chosen
}
