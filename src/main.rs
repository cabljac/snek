use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    run();
}

const GAME_IS_RUNNING: bool = true;
const FPS: u128 = 60;
const SKIP: u128 = 1000 / FPS;

fn run() {
    let start = Instant::now();
    let mut frame_counter: u128 = 0;
    let mut pos: u32 = 4;
    while (GAME_IS_RUNNING) {
        let now = Instant::now();
        let last_pressed = get_device_state();
        pos = update_game(pos, last_pressed);
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
    let mut pos: u32 = 0;
    if last_pressed.contains(&Keycode::LShift) {
        pos = 1
    }
    previous_state + pos
    // println!("update time: {}", now.duration_since(start).as_millis());
}

fn display_game(pos: u32) {
    clear_screen();
    print_grid();
    print_border(true, 1, true);
    print_body(pos);
    print_border(true, 1, false);
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
    y: u32,
}

struct Row {
    length: u32,
    has_snek_head: bool,
    has_mice: bool,
    snek_head_position: u32,
    mice_positions: Vec<u32>,
}

type RowState = Vec<CoordState>;

// TODO 1. fix ownership and types and lifetime of Row:

impl Row {
    fn build_state(&self) -> RowState {
        let mut state = RowState::new();
        for n in 0..self.length {
            let mut coord_value: CoordState = CoordState::Empty;
            if self.has_mice {
                if self.mice_positions.contains(&n) {
                    coord_value = CoordState::Mouse
                }
            }
            if self.has_snek_head {
                if n == self.snek_head_position {
                    coord_value = CoordState::SnekHead
                }
            }
            state.push(coord_value);
        }
        state
    }
    fn display(&self) {
        let state = self.build_state();
        let to_print = state
            .iter()
            .map(|s| match s {
                CoordState::Empty => " ",
                CoordState::Mouse => "M",
                CoordState::SnekBody => "*",
                CoordState::SnekHead => "S",
            })
            .collect::<Vec<_>>()
            .join("");
        println!("{:#?}", to_print);
    }
}

struct Grid {
    width: u32,
    height: u32,
    rows: Vec<Row>,
}

enum KinkDirection {
    Left,
    Right,
}

struct Snek {
    head_position: Coords,
    length: u32,
    kinks: Vec<KinkDirection>,
}

struct Mice {
    population: u32,
    positions: Vec<Coords>,
}

impl Mice {
    fn birth(&mut self, pos: Coords) {
        self.population += 1;
        self.positions.push(pos)
    }
    fn death(&mut self) {
        self.population -= 1;
        self.positions.pop();
    }
}

struct GameState {
    snek: Snek,
    score: u32,
    mice: Mice,
}

// TODO 3: handle user input for stop/start game
// TODO 4: handle user input for moving snake head about
// 5: handle snake eating mouse
// 6: handle score
// 7: handle snake getting longer
// For snake body could just remember history of where snake has been (prev user inputs) - would make snake death easier to handle? not sure

fn print_grid() {
    let row = Row {
        length: 10,
        has_snek_head: false,
        has_mice: false,
        snek_head_position: 0,
        mice_positions: vec![0],
    };

    row.display();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_border(empty: bool, _pos: u32, top: bool) {
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
