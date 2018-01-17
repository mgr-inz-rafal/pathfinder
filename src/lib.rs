mod playfield;
mod errors;
mod constdefs;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::fmt;
use playfield::*;
use constdefs::*;

#[derive(PartialEq)]
enum CandidateStatus {
    StillLooking,
    AtDestination
}

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
            self.current_node = self.playfield.field_at(&self.current_pos);
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
        self.candidate = self.playfield.field_at(&point);
        if !self.candidate.visited {
            self.candidate.distance = self.current_node.distance + self.candidate.penalty;
            self.candidate.predecessor = Some(self.current_node.my_pos.clone());
            self.playfield.set_field_at(&point, &self.candidate);   // See why I "cannot move out of borrowed content" and need to borrow candidate
        }
        if point == self.playfield.destination { CandidateStatus::AtDestination } else { CandidateStatus::StillLooking }
    }
}

#[derive(Default, Serialize, Deserialize)]
struct Response {
    status: bool,
    comment: String,
    path: Path
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

pub fn calculate_shortest_path(width: i64, height: i64, map: Vec<f64>, start: (i64, i64), destination: (i64, i64)) -> String {
    let start_point = Point2d {x: start.0, y: start.1};
    let destination_point = Point2d {x: destination.0, y: destination.1};
    match Playfield::new(width, height, start_point, destination_point, map) {
        Ok(playfield) => {
            let mut pf = Pathfinder{ playfield, ..Default::default() };
            pf.calculate();
            let mut resp = Response { status: true, comment: "Ok".to_string(), path: pf.path };
            serde_json::to_string(&resp).unwrap()
        },
        Err(e) => {
            let mut resp = Response { status: false, comment: format!("[ERROR] {:?}", e), ..Default::default() };
            serde_json::to_string(&resp).unwrap()
         }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
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

        let deserialized: Response = serde_json::from_str(&result).unwrap();

        assert_eq!(deserialized.path.steps[0], Point2d{ x: 1, y: 1});
        assert_eq!(deserialized.path.steps[1], Point2d{ x: 1, y: 2});
        assert_eq!(deserialized.path.steps[2], Point2d{ x: 2, y: 2});
        assert_eq!(deserialized.path.steps[3], Point2d{ x: 3, y: 2});
        assert_eq!(deserialized.path.steps[4], Point2d{ x: 3, y: 1});
        assert_eq!(deserialized.path.steps[5], Point2d{ x: 4, y: 1});
   }

    #[test]
    fn falling_over_top_edge() {
        let test_level = vec![
            1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
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
        let deserialized: Response = serde_json::from_str(&result).unwrap();

        assert_eq!(deserialized.path.steps[0], Point2d{ x: 1, y: 1});
        assert_eq!(deserialized.path.steps[1], Point2d{ x: 1, y: 2});
        assert_eq!(deserialized.path.steps[2], Point2d{ x: 2, y: 2});
        assert_eq!(deserialized.path.steps[3], Point2d{ x: 3, y: 2});
        assert_eq!(deserialized.path.steps[4], Point2d{ x: 3, y: 1});
        assert_eq!(deserialized.path.steps[5], Point2d{ x: 4, y: 1});
   }

    #[test]
    fn falling_over_left_edge() {
        let test_level = vec![
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            0.0, 0.1, 0.7, 0.1, 0.1, 0.1, 1.0,
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
        let deserialized: Response = serde_json::from_str(&result).unwrap();

        assert_eq!(deserialized.path.steps[0], Point2d{ x: 1, y: 1});
        assert_eq!(deserialized.path.steps[1], Point2d{ x: 1, y: 2});
        assert_eq!(deserialized.path.steps[2], Point2d{ x: 2, y: 2});
        assert_eq!(deserialized.path.steps[3], Point2d{ x: 3, y: 2});
        assert_eq!(deserialized.path.steps[4], Point2d{ x: 3, y: 1});
        assert_eq!(deserialized.path.steps[5], Point2d{ x: 4, y: 1});
   }

    #[test]
    fn falling_over_bottom_edge() {
        let test_level = vec![
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            0.0, 0.1, 0.7, 0.1, 0.1, 0.1, 1.0,
            1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0,
            1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0,
            1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0
        ];
        let width = 7;
        let height = 5;
        let start = (1, 4);
        let destination = (4, 1);
        let result = calculate_shortest_path(width, height, test_level, start, destination);

        // Got JSON - deserialize it and verify predefined path
        let deserialized: Response = serde_json::from_str(&result).unwrap();

        assert_eq!(deserialized.path.steps[0], Point2d{ x: 1, y: 4});
        assert_eq!(deserialized.path.steps[1], Point2d{ x: 1, y: 3});
        assert_eq!(deserialized.path.steps[2], Point2d{ x: 2, y: 3});
        assert_eq!(deserialized.path.steps[3], Point2d{ x: 3, y: 3});
        assert_eq!(deserialized.path.steps[4], Point2d{ x: 3, y: 2});
        assert_eq!(deserialized.path.steps[5], Point2d{ x: 3, y: 1});
        assert_eq!(deserialized.path.steps[6], Point2d{ x: 4, y: 1});
   }

    #[test]
    fn falling_over_right_edge() {
        let test_level = vec![
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 0.1, 0.7, 0.1, 0.1, 0.1, 0.0,
            1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0,
            1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0
        ];
        let width = 7;
        let height = 5;
        let start = (4, 1);
        let destination = (1, 1);
        let result = calculate_shortest_path(width, height, test_level, start, destination);

        // Got JSON - deserialize it and verify predefined path
        let deserialized: Response = serde_json::from_str(&result).unwrap();

        assert_eq!(deserialized.path.steps[0], Point2d{ x: 4, y: 1});
        assert_eq!(deserialized.path.steps[1], Point2d{ x: 4, y: 2});
        assert_eq!(deserialized.path.steps[2], Point2d{ x: 3, y: 2});
        assert_eq!(deserialized.path.steps[3], Point2d{ x: 2, y: 2});
        assert_eq!(deserialized.path.steps[4], Point2d{ x: 1, y: 2});
        assert_eq!(deserialized.path.steps[5], Point2d{ x: 1, y: 1});
   }

    #[test]
    fn map_validation_start_eq_destination() {
        let test_level = vec![
            1.0, 1.0, 1.0,
            1.0, 0.1, 0.7,
            1.0, 0.1, 0.1
        ];
        let width = 3;
        let height = 3;
        let start = (2, 2);
        let destination = (2, 2);
        let result = calculate_shortest_path(width, height, test_level, start, destination);
        let deserialized: Response = serde_json::from_str(&result).unwrap();
        assert_eq!(deserialized.comment, "[ERROR] StartEqEnd");
    }

    #[test]
    fn map_validation_size_mismatch() {
        let test_level = vec![
            1.0, 1.0,
            1.0, 0.1
        ];
        let width = 3;
        let height = 3;
        let start = (2, 2);
        let destination = (2, 2);
        let result = calculate_shortest_path(width, height, test_level, start, destination);
        let deserialized: Response = serde_json::from_str(&result).unwrap();
        assert_eq!(deserialized.comment, "[ERROR] SizeMismatch");
    }

    #[test]
    fn map_validation_start_out_of_bounds_negative() {
        let test_level = vec![
            1.0, 1.0, 1.0,
            1.0, 0.1, 0.7,
            1.0, 0.1, 0.1
        ];
        let width = 3;
        let height = 3;
        let start = (-3, -3);
        let destination = (2, 2);
        let result = calculate_shortest_path(width, height, test_level, start, destination);
        let deserialized: Response = serde_json::from_str(&result).unwrap();
        assert_eq!(deserialized.comment, "[ERROR] StartOutOfBounds");
    }

    #[test]
    fn map_validation_start_out_of_bounds_positive() {
        let test_level = vec![
            1.0, 1.0, 1.0,
            1.0, 0.1, 0.7,
            1.0, 0.1, 0.1
        ];
        let width = 3;
        let height = 3;
        let start = (10, 10);
        let destination = (2, 2);
        let result = calculate_shortest_path(width, height, test_level, start, destination);
        let deserialized: Response = serde_json::from_str(&result).unwrap();
        assert_eq!(deserialized.comment, "[ERROR] StartOutOfBounds");
    }

    #[test]
    fn map_validation_destination_out_of_bounds_negative() {
        let test_level = vec![
            1.0, 1.0, 1.0,
            1.0, 0.1, 0.7,
            1.0, 0.1, 0.1
        ];
        let width = 3;
        let height = 3;
        let start = (1, 1);
        let destination = (-100, -100);
        let result = calculate_shortest_path(width, height, test_level, start, destination);
        let deserialized: Response = serde_json::from_str(&result).unwrap();
        assert_eq!(deserialized.comment, "[ERROR] DestinationOutOfBounds");
    }

    #[test]
    fn map_validation_destination_out_of_bounds_positive() {
        let test_level = vec![
            1.0, 1.0, 1.0,
            1.0, 0.1, 0.7,
            1.0, 0.1, 0.1
        ];
        let width = 3;
        let height = 3;
        let start = (1, 1);
        let destination = (200, 200);
        let result = calculate_shortest_path(width, height, test_level, start, destination);
        let deserialized: Response = serde_json::from_str(&result).unwrap();
        assert_eq!(deserialized.comment, "[ERROR] DestinationOutOfBounds");
    }

    #[test]
    fn map_validation_destination_too_big_width() {
        let test_level = vec![
            1.0, 1.0, 1.0,
            1.0, 0.1, 0.7,
            1.0, 0.1, 0.1
        ];
        let width = MAX_WIDTH+1;
        let height = 3;
        let start = (1, 1);
        let destination = (200, 200);
        let result = calculate_shortest_path(width, height, test_level, start, destination);
        let deserialized: Response = serde_json::from_str(&result).unwrap();
        assert_eq!(deserialized.comment, "[ERROR] TooBig");
    }

    #[test]
    fn map_validation_destination_too_big_height() {
        let test_level = vec![
            1.0, 1.0, 1.0,
            1.0, 0.1, 0.7,
            1.0, 0.1, 0.1
        ];
        let width = 3;
        let height = MAX_HEIGHT+1;
        let start = (1, 1);
        let destination = (200, 200);
        let result = calculate_shortest_path(width, height, test_level, start, destination);
        let deserialized: Response = serde_json::from_str(&result).unwrap();
        assert_eq!(deserialized.comment, "[ERROR] TooBig");
    }

    #[test]
    fn map_validation_destination_too_big_height_width() {
        let test_level = vec![
            1.0, 1.0, 1.0,
            1.0, 0.1, 0.7,
            1.0, 0.1, 0.1
        ];
        let width = MAX_WIDTH+1;
        let height = MAX_HEIGHT+1;
        let start = (1, 1);
        let destination = (200, 200);
        let result = calculate_shortest_path(width, height, test_level, start, destination);
        let deserialized: Response = serde_json::from_str(&result).unwrap();
        assert_eq!(deserialized.comment, "[ERROR] TooBig");
    }
}
