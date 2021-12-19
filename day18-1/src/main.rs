use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Clone, Debug)]
enum Elem {
    Open,
    Close,
    Num(i32),
}
type Expr = Vec<Elem>;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut expr = parse_expr(lines.next().unwrap().unwrap());
    for _line in lines {
        let line = _line.unwrap();

        let mut new_expr = parse_expr(line);
        expr = add(&expr, &mut new_expr);
    }

    println!("{:?}", eval(&expr).unwrap());
}

fn parse_expr(str: String) -> Expr {
    let mut expr = Expr::new();

    for c in str.replace(",", "").chars() {
        expr.push(match c {
            '[' => Elem::Open,
            ']' => Elem::Close,
            _ => Elem::Num(c.to_string().parse::<i32>().unwrap()),
        });
    }

    expr
}

fn add(e1: &Expr, e2: &Expr) -> Expr {
    let mut expr = e1.clone();
    expr.extend(e2.clone());
    expr.insert(0, Elem::Open);
    expr.push(Elem::Close);

    loop {
        let (e, changed) = explode(&expr);
        expr = e;
        if changed {
            continue;
        }

        let (e, changed) = split(&expr);
        expr = e;
        if changed {
            continue;
        }

        break;
    }

    expr
}

fn explode(expr: &Expr) -> (Expr, bool) {
    let mut new_expr = expr.clone();

    let mut explode_index: usize = 0;
    let mut is_explode = false;
    let mut depth = 0;
    for i in 0..expr.len() - 1 {
        match expr[i] {
            Elem::Open => {
                depth += 1;
            }
            Elem::Close => {
                depth -= 1;
            }
            Elem::Num(_) => {
                if matches!(expr[i + 1], Elem::Num(_)) && depth > 4 {
                    is_explode = true;
                    explode_index = i;
                    break;
                }
            }
        };
    }

    if is_explode {
        for i in (0..explode_index).rev() {
            if let Elem::Num(n) = expr[i] {
                if let Elem::Num(left_n) = expr[explode_index] {
                    new_expr[i] = Elem::Num(n + left_n);
                    break;
                }
            }
        }
        for i in explode_index + 2..expr.len() {
            if let Elem::Num(n) = expr[i] {
                if let Elem::Num(right_n) = expr[explode_index + 1] {
                    new_expr[i] = Elem::Num(n + right_n);
                    break;
                }
            }
        }
        new_expr.remove(explode_index);
        new_expr.remove(explode_index);
        new_expr.remove(explode_index);
        new_expr[explode_index - 1] = Elem::Num(0);
    }

    (new_expr, is_explode)
}

fn split(expr: &Expr) -> (Expr, bool) {
    let mut new_expr = expr.clone();

    for i in 0..expr.len() - 1 {
        match expr[i] {
            Elem::Num(n) => {
                if n > 9 {
                    new_expr.remove(i);
                    new_expr.insert(i, Elem::Close);
                    new_expr.insert(i, Elem::Num((n as f64 / 2.0).ceil() as i32));
                    new_expr.insert(i, Elem::Num((n as f64 / 2.0).floor() as i32));
                    new_expr.insert(i, Elem::Open);

                    return (new_expr, true);
                }
            }
            _ => (),
        };
    }

    return (new_expr, false);
}

fn eval(expr: &Expr) -> Result<i32, ()> {
    let mut new_expr = expr.clone();
    while new_expr.len() != 1 {
        for i in 0..new_expr.len() - 1 {
            if let Elem::Num(n1) = new_expr[i] {
                if let Elem::Num(n2) = new_expr[i + 1] {
                    new_expr.remove(i);
                    new_expr.remove(i);
                    new_expr.remove(i);
                    new_expr[i - 1] = Elem::Num(n1 * 3 + n2 * 2);
                    break;
                }
            }
        }
    }

    if let Elem::Num(n) = new_expr[0] {
        Ok(n)
    } else {
        Err(())
    }
}
