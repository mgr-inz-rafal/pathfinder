enum PathfindingError {
    OutOfMap
}

#[derive(Default, PartialEq, Debug, Clone)]
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

#[cfg(debug_assertions)]
static MAX_DISTANCE: f64 = 6666.0;
#[cfg(not(debug_assertions))]
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

    fn glue_path_to_destination(&self, node: Node, buf: &mut String) {
        let mut current_pos = Point2d { x: node.my_pos.x, y: node.my_pos.y};
        loop {
            let n = self.get_field_at(&current_pos);
            buf.push_str(&n.my_pos.x.to_string());
            buf.push_str(",");
            buf.push_str(&n.my_pos.y.to_string());
            buf.push_str(" ");
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

    /*
    #[cfg(debug_assertions)]
    fn _dump(&self) {   // For debug purposes only
        println!("Start: {:?}   End: {:?}", self.start, self.destination);
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{:5.2} ", self.field[self.to_index(x, y)].penalty);
                let point = Point2d{ x, y };
                match point {
                    ref tmp if (*tmp == self.start)         => print!("S"),
                    ref tmp if (*tmp == self.destination)   => print!("D"),
                    _ => print!(".")
                }
                match self.field[self.to_index(x, y)].visited {
                    true => print!("1  "),
                    false => print!("0  ")
                }
            }
            println!();
            for x in 0..self.width {
                print!("{:8.2} ", self.field[self.to_index(x, y)].distance);
            }
            println!();
            println!();
        }
        let shortest_distance_position = self.find_shortest_distance();
        println!("Shortest distance at: {:?}", shortest_distance_position);
    }
    */
}

pub fn calculate_shortest_path(width: u64, height: u64, map: Vec<f64>, start: (u64, u64), destination: (u64, u64)) -> String {
    let start_point = Point2d {x: start.0, y: start.1};
    let destination_point = Point2d {x: destination.0, y: destination.1};
    let mut playfield = Playfield::new(width, height, start_point, destination_point, map);
    calculate_from_playfield(&mut playfield)
}

fn calculate_from_playfield(playfield: &mut Playfield) -> String {
    // Validate precondition (map initialized, start and end set correctly, etc.)
    let offsets: [(i64, i64); 4] = [
        (-1, 0), (1, 0), (0, -1), (0, 1)
    ];

    let mut buf = String::new();

    loop {
        let current_pos = playfield.find_shortest_distance();
        let current_node = playfield.get_field_at(&current_pos);
        playfield.set_visited(&current_pos);

        for offset in offsets.iter() {
            let new_pos = playfield.apply_offset(&current_pos, offset);
            match new_pos {
                Err(_) => { continue; },
                Ok(position) => {
                    let mut candidate = playfield.get_field_at(&position);
                    if !candidate.visited {
                        candidate.distance = current_node.distance + 1000.0 * candidate.penalty;    // TODO: Do not hardcode 1000.0
                        candidate.predecessor = Some(current_node.my_pos.clone());
                        playfield.set_field_at(&position, &candidate);
                        if position == playfield.destination {
                            playfield.glue_path_to_destination(candidate, &mut buf);
                            return buf;
                        }
                    }
                }
            }
        }
    }

    // Dead code
    return "Not found".to_string();
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
        assert_eq!(result.trim(), "4,1 3,1 3,2 2,2 1,2 1,1");
    }
}

// TODO: Return with Option everywhere