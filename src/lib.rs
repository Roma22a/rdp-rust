fn hypotenuse(start: &[f64; 2], end: &[f64; 2]) -> f64 {
    let (dx, dy) = (start[0] - end[0], start[1] - end[1]);
    dx.hypot(dy)
}

pub fn perpendicular_distance(
    point: &[f64; 2],
    start: &[f64; 2],
    end: &[f64; 2]
) -> f64 {
    if start == end {
        return hypotenuse(*&point, *&start);
    } else {

        let numerator = (
            (end[0] - start[0]) * (start[1] - point[1]) -
            (start[0] - point[0]) * (end[1] - start[1])
        ).abs();

        let denominator =  hypotenuse(start, end);

        numerator / denominator
    }
}

pub fn rdp(points: &[[f64; 2]], tolerance: &f64) -> Vec<[f64; 2]> {
    let mut dmax: f64 = 0.0;
    let mut index: usize = 0;
    for (i, _) in points.iter().enumerate().take(points.len() - 1).skip(1) {
        let distance = perpendicular_distance(
            &points[i], 
            &*points.first().unwrap(), 
            &*points.last().unwrap()
        );

        if distance > dmax {
            index = i;
            dmax = distance;
        }
    }

    if dmax > *tolerance {
        let mut intermediate = rdp(&points[..index + 1], &*tolerance);
        intermediate.pop();
        intermediate.extend_from_slice(&rdp(&points[index..], &*tolerance));
        intermediate
    } else {
        vec![*points.first().unwrap(), *points.last().unwrap()]
    }
    
}

#[cfg(test)]
mod tests {
    use super::{rdp, hypotenuse, perpendicular_distance};
    #[test]
    fn test_distance() {
        let start = [0.0, 0.0];
        let end = [3.0, 4.0];
        assert_eq!(hypotenuse(&start, &end), 5.);
    }
    #[test]
    fn test_perpendicular_distance() {
        let point = [1.0, 1.0];
        let start = [1.0, 2.0];
        let end = [3.0, 4.0];
        assert_eq!(perpendicular_distance(&point, &start, &end),
                   0.7071067811865475);
    }
    #[test]
    fn test_rdp() {
        let points = vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [17.3, 3.2], [27.8, 0.1]];
        let foo: Vec<_> = rdp(&points, &1.0);
        assert_eq!(foo, vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [27.8, 0.1]]);
    }
    #[test]
    fn test_array_conversion() {
        let original = vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [17.3, 3.2], [27.8, 0.1]];
        let converted: Vec<_> = original.into();
        assert_eq!(converted,
                   vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [17.3, 3.2], [27.8, 0.1]]);
    }
}