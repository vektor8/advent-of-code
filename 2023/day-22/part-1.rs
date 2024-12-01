use std::vec;

type Point2D = (usize, usize);

type Line2D = (Point2D, Point2D);

type Point3D = (usize, usize, usize);

type Brick = (Point3D, Point3D);

fn intersect(a: Line2D, b: Line2D) -> bool {
    let (x1, y1) = a.0;
    let (x2, y2) = a.1;
    let (x3, y3) = b.0;
    let (x4, y4) = b.1;
    if x1.max(x2) < x3.min(x4) || x3.max(x4) < x1.min(x2) {
        return false;
    }
    if y1.max(y2) < y3.min(y4) || y3.max(y4) < y1.min(y2) {
        return false;
    }
    true
}

fn brick_in_xoy(a: Brick) -> Line2D {
    ((a.0 .0, a.0 .1), (a.1 .0, a.1 .1))
}

fn main() {
    let mut bricks: Vec<Brick> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|line| {
            let mut line = line.split('~');
            let start: Vec<usize> = line
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let end: Vec<usize> = line
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            assert!(start.len() == 3 && end.len() == 3);
            ((start[0], start[1], start[2]), (end[0], end[1], end[2]))
        })
        .collect();

    bricks.sort_by(|a, b| a.0 .2.cmp(&b.0 .2));
    let mut supports: Vec<Vec<usize>> = vec![vec![]; bricks.len()];
    for i in 0..bricks.len() {
        let mut max_z = 1;
        for j in 0..i {
            if intersect(brick_in_xoy(bricks[i]), brick_in_xoy(bricks[j])) {
                max_z = max_z.max(bricks[j].1 .2 + 1);
            }
        }
        bricks[i].1 .2 -= bricks[i].0 .2 - max_z;
        bricks[i].0 .2 = max_z;
    }
    bricks.sort_by(|a, b| a.0 .2.cmp(&b.0 .2));

    for (i, &a) in bricks.iter().enumerate() {
        for (j, &b) in bricks.iter().enumerate().take(i) {
            if !intersect(brick_in_xoy(a), brick_in_xoy(b)) || a.0 .2 != b.1 .2 + 1 {
                continue;
            }
            supports[j].push(i);
        }
    }
    let res = supports
        .iter()
        .enumerate()
        .filter(|(i, a)| {
            let others: Vec<usize> = supports
                .iter()
                .enumerate()
                .filter(|(j, _b)| j != i)
                .flat_map(|(_, b)| b)
                .copied()
                .collect();
            a.iter().all(|e| others.contains(e))
        })
        .count();
    println!("{}", res)
}
