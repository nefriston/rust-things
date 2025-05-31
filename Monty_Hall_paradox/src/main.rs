use rand::{prelude::*, random_bool};
fn main() {
    let mut swithch_wins = 0;
    let mut stay_wins = 0;
    for _ in 0..1_000_000 {
        if monty_hall_problem() {
            swithch_wins += 1;
        } else {
            stay_wins += 1;
        }
    }
    println!("Switch wins: {}", swithch_wins);
    println!("Stay wins: {}", stay_wins);
}
fn monty_hall_problem()->bool {
    let mut rng =rand::rng();
    let right_answer=rng.gen_range(0..3);
    let player_choice = rng.random_range(0..3);
    if(random_bool(0.5)){
        return if(player_choice == right_answer) {
            false
        } else {
            monty_hall_problem()
        };
    } else {
        let false_door =(0..3).find(|&i| i!=player_choice && i!=right_answer).unwrap();
        let new_choice =(0..3).find(|&i| i != player_choice && i != false_door).unwrap();
        return if(new_choice == right_answer) {
            true
        } else {
            monty_hall_problem()
        };
    }
}

