// TODO: Not sure if lib should spit out JSON... Maybe 'pathfinder-server' should
// just receive an object and then work on the serialization.
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::fmt;

enum PathfindingError {
    OutOfMap
}

#[derive(PartialEq)]
enum CandidateStatus {
    StillLooking,
    AtDestination
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Point2d {
    pub x: u64,
    pub y: u64
}

#[derive(Clone, Debug, Default)]
pub struct Node {
    pub my_pos: Point2d,
    pub penalty: f64,
    pub visited: bool,
    pub distance: f64,
    pub predecessor: Option<Point2d>,
}

static OFFSETS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Default)]
pub struct Pathfinder {
    current_pos: Point2d,
    current_node: Node,
    candidate: Node,
    path: Path,
    playfield: Playfield
}

impl Pathfinder {
    fn calculate(&mut self) {
        loop {
            self.current_pos = self.playfield.find_shortest_distance();
            self.current_node = self.playfield.get_field_at(&self.current_pos);
            self.playfield.set_visited(&self.current_pos);

            for offset in OFFSETS.iter() {
                let new_pos = self.playfield.apply_offset(&self.current_pos, offset);
                match new_pos {
                    Err(_) => { continue; },
                    Ok(position) => {
                        if self.apply_candidate(position) == CandidateStatus::AtDestination {
                            self.playfield.glue_path_to_destination(&self.candidate, &mut self.path);
                            return;
                        }
                    }
                }
            }
        }
    }

    fn apply_candidate(&mut self, point: Point2d) -> CandidateStatus {
        self.candidate = self.playfield.get_field_at(&point);
        if !self.candidate.visited {
            self.candidate.distance = self.current_node.distance + self.candidate.penalty;
            self.candidate.predecessor = Some(self.current_node.my_pos.clone());
            self.playfield.set_field_at(&point, &self.candidate);
        }
        if point == self.playfield.destination { CandidateStatus::AtDestination } else { CandidateStatus::StillLooking }
    }
}

static MAX_DISTANCE: f64 = std::f64::MAX;

#[derive(Default)]
// TODO: Privatize filed
struct Playfield {
    pub width: u64,
    pub height: u64,
    field: Vec<Node>,
    start: Point2d,
    pub destination: Point2d,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Path {
    steps: Vec<Point2d>
}

#[cfg(debug_assertions)]
impl fmt::Debug for Playfield {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Start: {:?}   End: {:?}", self.start, self.destination).unwrap();
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point2d{x: x, y: y};
                write!(f, "{:5.2} ", self.field[self.to_index(&point)].penalty).unwrap();
                let point = Point2d{ x, y };
                match point {
                    ref tmp if (*tmp == self.start)         => write!(f, "S").unwrap(),
                    ref tmp if (*tmp == self.destination)   => write!(f, "D").unwrap(),
                    _ => write!(f, ".").unwrap()
                };
                match self.field[self.to_index(&point)].visited {
                    true => write!(f, "1  ").unwrap(),
                    false => write!(f, "0  ").unwrap()
                };
            }
            writeln!(f).unwrap();
            for x in 0..self.width {
                let point = Point2d{x: x, y: y};
                write!(f, "{:8.2} ", self.field[self.to_index(&point)].distance).unwrap();
            }
            writeln!(f).unwrap();
            writeln!(f).unwrap();
        }
        let shortest_distance_position = self.find_shortest_distance();
        write!(f, "Shortest distance at: {:?}", shortest_distance_position)
    }
}

impl Playfield {
    // TODO: Validate width*height = vec.len
    fn new(width: u64, height: u64, start: Point2d, destination: Point2d, map: Vec<f64>) -> Playfield {
        let n = Node { distance: MAX_DISTANCE, ..Default::default() };
        let mut playfield: Playfield = Playfield { width, height, start, destination, ..Default::default() };
        playfield.field.resize((playfield.width * playfield.height) as usize, n);
        playfield.init_with_vector(map);
        playfield
    }

