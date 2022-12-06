use std::io;

fn main() -> io::Result<()>{
    let mut buffer = String::new();

    let mut items = Vec::<i32>::new();
    let mut cur_sum = 0;

    while io::stdin().read_line(&mut buffer)? > 0 {
        let buffer_trimmed = buffer.trim();
        if buffer_trimmed.len() < 1 {
            items.push(cur_sum);
            cur_sum = 0;
            continue;
        }
        cur_sum += match buffer_trimmed.parse::<i32>() {
            Ok(v) => v,
            Err(_) => panic!("failed to parse input")
        };
        buffer.clear();
    }
    items.push(cur_sum);

    items.sort_by(|a, b| b.cmp(a));
    println!("{}", items[..3].iter().sum::<i32>());
    Ok(())
}
