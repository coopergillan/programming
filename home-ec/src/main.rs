//!
//! Home Economics
//!

use rand::prelude::*;

use std::collections::VecDeque;
// use std::fmt;
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug)]
struct HouseholdTask {
    name: String,
    duration: Duration,
}

fn main() {
    println!("Hello, world! Here are some household tasks!");

    let mut queue = VecDeque::<HouseholdTask>::new();
    println!("There are {:?} tasks in the queue", queue);

    let breakfast = HouseholdTask {
        name: "Make Breakfast".to_string(),
        duration: Duration::from_secs(30),
    };
    queue.push_back(breakfast);
    println!("Queue contains: {:?}", queue);

    let brush_teeth = HouseholdTask {
        name: "Brush Teeth".to_string(),
        duration: Duration::from_secs(120),
    };
    queue.push_back(brush_teeth);
    println!("Queue contains: {:?}", queue);

    let clean_up = HouseholdTask {
        name: "Clean up".to_string(),
        duration: Duration::from_secs(300),
    };
    queue.push_back(clean_up);
    println!("Queue contains: {:?}", queue);

    let random_urgent_tasks = vec![
        HouseholdTask {
            name: "Answer the door!".to_string(),
            duration: Duration::from_secs(15),
        },
        HouseholdTask {
            name: "Swat a fly!".to_string(),
            duration: Duration::from_secs(20),
        },
        HouseholdTask {
            name: "Pick up toy on floor that you could step on".to_string(),
            duration: Duration::from_secs(10),
        },
    ];

    while let Some(task) = queue.pop_front() {
        println!("\nAbout to start task {:?}", task.name);
        thread::sleep(task.duration / 10);

        println!(
            "Completed task {:?} in {:?} seconds\n",
            task.name, task.duration
        );

        // Pick a random number and if it's seven, add another task
        let mut rng = rand::rng();
        if rng.random_range(0..10) == 7 {
            println!("Suddenly another urgent task appears!");
            let new_task = random_urgent_tasks.choose(&mut rng).unwrap();
            queue.push_front(new_task.clone());
            println!("Added new task {:?}", new_task.name);
        }

        println!("Tasks remaining: {:?}\n", queue);
    }
}
