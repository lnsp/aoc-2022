use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut n_stacks = 0;
    let mut stacks_initial = Vec::<String>::new();
    while io::stdin().read_line(&mut buf)? > 0 {
        // If the line does not contain any bracket, break
        if !buf.contains("[") {
            n_stacks = buf.trim().split_whitespace().count();
            break;
        }
        stacks_initial.push(buf.clone());
        buf.clear();
    }

    // set up stacks
    let mut stacks = Vec::<Vec<u8>>::new();
    for _ in 0..n_stacks {
        stacks.push(Vec::<u8>::new())
    }
    // reverse initial line setup
    stacks_initial.reverse();
    for line in stacks_initial {
        let line_bytes = line.as_bytes();
        // position of stack i is always 1 + i * 4
        for n in 0..n_stacks {
            let p = 1 + n * 4;
            if line_bytes[p] != ' ' as u8 {
                stacks[n].push(line_bytes[p]);
            }
        }
    }

    io::stdin().read_line(&mut buf);
    buf.clear();

    // start performing ops
    while io::stdin().read_line(&mut buf)? > 0 {
        let substr: Vec<&str> = buf.trim().split_whitespace().collect();
        let n = substr[1].parse::<usize>().unwrap();
        let i = substr[3].parse::<usize>().unwrap();
        let j = substr[5].parse::<usize>().unwrap();

        let mut tmp = Vec::new();

        for _ in 0..n {
            let m = match stacks[i-1].pop() {
                Some(v) => v,
                None => panic!("fuck!"),
            };
            tmp.push(m);
        }
        while !tmp.is_empty()  {
            stacks[j-1].push(match tmp.pop() { Some(v) => v, None => 0 });
        }

        buf.clear();
    }

    let s: String = stacks.iter().map(|s| s[s.len()-1] as char).collect();

    println!("{}", s);

    Ok(())
}
