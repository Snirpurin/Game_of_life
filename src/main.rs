use rand::*;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;

const GRID_SIZE: usize = 30;

#[derive(Clone, Copy)]
struct State {
    cycles: usize,
    counter: usize,
    old_states: [i32; 6],
    number_alive: i32,
    array: [[Cell; GRID_SIZE]; GRID_SIZE],
    start_array: [[Cell; GRID_SIZE]; GRID_SIZE],
}

#[derive(Clone, Copy)]
struct Cell {
    alive: bool,
    cell_around: i32,
}

struct GenVal<T>
where
    T: Fn(i32) -> i32,
{
    item: T,
}

impl<T> GenVal<T>
where
    T: Fn(i32) -> i32,
{
    fn new(some: T) -> GenVal<T> {
        GenVal { item: some }
    }
}
struct Data {
    check: bool,
    states: Vec<State>,
    counter: usize,
}

impl Data {
    fn new() -> Data{
        Data {check: false, states: vec!(), counter: 0}
    }
}

fn main() {
    let mut threads = vec![];
    let array = [1, 2, 3, 1];

    //Use Arc (Atomic reference counter) meaning: that we make the mutex which covers a T, Atomic so we can use that haev multiple owners
    //to the the mutex
    let data = Arc::new(Mutex::new(Data::new()));

    //let (tx, rx) = mpsc::channel();
    // || is the closure, which means that inside the closure we have the same state as the main has, and can therfor use items
    // that are in the main scope
    //closures can be saved to variables, but is not done here
    //ex: let clos = |T| -> T { code here};
    for i in 0..4 {
        //let thraw = i;
        //let sender = mpsc::Sender::clone(&tx);
        let data = Arc::clone(&data);
        threads.push(thread::spawn(move || {
            let mut count = 0;
            let mut check = false;
            println!("start of thread {}", i);
            let my_id = i;
            loop{
                println!("going {}", i);

                let mut list = vec!();
                println!("made list {}", i);
                for _ in 0..50 {
                    
                    list.push(worker());
                    count += 1;

                }

                {
                    println!("about to take lock {}", i);
                    let mut da = data.lock().expect("states in thread ");
                    println!("took lock {}", i);
                    da.counter += count;
                    count = 0;
                    check = da.check;
                    
                    for st in list {
                        da.states.push(st);
                    }
                }
                println!("da lifteime ended {}", i);
                
                if check {
                    println!("did break {}", i);
                    break;
                } 
                println!("did not break {}", i);
            }
            println!("out {}", i);
        }));
    }

    /*loop{
        states.push(rx.recv().unwrap());
        counter += 1;
        if counter == 40 {break}
    }*/

    //thread::sleep(std::time::Duration::new(2, 0));
    loop{
        println!("Press: \n 1: print cycle");
        println!("Press: \n 2: stop");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {}
            Err(j) => print!("Error happend"),
        }
        {
            println!("main is about to take lock");
            let mut da = data.lock().expect("states in thread");
            

            match input.trim().parse::<i32>().expect("sss") {
                1 => {println!("just pressed 1");
                    //print_arrays(&da.states);
                    println!("tries: {}", da.counter);
                    println!("just pressed 1");},
                2 => {da.check = true;
                    println!("tries: {}", da.counter);
                    break;},
                _ => println!("No more, now sleep"),
            }
        }
    }
    let mut c = 0;
    for thread in threads{
        println!("main is joining thread {}", c);
        thread.join().unwrap();
        c +=1;
    }    

}
fn print_arrays(states: &Vec<State>) {
    for stuf in states {
        println!("printuitn");
        for x in 1..(GRID_SIZE - 1) {
            for y in 1..(GRID_SIZE - 1) {
                if stuf.start_array[x][y].alive {
                    print!("1  ",);
                } else {
                    print!("0  ",);
                }
            }
            println!("");
        }
        println!("Cycles: {}", stuf.cycles - 5);
    }
}

fn worker() -> State {
    let mut state = random_init();
    state.start_array = state.array.clone();
    init_matrix(&mut state.array);


    //State control strop the threads hear by getting in a pattern tha does not stop
    while !state_coontrol(&mut state) {
        update_numbers(&mut state);
    }

    state
}

