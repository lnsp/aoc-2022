use std::{
    cmp::Ordering,
    fmt::Write,
    io::{self},
};

#[derive(Clone, Copy)]
enum Token {
    ListStart(),
    ListEnd(),
    Number(i32),
}

fn parse(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut numbuf = String::new();
    for i in 0..input.len() {
        match &input[i..i + 1] {
            "[" => tokens.push(Token::ListStart()),
            "]" => {
                if numbuf.len() > 0 {
                    tokens.push(Token::Number(numbuf.parse().unwrap()))
                };
                numbuf.clear();
                tokens.push(Token::ListEnd());
            }
            "," => {
                if numbuf.len() > 0 {
                    tokens.push(Token::Number(numbuf.parse().unwrap()))
                };
                numbuf.clear();
            }
            s => {
                numbuf.write_str(s).unwrap();
            }
        }
    }

    tokens
}

enum Item {
    Value(i32),
    List(Vec<usize>),
}

enum Stack {
    Reference(usize),
    Marker(),
}

fn build(tokens: &[Token]) -> Vec<Item> {
    let mut items = Vec::<Item>::new();
    let mut stack = Vec::<Stack>::new();

    for t in tokens {
        match t {
            Token::ListStart() => stack.push(Stack::Marker()),
            Token::ListEnd() => {
                // Go through stack and pop until StackStart
                let mut refs = Vec::new();
                while stack.len() > 0 {
                    let next = stack.pop().unwrap();
                    match next {
                        Stack::Reference(r) => {
                            refs.push(r);
                        }
                        Stack::Marker() => break,
                    }
                }
                refs.reverse();
                let list_ref = items.len();
                items.push(Item::List(refs));
                stack.push(Stack::Reference(list_ref));
            }
            Token::Number(n) => {
                let value_ref = items.len();
                items.push(Item::Value(*n));
                stack.push(Stack::Reference(value_ref));
            }
        }
    }

    items
}

fn compare(left: &[Item], lpos: usize, right: &[Item], rpos: usize) -> Ordering {
    match &left[lpos] {
        Item::Value(lv) => match &right[rpos] {
            Item::Value(rv) => lv.cmp(rv),
            Item::List(rr) => {
                if rr.len() > 0 {
                    match compare(left, lpos, right, rr[0]) {
                        Ordering::Equal => {
                            if rr.len() > 1 {
                                Ordering::Less
                            } else {
                                Ordering::Equal
                            }
                        }
                        any => any,
                    }
                } else {
                    Ordering::Greater
                }
            }
        },
        Item::List(lr) => match &right[rpos] {
            Item::Value(_) => {
                if lr.len() > 0 {
                    match compare(left, lr[0], right, rpos) {
                        Ordering::Equal => {
                            if lr.len() > 1 {
                                Ordering::Greater
                            } else {
                                Ordering::Equal
                            }
                        }
                        any => any,
                    }
                } else {
                    Ordering::Less
                }
            }
            Item::List(rr) => {
                let mut index = 0;
                while index < lr.len() && index < rr.len() {
                    match compare(left, lr[index], right, rr[index]) {
                        Ordering::Equal => (),
                        any => return any,
                    }
                    index += 1;
                }
                lr.len().cmp(&rr.len())
            }
        },
    }
}

fn main() -> io::Result<()> {
    let lines: Vec<String> = io::stdin().lines().map(|s| s.unwrap()).collect();
    let mut packets: Vec<Vec<Item>> = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let left_tokens = parse(lines[i].trim());
        let right_tokens = parse(lines[i + 1].trim());

        let left_items = build(&left_tokens);
        let right_items = build(&right_tokens);

        packets.push(left_items);
        packets.push(right_items);

        i += 3;
    }

    let div1 = build(&parse("[[2]]"));
    let div2 = build(&parse("[[6]]"));

    // sort
    packets.sort_by(|a, b| compare(a, a.len() - 1, b, b.len() - 1));

    // find dividers
    let (mut div1_pos, mut div2_pos) = (0, 0);
    for (i, p) in packets.iter().enumerate() {
        if compare(p, p.len() - 1, &div1, 2) == Ordering::Less {
            div1_pos = i
        } else if compare(p, p.len() - 1, &div2, 2) == Ordering::Less {
            div2_pos = i
        }
    }
    println!("{}", (div1_pos + 2) * (div2_pos + 3));

    Ok(())
}
