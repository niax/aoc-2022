use aoc2022::commons::io::load_argv_lines;
use std::collections::HashMap;
use std::error::Error;

peg::parser! {
    grammar expr_parser() for str {
        rule variable() -> String
            = s:$(['a'..='z']+) {
                s.to_string()
            }

        rule number() -> isize
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule val_num() -> Expression
            = n:number() { Expression::Value(n) }

        rule val_var() -> Expression
            = v:variable() { Expression::Variable(v) }

        rule value() -> Expression
            = val_num() / val_var()

        rule expression() -> Expression =
            precedence!{
                l:value() " + " r:value() { Expression::Add(Box::new(l), Box::new(r)) }
                l:value() " - " r:value() { Expression::Sub(Box::new(l), Box::new(r)) }
                l:value() " * " r:value() { Expression::Mul(Box::new(l), Box::new(r)) }
                l:value() " / " r:value() { Expression::Div(Box::new(l), Box::new(r)) }
                l:value() { l }
            }

        pub rule assignment() -> (String, Expression)
            = v:variable() ": " e:expression() { (v, e) }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Variable(String),
    Value(isize),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn as_expr_string(&self) -> String {
        match self {
            Self::Value(u) => u.to_string(),
            Self::Variable(v) => v.to_string(),
            Self::Add(l, r) => format!("({} + {})", l.as_expr_string(), r.as_expr_string()),
            Self::Sub(l, r) => format!("({} - {})", l.as_expr_string(), r.as_expr_string()),
            Self::Mul(l, r) => format!("({} * {})", l.as_expr_string(), r.as_expr_string()),
            Self::Div(l, r) => format!("({} / {})", l.as_expr_string(), r.as_expr_string()),
        }
    }

    fn is_value(&self) -> bool {
        match self {
            Self::Value(_) => true,
            _ => false,
        }
    }

    fn value(&self) -> isize {
        match self {
            Self::Value(v) => *v,
            _ => panic!("Called value() on non-value"),
        }
    }

    pub fn reverse(&self, want: isize) -> isize {
        use Expression::*;
        match self {
            Value(u) => {
                if *u != want {
                    panic!("Nope!");
                }
                want
            }
            Variable(_) => want,
            Add(l, r) => {
                if l.is_value() {
                    r.reverse(want - l.value())
                } else {
                    l.reverse(want - r.value())
                }
            }
            Sub(l, r) => {
                if l.is_value() {
                    r.reverse(l.value() - want)
                } else {
                    // want = left - right
                    l.reverse(want + r.value())
                }
            }
            Mul(l, r) => {
                if l.is_value() {
                    r.reverse(want / l.value())
                } else {
                    l.reverse(want / r.value())
                }
            }
            Div(l, r) => {
                if l.is_value() {
                    r.reverse(l.value() / want)
                } else {
                    l.reverse(want * r.value())
                }
            }
        }
    }

    pub fn resolve(&self, variables: &HashMap<String, Self>) -> Self {
        match self {
            Self::Value(u) => Self::Value(*u),
            Self::Variable(v) => match variables.get(&v.to_string()) {
                Some(var) => var.resolve(variables),
                None => Self::Variable(v.to_string()),
            },
            Self::Add(l, r) => {
                let l_val = l.resolve(variables);
                let r_val = r.resolve(variables);

                if l_val.is_value() && r_val.is_value() {
                    Self::Value(l_val.value() + r_val.value())
                } else {
                    Self::Add(Box::new(l_val), Box::new(r_val))
                }
            }
            Self::Sub(l, r) => {
                let l_val = l.resolve(variables);
                let r_val = r.resolve(variables);

                if l_val.is_value() && r_val.is_value() {
                    Self::Value(l_val.value() - r_val.value())
                } else {
                    Self::Sub(Box::new(l_val), Box::new(r_val))
                }
            }
            Self::Mul(l, r) => {
                let l_val = l.resolve(variables);
                let r_val = r.resolve(variables);

                if l_val.is_value() && r_val.is_value() {
                    Self::Value(l_val.value() * r_val.value())
                } else {
                    Self::Mul(Box::new(l_val), Box::new(r_val))
                }
            }
            Self::Div(l, r) => {
                let l_val = l.resolve(variables);
                let r_val = r.resolve(variables);

                if l_val.is_value() && r_val.is_value() {
                    Self::Value(l_val.value() / r_val.value())
                } else {
                    Self::Div(Box::new(l_val), Box::new(r_val))
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = load_argv_lines().collect::<Result<_, _>>()?;
    let input = input
        .iter()
        .map(|s| expr_parser::assignment(&s))
        .collect::<Result<Vec<_>, _>>()?;
    let mut variables = HashMap::new();
    for (k, v) in input {
        variables.insert(k, v);
    }
    let p1 = variables
        .get(&"root".to_string())
        .expect("expect root")
        .resolve(&variables);
    println!("{}", p1.value());

    variables.remove(&"humn".to_string());
    let p2 = variables.get(&"root".to_string()).expect("expect root");
    if let Expression::Add(l, r) = p2 {
        let l_val = l.resolve(&variables);
        let r_val = r.resolve(&variables);

        let (wanted, expr) = if l_val.is_value() {
            (l_val.value(), r_val)
        } else {
            (r_val.value(), l_val)
        };

        println!("{:?}", expr.reverse(wanted));
    } else {
        panic!("shortcut doesn't work");
    }

    Ok(())
}
