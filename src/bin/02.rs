use std::io;

fn main() -> io::Result<()> {
    let score_action = | c | match c {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("unknown char")
    };
    //A Rock, B Paper, C scissors
    //X       Y        Z
    let score_outcome = | a, b | match (a, b) {
        ('A', 'X') => 3,
        ('A', 'Y') => 6,
        ('A', 'Z') => 0,
        ('B', 'X') => 0,
        ('B', 'Y') => 3,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        ('C', 'Y') => 0,
        ('C', 'Z') => 3,
        (_, _) => panic!("unknown chars")
    };
    let expected_score = | a | match a {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!("unknown char")
    };

    let mut buf = String::new();
    let mut scores = Vec::<i32>::new();
    while io::stdin().read_line(&mut buf)? > 0 {
        let bytes = buf.trim().as_bytes();

        let action = ['X', 'Y', 'Z'].iter().find(| x | expected_score(bytes[2] as char) == score_outcome(bytes[0] as char, **x)).unwrap();

        let total_score = score_action(*action) +
        score_outcome(bytes[0] as char, *action);

        scores.push(total_score);

        buf.clear();
    }

    println!("{}", scores.iter().sum::<i32>());

    Ok(())
}