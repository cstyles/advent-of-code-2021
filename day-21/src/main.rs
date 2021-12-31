use std::collections::HashMap;

fn main() {
    let (p1, p2) = (7, 3);
    part1(p1, p2);
    part2(p1, p2);
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    p1_score: u32,
    p2_score: u32,
    p1_pos: u32,
    p2_pos: u32,
    p1_next: bool,
}

static ROLLS: [(u32, u32, u32); 27] = [
    (1, 1, 1),
    (1, 1, 2),
    (1, 1, 3),
    (1, 2, 1),
    (1, 2, 2),
    (1, 2, 3),
    (1, 3, 1),
    (1, 3, 2),
    (1, 3, 3),
    (2, 1, 1),
    (2, 1, 2),
    (2, 1, 3),
    (2, 2, 1),
    (2, 2, 2),
    (2, 2, 3),
    (2, 3, 1),
    (2, 3, 2),
    (2, 3, 3),
    (3, 1, 1),
    (3, 1, 2),
    (3, 1, 3),
    (3, 2, 1),
    (3, 2, 2),
    (3, 2, 3),
    (3, 3, 1),
    (3, 3, 2),
    (3, 3, 3),
];

fn part2(p1_pos: u32, p2_pos: u32) {
    // Key: Current game state
    // Value: (number of sub-games in which P1 wins, number of sub-games in which P2 wins)
    let mut states: HashMap<State, (u64, u64)> = Default::default();

    for p1_score in (0..=20).rev() {
        let mut state = State {
            p1_score,
            ..State::default()
        };

        for p2_score in (0..=20).rev() {
            state.p2_score = p2_score;

            for p1_pos in 1..=10 {
                state.p1_pos = p1_pos;

                for p2_pos in 1..=10 {
                    state.p2_pos = p2_pos;

                    let mut p1_wins = 0;
                    let mut p2_wins = 0;
                    state.p1_next = true;
                    for roll in ROLLS {
                        let sum = roll.0 + roll.1 + roll.2;
                        let new_p1_pos = (p1_pos + sum - 1) % 10 + 1;
                        let new_p1_score = p1_score + new_p1_pos;

                        if new_p1_score >= 21 {
                            p1_wins += 1;
                        } else {
                            let lookup_state = State {
                                p1_pos: new_p1_pos,
                                p1_score: new_p1_score,
                                p1_next: false,
                                ..state
                            };
                            let (p1_wins_lookup, p2_wins_lookup) =
                                states.get(&lookup_state).unwrap();

                            p1_wins += p1_wins_lookup;
                            p2_wins += p2_wins_lookup;
                        }
                    }

                    states.insert(state, (p1_wins, p2_wins));

                    let mut p1_wins = 0;
                    let mut p2_wins = 0;
                    state.p1_next = false;
                    for roll in ROLLS {
                        let sum = roll.0 + roll.1 + roll.2;
                        let new_p2_pos = (p2_pos + sum - 1) % 10 + 1;
                        let new_p2_score = p2_score + new_p2_pos;

                        if new_p2_score >= 21 {
                            p2_wins += 1;
                        } else {
                            let lookup_state = State {
                                p2_pos: new_p2_pos,
                                p2_score: new_p2_score,
                                p1_next: true,
                                ..state
                            };
                            let (p1_wins_lookup, p2_wins_lookup) =
                                states.get(&lookup_state).unwrap();

                            p1_wins += p1_wins_lookup;
                            p2_wins += p2_wins_lookup;
                        }
                    }

                    states.insert(state, (p1_wins, p2_wins));
                }
            }
        }
    }

    let start_state = State {
        p1_pos,
        p2_pos,
        p1_next: true,
        ..State::default()
    };
    println!("part2 = {:?}", states.get(&start_state).unwrap());
}

fn part1(mut p1_pos: u32, mut p2_pos: u32) {
    let mut die = (1..=100).into_iter().cycle();
    let mut rolls = 0;

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
