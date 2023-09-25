mod battle;
mod ship;

use lazy_static::lazy_static;

use crate::battle::{Battle, BattleBuilder};
use crate::ship::Ship;

lazy_static! {
    static ref ITERATIONS: u16 = 10000;
    static ref PRINT_LOGS: bool = false;
}

fn main() {
    let mut att_win = 0.0;

    for _n in 0..*ITERATIONS as u16 {
        let dread = Ship::dreadnaught();
        let ancient = Ship::base_ancient();
        let interceptor = Ship::interceptor();
        let battle_builder = BattleBuilder::new(dread, 1, ancient, 1);
        let mut bat = battle_builder.set_att_type2(interceptor, 1).build();
        bat.resolve();
        if bat.att_won {
            att_win += 1.0;
        }
    }
    // for _n in 0..*ITERATIONS as u16 {
    //     let dread = Ship::dreadnaught();
    //     let ancient = Ship::base_ancient();
    //     let mut bat = Battle::new(dread, 1, ancient, 1);
    //     bat.resolve();
    //     if bat.att_won {
    //         att_win += 1.0;
    //     }
    // }
    println!("Attacker wins: {}, total: {}", att_win, *ITERATIONS as f64);
    println!("Attacker win statistic: {}", att_win / *ITERATIONS as f64);
}
