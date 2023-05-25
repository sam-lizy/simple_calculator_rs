use std::io::stdin;
fn main()->Result<(),Error>{
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input){
            Ok(_) => {
                let tokens = Calculator::parse(input);
                if tokens.is_err(){
                    println!("{:?}",tokens.err().unwrap());
                    continue;
                }
                let expr = Calculator::expression(tokens?);
                if let Some(v) = Calculator::evaluate(expr){
                    println!("{}",v)
                }
            },
            Err(error) => println!("error:{}",error)

        }
    }

}

struct Calculator{
    
}
impl Calculator{
    pub fn parse(expr:impl AsRef<str>)->Result<Vec<Token>,Error>{
        let expr = expr.as_ref();
        let chars = expr.chars();
        let mut tokens:Vec<Token> = Vec::new();//5 .
        let mut parens = Vec::new();
        let mut digits = Vec::new();
        //5.5+3
        for c in chars{
            match c {
                '0'..='9' => {
                    if !digits.is_empty(){
                        digits.push(c);
                    }else {
                        match tokens.last_mut() {
                            Some(Token::Number(n)) => {
                                *n = *n*10 + (c as u32 -48);
                            },
                            Some(Token::Point) => {
                                digits.push(c);
        
                            },
                            _ => {
                                let num = c as u32 -48;
                                tokens.push(Token::Number(num));
        
                            },
                        }
                    }

                },
                '.' => {
                    tokens.push(Token::Point);
                }
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push(c);
                },
                ')' => {
                    if let Some(p) = parens.pop() {
                        if p != '('{
                            return Err(Error::MisMatchParens);
                        }
                    }else {
                        return Err(Error::MisMatchParens);
                    }
                    tokens.push(Token::Bracket(')'));
                },
                '+' => {
                    Self::check_point(&mut tokens, &mut digits);
                    tokens.push(Token::Op(Operatopr::Add));
                },
                '-' => {
                    Self::check_point(&mut tokens, &mut digits);
                    tokens.push(Token::Op(Operatopr::Sub));
                },
                '*' => {
                    Self::check_point(&mut tokens, &mut digits);
                    tokens.push(Token::Op(Operatopr::Mul));
                },
                '/' => {
                    Self::check_point(&mut tokens, &mut digits);
                    tokens.push(Token::Op(Operatopr::Div));
                },
                ' ' | '\n'| '\r' => {},
                _ => return Err(Error::BadToken(c))

            }
        }
        if parens.len() > 0{
            return Err(Error::MisMatchParens);
        }
        Self::check_point(&mut tokens, &mut digits);
        Ok(tokens)
    }

    pub fn expression(mut tokens:Vec<Token>)->Vec<Token>{
        tokens.reverse();//(6+3)*5 => 63+5*
        let mut queue = Vec::new();
        // 6
        let mut stack = Vec::new();
        // ( +
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => queue.push(token),
                // Token::Point =>{
                //     stack.push(Token::Op(Operatopr::Add))
                // },
                Token::Prior => {
                    queue.push(stack.pop().unwrap());
                    queue.push(Token::Op(Operatopr::Add))
                },
                Token::Op(_) => {
                    if  !stack.is_empty() && stack[stack.len()-1] == Token::Bracket('('){
                        stack.push(token);
                    }else {
                        while !stack.is_empty() && stack[stack.len()-1] >= token && stack[stack.len()-1]!= Token::Bracket('('){
                            queue.push(stack.pop().unwrap())
                        };
                        stack.push(token);
                    }
                    
                },
                Token::Bracket('(') => {
                    stack.push(token);
                },
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack[stack.len()-1] != Token::Bracket('(') {
                        queue.push(stack.pop().unwrap());
                    }
                    let index = queue.iter().position(|x|{
                        *x == Token::Bracket('(')
                    });
                    if let Some(index) = index{
                        queue.remove(index);
                    }

                },
                _ =>{}
            }
            
        }
        while stack.len() > 0 {
            queue.push(stack.pop().unwrap());
        }
        queue
    }

    pub fn evaluate(mut tokens:Vec<Token>)->Option<f32>{
        tokens.reverse();
        let mut stack = Vec::new();
        while let Some(token) =tokens.pop() {
            match token {
                Token::Number(num) => stack.push(num as f32),
                Token::Op(Operatopr::Add) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left+right)
                },
                Token::Op(Operatopr::Sub) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left-right)
                },
                Token::Op(Operatopr::Mul) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(right*left)
                },
                Token::Op(Operatopr::Div) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left/right)
                },
                _ => {}
            }
        }
        if stack.len()>1{
            None
        }else {
            stack.pop()
        }

    }
    fn check_point(tokens:&mut Vec<Token>,digits:&mut Vec<char>){
        if !digits.is_empty() {
            let mut num = 0;
            let mut div_num = 1;
            digits.reverse();
            while let Some(n) = digits.pop(){
                let n = n as u32 -48;
                num = num *10 +n;
                div_num = div_num*10;
            }
            tokens.push(Token::Number(num));
            tokens.push(Token::Op(Operatopr::Div));
            tokens.push(Token::Number(div_num));
            tokens.push(Token::Prior)

        }
    }
}
#[derive(Debug,PartialEq, Eq,PartialOrd,Ord)]
pub enum Operatopr {
    Add,
    Sub, 
    Mul,
    Div
}

#[derive(Debug,PartialEq, Eq,PartialOrd,Ord)]
pub enum Token {
    Number(u32),
    Point,
    Op(Operatopr),
    Bracket(char),
    Prior
}

#[derive(Debug)]
pub enum Error {
    BadToken(char),
    MisMatchParens
}

#[test]

fn test(){
    use crate::Operatopr::*;
    assert!(Token::Op(Add)<Token::Op(Div))
}