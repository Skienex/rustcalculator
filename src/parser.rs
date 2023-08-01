use crate::error::{Error, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Num(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    Eof,
}

impl Token {
    fn is_unary_op(&self) -> bool {
        matches!(self, Token::Plus | Token::Minus)
    }

    fn is_binary_op(&self) -> bool {
        matches!(
            self,
            Token::Plus | Token::Minus | Token::Star | Token::Slash
        )
    }

    fn precedence(&self) -> usize {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Star | Token::Slash => 2,
            _ => panic!("Invalid operator"),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Plus(Box<Expr>),
    Minus(Box<Expr>),
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
    Mul { lhs: Box<Expr>, rhs: Box<Expr> },
    Div { lhs: Box<Expr>, rhs: Box<Expr> },
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Num(num) => *num,
            Expr::Plus(expr) => expr.eval(),
            Expr::Minus(expr) => -expr.eval(),
            Expr::Add { lhs, rhs } => lhs.eval() + rhs.eval(),
            Expr::Sub { lhs, rhs } => lhs.eval() - rhs.eval(),
            Expr::Mul { lhs, rhs } => lhs.eval() * rhs.eval(),
            Expr::Div { lhs, rhs } => lhs.eval() / rhs.eval(),
        }
    }
}

pub fn parse(input: &str) -> Result<Expr> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                tokens.push(Token::LeftParen);
            }
            ')' => {
                tokens.push(Token::RightParen);
            }
            '+' => {
                tokens.push(Token::Plus);
            }
            '-' => {
                tokens.push(Token::Minus);
            }
            '*' => {
                tokens.push(Token::Star);
            }
            '/' => {
                tokens.push(Token::Slash);
            }
            'i' => {
                if chars.next().unwrap() != 'n' || chars.next().unwrap() != 'f' {
                    return Err(Error::InvalidIdent());
                }
                tokens.push(Token::Num(f64::INFINITY));
            }
            c if c.is_ascii_digit() || c == '.' => {
                let mut buf = String::new();
                buf.push(c);
                while let Some(ac) = chars.peek() {
                    let ac = *ac;
                    if ac.is_ascii_digit() || ac == '.' {
                        chars.next();
                        buf.push(ac);
                        continue;
                    }
                    break;
                }
                let Ok(num) = buf.parse() else {
                    return Err(Error::InvalidNumber(buf));
                };
                tokens.push(Token::Num(num));
            }
            c if c.is_whitespace() => {}
            c => return Err(Error::UnexpectedChar(c)),
        }
    }
    tokens.push(Token::Eof);
    prat(&tokens)
}

struct State<'a> {
    tokens: std::iter::Peekable<std::slice::Iter<'a, Token>>,
}

impl State<'_> {
    fn peek(&mut self) -> Token {
        **self.tokens.peek().unwrap()
    }

    fn eat(&mut self) {
        self.tokens.next().unwrap();
    }
}

fn prat(tokens: &[Token]) -> Result<Expr> {
    let mut state = State {
        tokens: tokens.iter().peekable(),
    };
    parse_expr(&mut state, &Token::Eof)
}

fn parse_expr(state: &mut State<'_>, end_token: &Token) -> Result<Expr> {
    let next = state.peek();
    let left = parse_unary(state, next)?;
    let op = state.peek();
    if &op == end_token {
        state.eat(); // ???
        return Ok(left);
    }
    if !op.is_binary_op() {
        return Err(Error::InvalidBinOp());
    }
    parse_binary(state, left, end_token)
}

fn parse_unary(state: &mut State<'_>, left: Token) -> Result<Expr> {
    if left.is_unary_op() {
        state.eat();
        let next = state.peek();
        return Ok(apply_unary(left, parse_unary(state, next)?));
    }
    if let Token::LeftParen = left {
        state.eat();
        return parse_expr(state, &Token::RightParen);
    }
    if let Token::Num(value) = left {
        state.eat();
        return Ok(Expr::Num(value));
    }
    Err(Error::InvalidUnaryOp())
}

fn parse_binary(state: &mut State<'_>, left: Expr, end_token: &Token) -> Result<Expr> {
    let op = state.peek();
    state.eat();
    let next = state.peek();
    let right = parse_unary(state, next)?;
    let next = state.peek();
    if &next == end_token {
        state.eat(); // ???
        return Ok(apply_binary(op, left, right));
    };
    if !next.is_binary_op() {
        return Err(Error::InvalidBinOp());
    }
    if op.precedence() < next.precedence() {
        return Ok(apply_binary(
            op,
            left,
            parse_binary(state, right, end_token)?,
        ));
    }
    parse_binary(state, apply_binary(op, left, right), end_token)
}

fn apply_unary(op: Token, expr: Expr) -> Expr {
    match op {
        Token::Plus => Expr::Plus(Box::new(expr)),
        Token::Minus => Expr::Minus(Box::new(expr)),
        _ => panic!("Illegal unary apply"),
    }
}

fn apply_binary(op: Token, lhs: Expr, rhs: Expr) -> Expr {
    match op {
        Token::Plus => Expr::Add {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        Token::Minus => Expr::Sub {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        Token::Star => Expr::Mul {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        Token::Slash => Expr::Div {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        _ => panic!("Illegal binary apply"),
    }
}