fn update_numbers(state: &mut State) {
    let clone = state.clone();
    for x in 1..(GRID_SIZE - 1) {
        for y in 1..(GRID_SIZE - 1) {
            match clone.array[x][y].alive {
                true => {
                    if clone.array[x][y].cell_around > 3 || clone.array[x][y].cell_around < 2 {
                        state.array[x][y].alive = false;
                        state.number_alive -= 1;
                        despawn(&mut state.array, x, y, 1);
                    }
                }
                false => {
                    if clone.array[x][y].cell_around == 3 {
                        state.array[x][y].alive = true;
                        state.number_alive += 1;
                        spawn(&mut state.array, x, y, 1);
                    }
                }
            }
        }
    }
}

fn state_coontrol(state: &mut State) -> bool {
    state.cycles += 1;
    if state.counter == 6 {
        state.counter = 0;
    }
    state.old_states[state.counter] = state.number_alive;
    state.counter += 1;

    let mut holder = state.old_states[0];
    for j in state.old_states.iter() {
        if holder != *j || *j == -1 {
            return false;
        }
        holder = *j;
    }
    return true;
}

fn spawn(array: &mut [[Cell; GRID_SIZE]; GRID_SIZE], x: usize, y: usize, diff: i32) {
    array[x + 1][y].cell_around += diff;
    array[x + 1][y + 1].cell_around += diff;
    array[x][y - 1].cell_around += diff;
    array[x][y + 1].cell_around += diff;
    array[x + 1][y - 1].cell_around += diff;
    array[x - 1][y].cell_around += diff;
    array[x - 1][y + 1].cell_around += diff;
    array[x - 1][y - 1].cell_around += diff;
}

fn despawn(array: &mut [[Cell; GRID_SIZE]; GRID_SIZE], x: usize, y: usize, diff: i32) {
    if array[x + 1][y].cell_around > 0 {
        array[x + 1][y].cell_around -= diff
    }
    if array[x + 1][y + 1].cell_around > 0 {
        array[x + 1][y + 1].cell_around -= diff
    }
    if array[x][y - 1].cell_around > 0 {
        array[x][y - 1].cell_around -= diff
    }
    if array[x][y + 1].cell_around > 0 {
        array[x][y + 1].cell_around -= diff
    }
    if array[x + 1][y - 1].cell_around > 0 {
        array[x + 1][y - 1].cell_around -= diff
    }
    if array[x - 1][y].cell_around > 0 {
        array[x - 1][y].cell_around -= diff
    }
    if array[x - 1][y + 1].cell_around > 0 {
        array[x - 1][y + 1].cell_around -= diff
    }
    if array[x - 1][y - 1].cell_around > 0 {
        array[x - 1][y - 1].cell_around -= diff
    }
}

fn init_matrix(array: &mut [[Cell; GRID_SIZE]; GRID_SIZE]) {
    for x in 1..(GRID_SIZE - 1) {
        for y in 1..(GRID_SIZE - 1) {
            if array[x + 1][y].alive {
                array[x][y].cell_around += 1
            }
            if array[x + 1][y + 1].alive {
                array[x][y].cell_around += 1
            }
            if array[x][y - 1].alive {
                array[x][y].cell_around += 1
            }
            if array[x][y + 1].alive {
                array[x][y].cell_around += 1
            }
            if array[x + 1][y - 1].alive {
                array[x][y].cell_around += 1
            }
            if array[x - 1][y].alive {
                array[x][y].cell_around += 1
            }
            if array[x - 1][y + 1].alive {
                array[x][y].cell_around += 1
            }
            if array[x - 1][y - 1].alive {
                array[x][y].cell_around += 1
            }
        }
    }
}

fn random_init() -> State {
    let mut state = State {
        number_alive: 0,
        array: [[Cell {
            alive: false,
            cell_around: 0,
        }; GRID_SIZE]; GRID_SIZE],
        old_states: [-1; 6],
        counter: 0,
        start_array: [[Cell {
            alive: false,
            cell_around: 0,
        }; GRID_SIZE]; GRID_SIZE],
        cycles: 0,
    };

    for x in (GRID_SIZE - 20)..(GRID_SIZE - 10) {
        for y in (GRID_SIZE - 20)..(GRID_SIZE - 10) {
            state.array[x][y].alive = rand::random() && rand::random();
            if state.array[x][y].alive {
                state.number_alive += 1;
            }
        }
    }
    return state;
}
