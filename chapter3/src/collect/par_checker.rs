//"()(())"
use super::stack::Stack;
use std::collections::HashMap;


pub fn par_checker1(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars(){
        char_list.push(c);
    }

    let mut balance = true;
    let mut stack: Stack<char> = Stack::new();
    let mut index = 0;

    while index < char_list.len() && balance {
        let c = char_list[index];

        if '(' == c {
            stack.push(c);
        }else{
            if stack.is_empty(){
                balance = false;
            }else{
                stack.pop();
            }
        }
        index += 1;
    }
    return balance;
}

fn par_match(open : char, close : char) -> bool {
    let opens = "([{";
    let closes = ")]}";
    opens.find(open) == closes.find(close)
}

pub fn par_checker2(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars(){
        char_list.push(c);
    }

    let mut balance = true;
    let mut stack: Stack<char> = Stack::new();
    let mut index = 0;

    while index < char_list.len() && balance {
        let c = char_list[index];

        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        }else {
            if stack.is_empty(){
                balance = false;
            }else{
                let top = stack.pop().unwrap();
                if !par_match(c, top) {
                    balance = false;
                }
            }
        }
        index += 1;
    }
    return balance;
}

pub fn par_checker3(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars(){
        char_list.push(c);
    }

    let mut balance = true;
    let mut stack: Stack<char> = Stack::new();
    let mut index = 0;

    while index < char_list.len() && balance {
        let c = char_list[index];

        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        }
        
        if ')' == c || ']' == c || '}' == c {
            if stack.is_empty(){
                balance = false;
            }else{
                let top = stack.pop().unwrap();
                if !par_match(c, top) {
                    balance = false;
                }
            }
        }
        index += 1;
    }
    return balance;
}

pub fn devide_by_two(mut num : u32) -> String {
    let mut rem_stack : Stack<u32> = Stack::new();

    while num > 0 {
        let left = num % 2;
        rem_stack.push(left);
        num /= 2;
    }

    let mut bin_str = "".to_string();
    while !rem_stack.is_empty() {
        let str = rem_stack.pop().unwrap().to_string();
        bin_str += &str;
    }

    bin_str
}

pub fn base_conver(mut num : u32, base : u32) -> String {
    let mut rem_stack : Stack<char> = Stack::new();
    let digits = ['0', '1', '2', '3', '4', '5', '6', '7',
                            '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
    while num > 0 {
        let left = (num % base) as usize;
        rem_stack.push(digits[left]);
        num /= base;
    }

    let mut bin_str = "".to_string();
    while !rem_stack.is_empty() {
        let str = rem_stack.pop().unwrap().to_string();
        bin_str += &str;
    }

    bin_str
}

pub fn infix_to_postfix(infix : &str) -> Option<String> {
    if !par_checker3(infix) {  return None;}

    let mut prec = HashMap::new();
    prec.insert("(", 1);
    prec.insert(")", 1);
    prec.insert("+", 2);
    prec.insert("-", 2);
    prec.insert("*", 3);
    prec.insert("/", 3);

    let mut op_stack = Stack::new();
    let mut postfix = Vec::new();

    for token in infix.split_whitespace(){
        if ("A" <= token && token <= "Z") ||
            ("0" <= token && token <= "9"){
                postfix.push(token);
        }else if "(" == token {
            op_stack.push(token);
        }else if ")" == token {
            let mut top = op_stack.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = op_stack.pop().unwrap();
            }
        }else {
            while (!op_stack.is_empty()) &&
                (prec[op_stack.peek().unwrap()] >= prec[token]){
                    postfix.push(op_stack.pop().unwrap());
                }
            op_stack.push(token);
        }
    }

    while !op_stack.is_empty(){
        postfix.push(op_stack.pop().unwrap());
    }

    let mut postfix_str = " ".to_string();
    for c in postfix{
        postfix_str += &c.to_string();
        postfix_str += " ";
    }
    Some(postfix_str)
}