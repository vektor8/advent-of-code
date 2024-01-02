use z3::{ast::Ast, ast::Int, Config, Context, SatResult, Solver};

#[derive(Clone, Copy)]
struct Stone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

struct Z3Stone<'a> {
    x: Int<'a>,
    y: Int<'a>,
    z: Int<'a>,
    vx: Int<'a>,
    vy: Int<'a>,
    vz: Int<'a>,
}

impl Stone {
    fn into_z3_stone(self, ctx: &Context) -> Z3Stone<'_> {
        Z3Stone {
            x: Int::from_i64(ctx, self.x as i64),
            y: Int::from_i64(ctx, self.y as i64),
            z: Int::from_i64(ctx, self.z as i64),
            vx: Int::from_i64(ctx, self.vx as i64),
            vy: Int::from_i64(ctx, self.vy as i64),
            vz: Int::from_i64(ctx, self.vz as i64),
        }
    }
}

fn main() {
    let stones: Vec<Stone> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|l| {
            let l = l.replace(" @", ",");
            let v: Vec<f64> = l.split(", ").map(|s| s.parse::<f64>().unwrap()).collect();
            Stone {
                x: v[0],
                y: v[1],
                z: v[2],
                vx: v[3],
                vy: v[4],
                vz: v[5],
            }
        })
        .collect();

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let ans_x = Int::new_const(&ctx, "ans_x");
    let ans_y = Int::new_const(&ctx, "ans_y");
    let ans_z = Int::new_const(&ctx, "ans_z");
    let ans_vx = Int::new_const(&ctx, "ans_vx");
    let ans_vy = Int::new_const(&ctx, "ans_vy");
    let ans_vz = Int::new_const(&ctx, "ans_vz");
    let zero = Int::from_i64(&ctx, 0);

    let stones: Vec<Z3Stone> = stones.iter().map(|&s| s.into_z3_stone(&ctx)).collect();

    for (i, s) in stones.into_iter().enumerate() {
        let t = Int::new_const(&ctx, format!("t{}", i));
        solver.assert(&t.gt(&zero));
        solver.assert(&(&ans_x + &ans_vx * &t)._eq(&(s.x + s.vx * &t)));
        solver.assert(&(&ans_y + &ans_vy * &t)._eq(&(s.y + s.vy * &t)));
        solver.assert(&(&ans_z + &ans_vz * &t)._eq(&(s.z + s.vz * &t)));
    }
    if let (SatResult::Sat, Some(model)) = (solver.check(), solver.get_model()) {
        let res = model.eval(&(ans_x + ans_y + ans_z), true).unwrap();
        println!("{}", res);
    };
}