    fn init_with_vector(&mut self, playfield: Vec<f64>) {
        for x in 0..playfield.len() {
            let position = self.from_index(x);
            self.field[x] = Node{
                penalty: playfield[x],
                visited: false,
                distance: MAX_DISTANCE,
                my_pos: position,
                ..Default::default() };
        }
        let start_index = self.to_index(&self.start);
        self.field[start_index].distance = 0.0;
    }

    // TODO: Rework this. Maybe playfield should return neighbours according to given offset?
    fn apply_offset(&self, point: &Point2d, offset: &(i64, i64)) -> Result<Point2d, PathfindingError> {
        if point.x == 0 && offset.0 == -1 {
            Err(PathfindingError::OutOfMap)
        } else if point.y == 0 && offset.1 == -1 {
            Err(PathfindingError::OutOfMap)
        } else if point.x == self.width-1 && offset.0 == 1 {
            Err(PathfindingError::OutOfMap)
        } else if point.y == self.height-1 && offset.1 == 1 {
            Err(PathfindingError::OutOfMap)
        } else {
            Ok(
                Point2d {x: (point.x as i64 + offset.0) as u64, y: (point.y as i64 + offset.1) as u64}
                )
        }
    }

    fn glue_path_to_destination(&self, node: &Node, path: &mut Path) {
        let mut current_pos = Point2d { x: node.my_pos.x, y: node.my_pos.y};
        loop {
            let n = self.get_field_at(&current_pos);
            path.steps.insert(0, n.my_pos);
            match n.predecessor {
                None => return,
                Some(cp) => {
                    current_pos = Point2d {x: cp.x, y: cp.y}
                    }
            }
        }
    }

    fn to_index(&self, point: &Point2d) -> usize {
        (point.y * self.width + point.x) as usize
    }

    fn from_index(&self, i: usize) -> Point2d {
        let y = (i as u64) / self.width;
        let x = (i as u64) - y * self.width;
        Point2d { x, y }
    }

    fn set_visited(&mut self, point: &Point2d) {
        let index = self.to_index(&point);
        let pt = &mut self.field[index];
        pt.visited = true;
    }

    fn get_field_at(&self, point: &Point2d) -> Node {
        return self.field[self.to_index(point)].clone();
    }

    fn set_field_at(&mut self, point: &Point2d, node: &Node) {
        let index = self.to_index(point);
        self.field[index] = node.clone();
    }

    fn find_shortest_distance(&self) -> Point2d {
        let mut min_so_far = ::std::f64::MAX;
        let mut min_index: usize = ::std::usize::MAX;
        for index in 0..self.field.len() {
            if self.field[index].distance < min_so_far && self.field[index].visited == false {
                min_so_far = self.field[index].distance;
                min_index = index;
            }
        }
        self.from_index(min_index)
    }
}

pub fn calculate_shortest_path(width: u64, height: u64, map: Vec<f64>, start: (u64, u64), destination: (u64, u64)) -> String {
    let start_point = Point2d {x: start.0, y: start.1};
    let destination_point = Point2d {x: destination.0, y: destination.1};
    let playfield = Playfield::new(width, height, start_point, destination_point, map);
    let mut pf = Pathfinder{ playfield, ..Default::default() };
    pf.calculate();
    serde_json::to_string(&pf.path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lib_find_shortest_path_alt() {
        let test_level = vec![
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 0.1, 0.7, 0.1, 0.1, 0.1, 1.0,
            1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0,
            1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0
        ];
        let width = 7;
        let height = 5;
        let start = (1, 1);
        let destination = (4, 1);
        let result = calculate_shortest_path(width, height, test_level, start, destination);

        // Got JSON - deserialize it and verify predefined path
        let deserialized: Path = serde_json::from_str(&result).unwrap();

        assert_eq!(deserialized.steps[0], Point2d{ x: 1, y: 1});
        assert_eq!(deserialized.steps[1], Point2d{ x: 1, y: 2});
        assert_eq!(deserialized.steps[2], Point2d{ x: 2, y: 2});
        assert_eq!(deserialized.steps[3], Point2d{ x: 3, y: 2});
        assert_eq!(deserialized.steps[4], Point2d{ x: 3, y: 1});
        assert_eq!(deserialized.steps[5], Point2d{ x: 4, y: 1});
   }
}
