// src/bin/a.rs
use proconio::input;
use rand::prelude::*;
use rand_pcg::Pcg64;
use rustc_hash::FxHashSet;

const N: usize = 30;
const M: usize = 10; // number of robots
//const K: usize = 10; // number of buttons
const MAX_OPERATIONS: usize = 2 * N * N;
const TIME_LIMIT: f64 = 2.8; // seconds

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Stay = 4,
}

impl Action {
    const ALL: [Action; 5] = [Action::Up, Action::Down, Action::Left, Action::Right, Action::Stay];
    
    fn to_char(self) -> char {
        match self {
            Action::Up => 'U',
            Action::Down => 'D',
            Action::Left => 'L',
            Action::Right => 'R',
            Action::Stay => 'S',
        }
    }

    fn get_delta(self) -> (i32, i32) {
        match self {
            Action::Up => (-1, 0),
            Action::Down => (1, 0),
            Action::Left => (0, -1),
            Action::Right => (0, 1),
            Action::Stay => (0, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Robot {
    id: usize,
    start_pos: (usize, usize),
    current_pos: (usize, usize),
}

#[derive(Debug, Clone)]
struct Grid {
    n: usize,
    vertical_walls: Vec<Vec<bool>>,   // walls between columns
    horizontal_walls: Vec<Vec<bool>>, // walls between rows
}

impl Grid {
    fn new(n: usize) -> Self {
        Self {
            n,
            vertical_walls: vec![vec![false; n.saturating_sub(1)]; n],
            horizontal_walls: vec![vec![false; n]; n.saturating_sub(1)],
        }
    }

    fn has_wall_between(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let (fr, fc) = from;
        let (tr, tc) = to;

        if fr == tr && fc != tc {
            // Horizontal movement
            let min_col = fc.min(tc);
            min_col < self.n.saturating_sub(1) && self.vertical_walls[fr][min_col]
        } else if fc == tc && fr != tr {
            // Vertical movement
            let min_row = fr.min(tr);
            min_row < self.n.saturating_sub(1) && self.horizontal_walls[min_row][fc]
        } else {
            false
        }
    }

    fn can_move(&self, from: (usize, usize), action: Action) -> bool {
        if action == Action::Stay {
            return true;
        }

        let (new_row, new_col) = get_new_position(from, action);

        if new_row < 0 || new_row >= self.n as i32 || new_col < 0 || new_col >= self.n as i32 {
            return false;
        }

        let to = (new_row as usize, new_col as usize);
        !self.has_wall_between(from, to)
    }

    fn apply_move(&self, from: (usize, usize), action: Action) -> (usize, usize) {
        if !self.can_move(from, action) {
            return from;
        }

        let (row, col) = from;
        let (dr, dc) = action.get_delta();
        ((row as i32 + dr) as usize, (col as i32 + dc) as usize)
    }
}

fn get_new_position(from: (usize, usize), action: Action) -> (i32, i32) {
    let (row, col) = from;
    let (dr, dc) = action.get_delta();

    (row as i32 + dr, col as i32 + dc)
}

fn is_waxed(from: (usize, usize), action: Action, waxed: &FxHashSet<(usize, usize)>) -> bool {
    if action != Action::Right && action != Action::Left {
        return true;
    }

    let mut fromu = (from.0, from.1);
    let mut fromi = (from.0 as i32, from.1 as i32);
    loop {
        fromi = get_new_position(fromu, action);
        if fromi.1 < 0 || 29 < fromi.1 {
            return true;
        }
        fromu = (fromi.1 as usize, fromi.1 as usize);
        if waxed.contains(&fromu) {
            return false;
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    robots: Vec<Robot>,
    waxed: FxHashSet<(usize, usize)>,
    operations: Vec<usize>,
}

impl State {
    fn new(robot_positions: Vec<(usize, usize)>) -> Self {
        let robots = robot_positions
            .into_iter()
            .enumerate()
            .map(|(id, pos)| Robot {
                id,
                start_pos: pos,
                current_pos: pos,
            })
            .collect();

        let mut waxed: FxHashSet<(usize, usize)> = FxHashSet::default();
        for robot in &robots {
            waxed.insert((robot as &Robot).current_pos);
        }

        State {
            robots,
            waxed,
            operations: Vec::new(),
        }
    }

    fn apply_button(&mut self, grid: &Grid, button_config: &[Action; M]) {
        for (i, robot) in self.robots.iter_mut().enumerate() {
            let action = button_config[i];
            robot.current_pos = grid.apply_move(robot.current_pos, action);
            self.waxed.insert((robot as &Robot).current_pos);
        }
    }

    fn calculate_score(&self) -> i64 {
        let total_cells = N * N;
        let unwaxed_cells = total_cells - self.waxed.len();
        let operations_count = self.operations.len();

        if unwaxed_cells == 0 {
            (3 * total_cells) as i64 - operations_count as i64
        } else {
            total_cells as i64 - unwaxed_cells as i64
        }
    }

    fn coverage_ratio(&self) -> f64 {
        self.waxed.len() as f64 / (N * N) as f64
    }

    fn reset_to_initial(&mut self) {
        self.waxed.clear();
        for robot in self.robots.iter_mut() {
            robot.current_pos = robot.start_pos;
            self.waxed.insert((robot as &Robot).current_pos);
        }
    }
}

#[derive(Debug, Clone)]
struct Timer {
    start_time: std::time::Instant,
}

impl Timer {
    fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }

    fn elapsed(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    fn is_time_over(&self, limit: f64) -> bool {
        self.elapsed() > limit
    }
}

#[derive(Debug)]
struct Solver {
    grid: Grid,
    button_configs: Vec<[Action; M]>,
    rng: Pcg64,
    timer: Timer,
}

impl Solver {
    fn new(grid: Grid, seed: u64, timer: Timer) -> Self {
        Self {
            grid,
            button_configs: Vec::new(),
            rng: Pcg64::seed_from_u64(seed),
            timer,
        }
    }

    fn generate_smart_button_configs(&mut self) -> Vec<[Action; M]> {
        let mut configs = Vec::new();

        // Basic movements - all robots move in same direction
        configs.push([Action::Right; M]);
        configs.push([Action::Left; M]);
        configs.push([Action::Down; M]);
        configs.push([Action::Up; M]);

        let mut alt_h = [Action::Stay; M];
        for i in 0..M {
            alt_h[i] = if i % 2 == 0 { Action::Up } else { Action::Down };
        }
        configs.push(alt_h);

        // Alternating patterns
        let mut alt_h = [Action::Stay; M];
        for i in 0..M {
            alt_h[i] = if i % 2 == 0 { Action::Right } else { Action::Left };
        }
        configs.push(alt_h);

        let mut alt_v = [Action::Stay; M];
        for i in 0..M {
            alt_v[i] = if i % 2 == 0 { Action::Down } else { Action::Up };
        }
        configs.push(alt_v);

        // Dispersal patterns
        let mut dispersal1 = [Action::Stay; M];
        for i in 0..M {
            dispersal1[i] = Action::ALL[i % 4]; // Skip Stay for initial dispersal
        }
        configs.push(dispersal1);

        // Corner seeking pattern
        let mut corners = [Action::Stay; M];
        for i in 0..M {
            corners[i] = match i % 4 {
                0 => Action::Right,
                1 => Action::Down,
                2 => Action::Left,
                3 => Action::Up,
                _ => Action::Stay,
            };
        }
        configs.push(corners);

        // Random exploration
        let mut random_config = [Action::Stay; M];
        for i in 0..M {
            random_config[i] = Action::ALL[self.rng.gen_range(0..Action::ALL.len())];
        }
        configs.push(random_config);

        configs
    }

    fn solve_greedy_scan(&mut self, initial_state: &State) -> State {
        let mut best_state = initial_state.clone();
        
        // Phase 1: Systematic row-by-row scanning
        let scan_patterns = [
            (0, 2, 1, 2), // Right, Down, Left, Down
            (1, 3, 0, 3), // Left, Up, Right, Up
        ];

        for (i, &(move1, move2, move3, move4)) in scan_patterns.iter().enumerate() {
            for _ in 0..15 {
                if (i == 0 && (0usize..30).filter(|&x| !best_state.waxed.contains(&(x, 29usize))).count() == 0) || (i == 1 && (0usize..30).filter(|&x| !best_state.waxed.contains(&(x, 0usize))).count() == 0) {break;}
                if best_state.operations.len() >= MAX_OPERATIONS { break; }
                
                // Horizontal sweep
                while best_state.robots.iter().enumerate().filter(|&x| self.grid.can_move(x.1.current_pos, self.button_configs[move1][x.0]) && !is_waxed(x.1.current_pos, self.button_configs[move1][x.0], &best_state.waxed)).count() != 0 {
                    if best_state.operations.len() >= MAX_OPERATIONS { break; }
                    best_state.operations.push(move1);
                    best_state.apply_button(&self.grid, &self.button_configs[move1]);
                }
                
                // Vertical move
                if best_state.operations.len() < MAX_OPERATIONS {
                    best_state.operations.push(move2);
                    best_state.apply_button(&self.grid, &self.button_configs[move2]);
                }
                
                // Reverse horizontal sweep
                while best_state.robots.iter().enumerate().filter(|x| self.grid.can_move(x.1.current_pos, self.button_configs[move3][x.0]) && !is_waxed(x.1.current_pos, self.button_configs[move1][x.0], &best_state.waxed)).count() != 0 {
                    if best_state.operations.len() >= MAX_OPERATIONS { break; }
                    best_state.operations.push(move3);
                    best_state.apply_button(&self.grid, &self.button_configs[move3]);
                }
                
                // Vertical move
                if best_state.operations.len() < MAX_OPERATIONS {
                    best_state.operations.push(move4);
                    best_state.apply_button(&self.grid, &self.button_configs[move4]);
                }

                if best_state.coverage_ratio() > 0.9 {
                    break;
                }
            }
            
            if best_state.coverage_ratio() > 0.95 {
                break;
            }
        }

        best_state
    }

    fn local_search_improvement(&mut self, state: &mut State, max_iterations: usize) {
        let mut improvement_found = true;
        let mut iterations = 0;

        while improvement_found && iterations < max_iterations && !self.timer.is_time_over(TIME_LIMIT * 0.9) {
            improvement_found = false;
            iterations += 1;
            
            let current_score = state.calculate_score();
            
            // Try extending with each button
            for button_idx in 0..self.button_configs.len() {
                if state.operations.len() >= MAX_OPERATIONS { break; }
                
                let mut test_state = state.clone();
                test_state.operations.push(button_idx);
                test_state.apply_button(&self.grid, &self.button_configs[button_idx]);
                
                if test_state.calculate_score() > current_score {
                    *state = test_state;
                    improvement_found = true;
                    break;
                }
            }
        }
    }

    fn simulated_annealing(&mut self, initial_state: &State, max_iterations: usize) -> State {
        let mut current_state = initial_state.clone();
        let mut best_state = current_state.clone();
        
        let mut temperature = 100.0;
        let cooling_rate = 0.999;
        let mut iteration = 0;

        while iteration < max_iterations && !self.timer.is_time_over(TIME_LIMIT * 0.95) {
            iteration += 1;
            temperature *= cooling_rate;

            let mut new_state = current_state.clone();
            
            // Random mutation strategies
            match self.rng.gen_range(0..3) {
                0 => {
                    // Change random operation
                    if !new_state.operations.is_empty() {
                        let pos = self.rng.gen_range(0..new_state.operations.len());
                        new_state.operations[pos] = self.rng.gen_range(0..self.button_configs.len());
                    }
                }
                1 => {
                    // Add operation
                    if new_state.operations.len() < MAX_OPERATIONS {
                        new_state.operations.push(self.rng.gen_range(0..self.button_configs.len()));
                    }
                }
                2 => {
                    // Remove operation
                    if !new_state.operations.is_empty() {
                        let pos = self.rng.gen_range(0..new_state.operations.len());
                        new_state.operations.remove(pos);
                    }
                }
                _ => {}
            }

            // Re-simulate
            self.simulate_state(&mut new_state);

            let current_score = current_state.calculate_score();
            let new_score = new_state.calculate_score();
            
            let accept = if new_score > current_score {
                true
            } else {
                let prob = ((new_score - current_score) as f64 / temperature).exp();
                self.rng.gen::<f64>() < prob
            };

            if accept {
                current_state = new_state.clone();
                
                if new_score > best_state.calculate_score() {
                    best_state = new_state;
                }
            }
        }

        best_state
    }

    fn simulate_state(&self, state: &mut State) {
        state.reset_to_initial();
        
        for button_idx in state.operations.clone() {
            if button_idx < self.button_configs.len() {
                state.apply_button(&self.grid, &self.button_configs[button_idx]);
            }
        }
    }

    fn solve(&mut self, robot_positions: Vec<(usize, usize)>) -> State {
        eprintln!("Starting solver with {} robots", robot_positions.len());
        
        self.button_configs = self.generate_smart_button_configs();
        let initial_state = State::new(robot_positions);
        
        eprintln!("Initial coverage: {:.1}%", initial_state.coverage_ratio() * 100.0);

        // Phase 1: Greedy scanning
        let mut best_solution = self.solve_greedy_scan(&initial_state);
        eprintln!("After greedy scan - Score: {}, Coverage: {:.1}%", 
                 best_solution.calculate_score(), best_solution.coverage_ratio() * 100.0);

        // Phase 2: Local search improvement
        self.local_search_improvement(&mut best_solution, 100);
        eprintln!("After local search - Score: {}, Coverage: {:.1}%", 
                 best_solution.calculate_score(), best_solution.coverage_ratio() * 100.0);

        // Phase 3: Simulated annealing if time permits
        if !self.timer.is_time_over(TIME_LIMIT * 0.7) {
            let sa_solution = self.simulated_annealing(&best_solution, 1000);
            if sa_solution.calculate_score() > best_solution.calculate_score() {
                best_solution = sa_solution;
                eprintln!("After simulated annealing - Score: {}, Coverage: {:.1}%", 
                         best_solution.calculate_score(), best_solution.coverage_ratio() * 100.0);
            }
        }

        eprintln!("Final solution - Score: {}, Operations: {}, Coverage: {:.1}%",
                best_solution.calculate_score(), 
                best_solution.operations.len(),
                best_solution.coverage_ratio() * 100.0
        );

        best_solution
    }
}

fn parse_input() -> (Grid, Vec<(usize, usize)>) {
    input! {
        n: usize, m: usize, _k: usize,
        robot_positions: [(usize, usize); m],
        vertical_walls: [String; n],
        horizontal_walls: [String; n-1],
    }

    let mut grid = Grid::new(n);
    
    // Parse vertical walls
    for (i, line) in vertical_walls.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if j < n.saturating_sub(1) {
                grid.vertical_walls[i][j] = c == '1';
            }
        }
    }
    
    // Parse horizontal walls
    for (i, line) in horizontal_walls.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if j < n {
                grid.horizontal_walls[i][j] = c == '1';
            }
        }
    }

    (grid, robot_positions)
}

fn output_solution(solver: &Solver, state: &State) {
    // Output button configurations
    for config in &solver.button_configs {
        for (i, &action) in config.iter().enumerate() {
            if i > 0 { print!(" "); }
            print!("{}", action.to_char());
        }
        println!();
    }

    // Output operations
    for &op in &state.operations {
        println!("{}", op);
    }
}

fn main() {
    let (grid, robot_positions) = parse_input();
    let mut solver = Solver::new(grid, 12345, Timer::new());
    
    let solution = solver.solve(robot_positions);
    output_solution(&solver, &solution);
}