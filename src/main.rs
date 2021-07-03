use std::collections::VecDeque;
use std::io::Write;
use std::iter::repeat_with;

use find_the_relation::{Matrix, Group, Game, Level, LeafState, Direction};


type Solution = Vec<Direction>;

struct Solutions {
    pub stories: Vec<Solution>,
    length: Option<usize>,
    first_only: bool,
}

fn solution_as_str(s: &Solution) -> String {
    s.iter().map(|d| d.to_string()).collect()
}

impl Solutions {
    pub fn new(first_only: bool) -> Self {
        Self {stories: Vec::new(), length: None, first_only}
    }

    pub fn evaluate(&mut self, leaf: &LeafState) -> bool {
        if let Some(length) = self.length {
            if leaf.length() > length {return false;}
        }
        match (self.length, if leaf.is_solution() {Some(leaf.recipe())} else {None}) {
            (Some(length), Some(solution)) => {
                if solution.len()<length {
                    self.length = Some(solution.len());
                    self.stories = vec![solution];
                } else {
                    self.stories.push(solution);
                };
                false
            },
            (None, Some(solution)) => {
                self.length = Some(solution.len());
                println!("\nFirst solution: {}", solution_as_str(&solution));
                self.stories.push(solution);
                false
            }
            (Some(length), None) => !self.first_only && (leaf.length() < length),
            (None, None) => true,
        }
    }
}

fn solve(level: &Level) {
    let mut work: VecDeque<_> = level.og().collect();
    let mut solutions = Solutions::new(false);
    let mut n_max: usize = 0;
    while let Some(w) = work.pop_front() {
        if w.length() > n_max {
            n_max = w.length();
            print!(", {}", n_max);
            std::io::stdout().flush().unwrap();
        }
        if solutions.evaluate(&w) {
            work.extend(level.get_children(&w)); //hoping this is push back
        }
    }

    print!("\nSolutions: ");
    for story in solutions.stories.iter() {
        print!("{}, ", solution_as_str(story));
    }
    println!("");
}

struct SortableLeafState {
    leaf: LeafState
}

fn error_measure(l: &LeafState) -> f64 {
    let dist = l.state.iter().map(|m| m.distance2_from_identity()).sum::<f64>();
    let infidelity = dist.sqrt() + l.length() as f64;
    -1.0 * infidelity
}

impl PartialEq for SortableLeafState {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl PartialOrd for SortableLeafState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        error_measure(&self.leaf).partial_cmp(&error_measure(&other.leaf))
    }
}

impl Eq for SortableLeafState {}

impl Ord for SortableLeafState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn solve2(level: &Level) {
    let mut work: std::collections::BinaryHeap<_> = level.og().map(|l| SortableLeafState{leaf: l}).collect();
    let mut solutions = Solutions::new(true);
    let mut n_max: usize = 0;
    while let Some(ww) = work.pop() {
        let w = ww.leaf;
        if w.length() > n_max {
            n_max = w.length();
            print!(", {}", n_max);
            std::io::stdout().flush().unwrap();
        }
        if solutions.evaluate(&w) {
            work.extend(level.get_children(&w).map(|l| SortableLeafState{leaf: l}));
        }
    }

    print!("\nSolutions: ");
    for story in solutions.stories.iter() {
        print!("{}, ", solution_as_str(story));
    }
    println!("");
}

struct Step {
    state: Matrix,
    direction_index: i8,
    distance2: f64,
}

fn solve_vec(g:&Group, n: usize) {
    let mut v: Vec<Step> = repeat_with(|| Step{state: Matrix::identity(), direction_index: -1, distance2: 0.0}).take(n+1).collect();
    let mut m: usize = 1;
    let m_max: usize = n;
    while m>0 {
        let mut d= v[m].direction_index+1;
        if (d+2)%4 == v[m-1].direction_index {
            d+=1;
        }
        if d>3 {
            m-=1;
            continue
        }
        if m<3 {
            println!("Level {} direction {}", m, d);
        }
        v[m].direction_index = d;
        v[m].state = g.apply(&v[m-1].state, Direction::by_index(d));
        if v[m].state.is_identity() {break;}
        if m<m_max {
            v[m+1].direction_index = -1;
            m+=1;
        }
    }
    if m>0 {
        let solution: Vec<Direction> = v.iter().skip(1).map(|s| Direction::by_index(s.direction_index)).take(m).collect();
        println!("Solution len {}: {}", m, solution_as_str(&solution));
    } else {
        println!("No solution at n: {}", n);
    }
}
fn main() {
    let game=Game::new();
    for (i, level) in game.levels.iter().enumerate() {
        println!("Level: {}", i);
        if level.groups.len()==1 {
            println!("Vector solve");
            solve_vec(&level.groups[0], 32);
        } else {
            println!("skipping");
            //solve2(level);
        }
    }
}