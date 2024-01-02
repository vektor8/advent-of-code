#[derive(Debug)]
struct Line {
    x: f64,
    y: f64,
    c: f64,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
fn to_line(v: &[f64]) -> Line {
    let xdiff = v[3];
    let ydiff = v[4];
    let c = -v[1] * xdiff + v[0] * ydiff;
    Line {
        x: -ydiff,
        y: xdiff,
        c,
    }
}

fn intersection_point(a: &Line, b: &Line) -> Option<Point> {
    if -a.x / a.y == -b.x / b.y {
        return None;
    }
    let x = (b.c * a.y - a.c * b.y) / (a.x * b.y - a.y * b.x);
    Some(Point {
        x,
        y: (-a.c - a.x * x) / a.y,
    })
}

fn is_in_future(stone: &[f64], intersection: &Point) -> bool {
    ((intersection.x - stone[0]) * stone[3] >= 0.0)
        && ((intersection.y - stone[1]) * stone[4] >= 0.0)
}

fn inside_area(p: &Point, area: &[Point; 2]) -> bool {
    p.x >= area[0].x && p.x <= area[1].x && p.y >= area[0].y && p.y <= area[1].y
}
fn main() {
    let test_area = [
        Point {
            x: 200000000000000.0,
            y: 200000000000000.0,
        },
        Point {
            x: 400000000000000.0,
            y: 400000000000000.0,
        },
    ];

    // let test_area = [Point { x: 7.0, y: 7.0 }, Point { x: 27.0, y: 27.0 }];
    let stones: Vec<Vec<f64>> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|l| {
            let l = l.replace(" @", ",");
            l.split(", ").map(|s| s.parse::<f64>().unwrap()).collect()
        })
        .collect();

    let lines: Vec<Line> = stones.iter().map(|s| to_line(s)).collect();

    let mut count: u64 = 0;

    for (i, a) in lines.iter().enumerate() {
        for (j, b) in lines.iter().take(i).enumerate() {
            if let Some(p) = intersection_point(a, b) {
                if inside_area(&p, &test_area)
                    && is_in_future(&stones[i], &p)
                    && is_in_future(&stones[j], &p)
                {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
