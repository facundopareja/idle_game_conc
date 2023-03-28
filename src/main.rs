use std::cmp::{min};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use rand::{thread_rng, Rng};

const STARTING_GOLD: i32 = 10;
const STARTING_WOOD: i32 = 0;
const STARTING_STONE: i32 = 0;
const THREAD_COUNT: i32 = 5;
const WOOD_TO_GOLD: i32 = 3;
const STONE_TO_GOLD: i32 = 5;
const MAX_RANGE: i32 = 15;
const ADD: i32 = 1;
const SUBSTRACT: i32 = -1;

fn main() {
    let gold = Arc::new(Mutex::new(STARTING_GOLD));
    let wood = Arc::new(Mutex::new(STARTING_WOOD));
    let stone = Arc::new(Mutex::new(STARTING_STONE));
    let mut handles = Vec::with_capacity((5 * THREAD_COUNT) as usize);
    for _ in 0..THREAD_COUNT {
        handles.push(create_new_modifying_thread(gold.clone(), ADD));
        handles.push(create_new_modifying_thread(wood.clone(), SUBSTRACT));
        handles.push(create_new_modifying_thread(stone.clone(), SUBSTRACT));
        handles.push(create_new_converting_thread(gold.clone(), wood.clone(),stone.clone(),ADD));
        handles.push(create_new_converting_thread(gold.clone(), wood.clone(),stone.clone(), SUBSTRACT));
    }
    for handle in handles {
        handle.join().expect("Error en handle al realizar el join");
    }
}

fn create_new_modifying_thread(resource:Arc<Mutex<i32>>, modification: i32) -> JoinHandle<()> {
    return thread::spawn(move || {modify_resource(resource, modification)});
}

fn create_new_converting_thread(gold:Arc<Mutex<i32>>, wood:Arc<Mutex<i32>>, stone:Arc<Mutex<i32>>,
                                direction: i32) -> JoinHandle<()> {
    return thread::spawn(move || {convert_resource(gold, wood, stone, direction)});
}

fn modify_resource(resource:Arc<Mutex<i32>>, modifier: i32) {
    let mut rng = thread_rng();
    loop {
        {
            let mut resource_guard = resource.lock().expect("Error tomando lock en thread");
            if *resource_guard <= 0 && modifier == SUBSTRACT {
                break;
            }
            let mut resource_delta: i32 = rng.gen_range(0..=MAX_RANGE);
            if modifier == SUBSTRACT {
                resource_delta = rng.gen_range(0..=min(*resource_guard,MAX_RANGE));
            }
            *resource_guard += modifier * resource_delta ;
        }
        thread::sleep(Duration::from_secs(2));
    }
}

fn convert_resource(gold:Arc<Mutex<i32>>, wood:Arc<Mutex<i32>>, stone:Arc<Mutex<i32>>, direction: i32) {
    let mut rng = thread_rng();
    loop {
        {
            let mut gold_guard = gold.lock().expect("Error tomando lock en thread");
            if *gold_guard <= 0 && direction==ADD {
                break;
            }
            let gold_converted: i32 = rng.gen_range(0..=min(*gold_guard,MAX_RANGE));
            let mut wood_guard = wood.lock().expect("Error tomando lock en thread");
            let mut stone_guard = stone.lock().expect("Error tomando lock en thread");
            if (*wood_guard < *gold_guard*WOOD_TO_GOLD  || *stone_guard < *gold_guard*STONE_TO_GOLD) && direction==SUBSTRACT {
                break;
            }
            *gold_guard += -direction * gold_converted ;
            *wood_guard += direction * WOOD_TO_GOLD * gold_converted ;
            *stone_guard += direction * STONE_TO_GOLD * gold_converted ;
            println!("Ahora hay {} de oro, {} de madera y {} de piedra", *gold_guard, *wood_guard, *stone_guard);
        }
        thread::sleep(Duration::from_secs(2));
    }
}