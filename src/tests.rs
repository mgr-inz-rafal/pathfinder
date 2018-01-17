use *;

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
