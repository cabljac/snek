
use std::time::{Duration, Instant};
use std::thread::sleep;
use device_query::{DeviceQuery, DeviceState, Keycode};


fn main() {
    run();
}

const GAME_IS_RUNNING : bool = true;
const FPS : u128 = 60;
const SKIP : u128 = 1000/FPS;

fn run() {
    let start = Instant::now();
    let mut frame_counter : u128 = 0;
    let mut pos : u32 = 4;
    while(GAME_IS_RUNNING) {
        let now = Instant::now();
        let last_pressed = get_device_state();
        pos = update_game(pos,last_pressed);
        display_game(pos);
        frame_counter += SKIP;
        if now.duration_since(start).as_millis() < frame_counter {
            sleep(Duration::new(0, (SKIP * 1000000).try_into().unwrap()));
        }
    }
}

fn get_device_state() -> Vec<Keycode> {
    let device_state = DeviceState::new();
    let keys: Vec<Keycode> = device_state.get_keys();
    keys
}

fn update_game(previous_state: u32, last_pressed: Vec<Keycode>) -> u32 {
    let mut pos : u32 = 0;
    if last_pressed.contains(&Keycode::LShift) {
        pos = 1
    }
    previous_state + pos
    // println!("update time: {}", now.duration_since(start).as_millis());
}

fn display_game(pos : u32) {
    clear_screen();
    print_grid();
    // print_border(true, 1, true);
    // print_body(pos);
    // print_border(true,1,false);
}
#[derive(Debug)]
enum CoordState {
    Empty,
    SnekHead,
    SnekBody,
    Mouse,
}
#[derive(Debug)]
struct Coords {
    x: u32,
    y: u32
}

struct Row {
    data: Vec<CoordState>
}

impl Row {
    fn display(&self) {
        let mut array_to_print = Vec::<&str>::new();

        for n in 0..self.data.len() {
            let coord_value = match self.data[n] {
                CoordState::Empty => " ",
                CoordState::Mouse => "M",
                CoordState::SnekBody => "*",
                CoordState::SnekHead => "S",
            };
            array_to_print.push(coord_value);
        }
        println!("|{:?}|",array_to_print.join(""));
    }
}

struct Grid {
    width: u32,
    height: u32,
    rows: Vec<Row>
}


enum KinkDirection {
    Left,
    Right
} 

struct Snek {
    head_position: Coords,
    length: u32,
    kinks: Vec<KinkDirection>
}

struct Mice {
    population: u32,
    positions: Vec<Coords>
}

impl Mice {
    fn birth (&mut self, pos: Coords) {
        self.population += 1;
        self.positions.push(pos)
    }
    fn death (&mut self) {
        self.population -= 1;
        self.positions.pop();
    }
}

struct GameState {
    snek: Snek,
    score: u32,
    mice: Mice
}

// TODO: display the empty grid

fn print_grid() {
    let row = Row { data: vec![CoordState::Empty,CoordState::Empty,CoordState::Empty,CoordState::Empty,CoordState::Empty,CoordState::SnekHead,CoordState::Empty,CoordState::Empty,CoordState::Empty,CoordState::Empty,CoordState::Empty] };

    row.display();
}



fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_border(empty: bool, _pos: u32,top: bool) {
    if top {
        print!("\n_________________________________________\n");
    }
    if empty {
        print!("|                                       |\n");
    } else {
        print!("|                  *                    |\n");
    }
    if !top {
        print!("|_______________________________________|\n");
    }
}

fn print_body(pos: u32) {
    for n in 1..pos {
        print!("|                                       |\n");
    }
    print!("|                  *                    |\n");
    for n in pos..15 {
        print!("|                                       |\n");
    }
}