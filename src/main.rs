use rand::*;
use std::thread;
use std::io;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const GRID_SIZE: usize = 30;

#[derive(Clone, Copy)]
struct State {
    cycles: usize,
    counter: usize,
    old_states: [i32; 6],
    number_alive: i32,
    array: [[Cell;GRID_SIZE];GRID_SIZE],
    start_array: [[Cell;GRID_SIZE];GRID_SIZE],

}


#[derive(Clone, Copy)]
struct Cell {
    alive: bool,
    cell_around: i32,
}



fn main() {
    let mut threads = vec![];

  for _ in 0..4{
      threads.push(thread::spawn(move || -> State {
          worker()
      }));
  }
  let mut states = vec![];
    for thread in threads {
        let holder = thread.join().unwrap();
        states.push(holder);
    } 


    println!("Press: \n 1: print cycle");
let mut input = String::new();
match io::stdin().read_line(&mut input){
    Ok(n) =>{},
    Err(j) => print!("Error happend"),
}
 match input.trim().parse::<i32>().expect("sss") {
     1 => print_arrays(&states),
     _=> println!("No more, now sleep"),

 }


}
fn print_arrays(states: & Vec<State>) {
    for stuf in states{
        for x in 1..(GRID_SIZE - 1){
            for y in 1..(GRID_SIZE - 1){
                if stuf.start_array[x][y].alive{
                    print!("1  ",);
                }
                else{
                    print!("0  ",);
                }
            }
            println!("");
            }
            println!("Cycles: {}", stuf.cycles - 5);
        }
}

fn worker() -> State{
    let mut state = random_init();
    state.start_array = state.array.clone();
    init_matrix(&mut state.array);

    while !state_coontrol(&mut state) {
        update_numbers(&mut state);
    }

    state


}

fn update_numbers(state: &mut State){
    let clone = state.clone();
    for x in 1..(GRID_SIZE - 1){
        for y in 1..(GRID_SIZE - 1){
            match clone.array[x][y].alive{
                true => {if clone.array[x][y].cell_around > 3 || clone.array[x][y].cell_around < 2 {
                        state.array[x][y].alive = false;
                        state.number_alive -=1;
                        despawn(&mut state.array, x, y, 1);
                        }
                    },
                false => {if clone.array[x][y].cell_around == 3 {
                    state.array[x][y].alive = true;
                    state.number_alive +=1;
                    spawn(&mut state.array, x, y, 1);
                        }
                    },
            }
            
        }
    }
}

fn state_coontrol(state: &mut State)-> bool{
    state.cycles += 1;
      if state.counter == 6 {
        state.counter = 0;
      }
      state.old_states[state.counter] = state.number_alive;
      state.counter += 1;

    let mut holder = state.old_states[0];
    for j in state.old_states.iter(){
        if holder != *j || *j == -1{
            return false;
        }
        holder = *j;
    }
    return true;
}

fn spawn(array: &mut [[Cell;GRID_SIZE];GRID_SIZE], x:usize ,y: usize, diff: i32){
    array[x + 1][y].cell_around +=diff;
    array[x + 1][y + 1].cell_around +=diff;
    array[x][y - 1].cell_around +=diff;
    array[x][y + 1].cell_around +=diff;
    array[x + 1][y - 1].cell_around +=diff;
    array[x - 1][y].cell_around +=diff;
    array[x - 1][y + 1].cell_around +=diff;
    array[x - 1][y - 1].cell_around +=diff;
}

fn despawn(array: &mut [[Cell;GRID_SIZE];GRID_SIZE], x:usize ,y: usize, diff: i32){
if array[x + 1][y].cell_around > 0 {array[x + 1][y].cell_around -=diff}
if array[x + 1][y + 1].cell_around > 0{array[x + 1][y + 1].cell_around -=diff}
if array[x][y - 1].cell_around > 0{array[x][y - 1].cell_around -=diff}
if array[x][y + 1].cell_around > 0{array[x][y + 1].cell_around -=diff}
if array[x + 1][y - 1].cell_around > 0{array[x + 1][y - 1].cell_around -=diff}
if array[x - 1][y].cell_around > 0{array[x - 1][y].cell_around -=diff}
if array[x - 1][y + 1].cell_around > 0{array[x - 1][y + 1].cell_around -=diff}
if array[x - 1][y - 1].cell_around > 0{array[x - 1][y - 1].cell_around -=diff}
}

fn init_matrix(array: &mut  [[Cell;GRID_SIZE];GRID_SIZE]){
    for x in 1..(GRID_SIZE - 1){
        for y in 1..(GRID_SIZE - 1){
            
                if array[x + 1][y].alive{array[x][y].cell_around +=1}
                if array[x + 1][y + 1].alive{array[x][y].cell_around +=1}
                if array[x][y - 1].alive{array[x][y].cell_around +=1}
                if array[x][y + 1].alive{array[x][y].cell_around +=1}
                if array[x + 1][y - 1].alive{array[x][y].cell_around +=1}
                if array[x - 1][y].alive {array[x][y].cell_around +=1}
                if array[x - 1][y + 1].alive{array[x][y].cell_around +=1}
                if array[x - 1][y - 1].alive{array[x][y].cell_around +=1}
            
        }
    }
}

fn random_init() -> State {
    let mut state = State {number_alive: 0,
        array: [[Cell {alive: false, cell_around: 0};GRID_SIZE];GRID_SIZE],
        old_states: [-1;6],
        counter: 0,
        start_array: [[Cell {alive: false, cell_around: 0};GRID_SIZE];GRID_SIZE],
        cycles: 0,
    };

    for x in (GRID_SIZE- 20)..(GRID_SIZE-10){
        for y in (GRID_SIZE- 20)..(GRID_SIZE-10){
            state.array[x][y].alive = rand::random();
            if state.array[x][y].alive {
                state.number_alive += 1;
            }
        }
    }
    return state;

}