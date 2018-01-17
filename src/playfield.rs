use errors::*;
use constdefs::*;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Point2d {
    pub x: i64,
    pub y: i64
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Path {
    pub steps: Vec<Point2d>
}

#[derive(Clone, Debug, Default)]
pub struct Node {
    pub my_pos: Point2d,
    pub penalty: f64,
    pub visited: bool,
    pub distance: f64,
    pub predecessor: Option<Point2d>,
}

#[derive(Default)]
pub struct Playfield {
    pub width: i64,
    pub height: i64,
    pub field: Vec<Node>,
    pub start: Point2d,
    pub destination: Point2d,
}

impl Playfield {
    pub fn new(width: i64, height: i64, start: Point2d, destination: Point2d, map: Vec<f64>) -> Result<Playfield, MapError> {
        if width > MAX_WIDTH || height > MAX_HEIGHT {
            return Err(MapError::TooBig);
        }
        if (width * height) as usize != map.len() {
            return Err(MapError::SizeMismatch);
        }
        if start == destination {
            return Err(MapError::StartEqEnd);
        }
        if start.x < 0 || start.x >= width || start.y < 0 || start.y >= height {
            return Err(MapError::StartOutOfBounds)
        }
        if destination.x < 0 || destination.x >= width || destination.y < 0 || destination.y >= height {
            return Err(MapError::DestinationOutOfBounds)
        }
        let n = Node { distance: MAX_DISTANCE, ..Default::default() };
        let mut playfield: Playfield = Playfield { width, height, start, destination, ..Default::default() };
        playfield.field.resize((playfield.width * playfield.height) as usize, n);
        playfield.init_with_vector(map);
        Ok(playfield)
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

    pub fn apply_offset(&self, point: &Point2d, offset: &(i64, i64)) -> Result<Point2d, PathfindingError> {
        let new_x = point.x + offset.0;
        let new_y = point.y + offset.1;

        if new_x.is_positive() && new_x < self.width &&
           new_y.is_positive() && new_y < self.height
        {
            { Ok(Point2d {x: new_x, y: new_y}) }
        }
        else
        {
            { Err(PathfindingError::OutOfMap) }
        }
    }

    pub fn glue_path_to_destination(&self, node: &Node, path: &mut Path) {
        let mut current_pos = Point2d { x: node.my_pos.x, y: node.my_pos.y};
        loop {
            let n = self.field_at(&current_pos);
            path.steps.insert(0, n.my_pos);
            match n.predecessor {
                None => return,
                Some(cp) => {
                    current_pos = Point2d {x: cp.x, y: cp.y}
                    }
            }
        }
    }

    pub fn to_index(&self, point: &Point2d) -> usize {
        if point.x.is_negative() || point.y.is_negative() {
            panic!("Referencing field with negative coordinates: ({},{})", point.x, point.y);
        }
        if point.x >= self.width || point.y >= self.height {
            panic!("Referencing field from beyond the map boundaries: ({},{}) - maximum indices: ({},{})", point.x, point.y, self.width-1, self.height-1);
        }
        (point.y * self.width + point.x) as usize
    }

    fn from_index(&self, i: usize) -> Point2d {
        let y = (i as i64) / self.width;
        let x = (i as i64) - y * self.width;
        Point2d { x, y }
    }

    pub fn set_visited(&mut self, point: &Point2d) {
        let index = self.to_index(&point);
        let pt = &mut self.field[index];
        pt.visited = true;
    }

    pub fn field_at(&self, point: &Point2d) -> Node {
        return self.field[self.to_index(point)].clone();
    }

    pub fn set_field_at(&mut self, point: &Point2d, node: &Node) {
        let index = self.to_index(point);
        self.field[index] = node.clone();
    }

    pub fn find_shortest_distance(&self) -> Point2d {
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
