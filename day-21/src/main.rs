fn main() {
    let mut die = (1..=100).into_iter().cycle();
    let mut rolls = 0;

    let mut p1_pos = 7;
    let mut p2_pos = 3;
    let mut p1_score = 0;
    let mut p2_score = 0;

    loop {
        for _ in 0..3 {
            p1_pos += die.next().unwrap();
            rolls += 1;
        }
        p1_pos = (p1_pos - 1) % 10 + 1;
        p1_score += p1_pos;

        if p1_score >= 1_000 {
            break;
        }

        for _ in 0..3 {
            p2_pos += die.next().unwrap();
            rolls += 1;
        }
        p2_pos = (p2_pos - 1) % 10 + 1;
        p2_score += p2_pos;

        if p2_score >= 1_000 {
            break;
        }
    }

    println!("part1 = {}", p1_score.min(p2_score) * rolls);
}
