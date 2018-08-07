use std::collections::HashMap;
use std::string::ToString;

type OpFn = fn(Vec<String>) -> Box<ToString>;

fn main() {
    let input = "( + ( - 2 1 ) 2 ( + ( + 1 2 ) 4 ) )";

    let mut ops: HashMap<String, OpFn> = HashMap::new();
    ops.insert("+".to_string(), plus);
    ops.insert("-".to_string(), minus);


    println!("{:?}", evaluate(input, ops));
}

fn evaluate(input: &str, ops: HashMap<String, OpFn>) -> String {
    let mut stack: Vec<&str> = Vec::new();
    let tokens: Vec<&str> = tokenize(input);

    for token in tokens {
        if token == "(" {
            // do nothing
        } else if token == ")" {
            let mut sub_stack: Vec<String> = Vec::new();
            loop {
                let t = match stack.pop() {
                    None => break,
                    Some(t) => t
                };

                sub_stack.push(t.clone());
                if ops.contains_key(&t) {
                    break;
                }
            }

            let op = sub_stack.pop().unwrap();
            let mut ivals: Vec<i32> = Vec::new();
            let tstr = ops.get(&op).unwrap()(sub_stack);
            stack.push((*tstr).to_string());
        } else {
            stack.push(token.to_string());
        }
    }

    assert!(stack.len() == 1);
    stack.pop().unwrap()
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect().to_string()
}

fn plus(args: Vec<String>) -> Box<ToString> {
    let iargs: Vec<i32> = args.iter().map(|s| s.parse().unwrap()).collect();
    Box::new(iargs.iter().fold(0, |acc, &x| acc + x))
}

fn minus(args: Vec<String>) -> Box<ToString> {
    let mut iargs: Vec<i32> = args.iter().map(|s| s.parse().unwrap()).collect();
    let first = iargs.pop().unwrap();
    Box::new(iargs.iter().fold(first, |acc, &x| acc - x))
}

#[test]
fn minus_test() {
    let result = minus(vec!["1".to_string(), "1".to_string()]);
    assert_eq!((*result).to_string(), "0");
}
