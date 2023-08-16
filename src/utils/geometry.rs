/// Get the distance between two points.
pub fn distance(point1: &(f64, f64), point2: &(f64, f64)) -> f64 {
    let (x1, y1) = point1;
    let (x2, y2) = point2;
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/// Get the most central coordinate.
pub fn calculate_centroid(coords: &Vec<(f64, f64)>) -> (f64, f64) {
    let sum = coords
        .iter()
        .fold((0.0, 0.0), |acc, &(x, y)| (acc.0 + x, acc.1 + y));
    (sum.0 / coords.len() as f64, sum.1 / coords.len() as f64)
}

/// Get the radial distance.
pub fn compute_radial_distance(centroid: &(f64, f64), point: &(f64, f64)) -> f64 {
    let (cx, cy) = centroid;
    (point.1 - cy).atan2(point.0 - cx)
}

/// Get the furthest pair of coordinates.
pub fn get_furthest_pair(coordinates: &Vec<(f64, f64)>) -> ((f64, f64), (f64, f64)) {
    let mut max_dist = 0.0;
    let mut pair = (coordinates[0], coordinates[1]);

    for i in 0..coordinates.len() {
        for j in i + 1..coordinates.len() {
            let dist = distance(&coordinates[i], &coordinates[j]);

            if dist > max_dist {
                max_dist = dist;
                pair = (coordinates[i], coordinates[j]);
            }
        }
    }

    pair
}

pub fn route_distance(route: &Vec<(f64, f64)>) -> f64 {
    let mut total = 0.0;

    for i in 1..route.len() {
        total += distance(&route[i - 1], &route[i]);
    }

    total
}
