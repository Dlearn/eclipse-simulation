use std::cmp;

use crate::{ship::Ship, PRINT_LOGS};

#[derive(Default)]
pub struct Battle {
    // 1 is targetted before 2, etc.
    pub att_type1: Ship,
    pub att_type1_count: u8,
    pub att_type1_acc_dmg: u8, // damage is accumulated until a ship is destroyed
    pub att_type2: Ship,
    pub att_type2_count: u8,
    pub att_type2_acc_dmg: u8, // damage is accumulated until a ship is destroyed
    pub def_type1: Ship,
    pub def_type1_count: u8,
    pub def_type1_acc_dmg: u8, // damage is accumulated until a ship is destroyed
    pub att_won: bool,
}

impl Battle {
    pub fn new(
        att_type1: Ship,
        att_type1_count: u8,
        def_type1: Ship,
        def_type1_count: u8,
    ) -> Battle {
        Battle {
            att_type1,
            att_type1_count,
            def_type1,
            def_type1_count,
            ..Default::default()
        }
    }

    pub fn resolve(&mut self) {
        while self.att_type1_count > 0 && self.def_type1_count > 0 {
            if self.def_type1.initiative >= self.att_type1.initiative {
                self.def_shoot_att();
                self.att_shoot_def();
            } else {
                self.att_shoot_def();
                self.def_shoot_att();
            }
        }
        if self.att_type1_count == 0 {
            if *PRINT_LOGS {
                println!("Defender {} wins", self.def_type1.name);
                println!("================================================")
            }
            self.att_won = false;
        } else {
            if *PRINT_LOGS {
                println!("Attacker {} wins", self.att_type1.name);
                println!("================================================")
            }
            self.att_won = true;
        }
    }

    fn att_shoot_def(&mut self) {
        let mut dmg = 0;
        for _n in 0..self.att_type1_count {
            dmg += self.att_type1.roll_against(&self.def_type1);
        }
        self.def_type1_acc_dmg += dmg % (self.def_type1.hull + 1);
        self.def_type1_count -= cmp::min(
            self.def_type1_count,
            self.def_type1_acc_dmg / (self.def_type1.hull + 1),
        );
        if *PRINT_LOGS {
            println!(
                "{} shoots {} for {} damage. {} leftover def ships. {} leftover damage.",
                self.att_type1.name,
                self.def_type1.name,
                dmg,
                self.def_type1_count,
                self.def_type1_acc_dmg
            );
        }
    }

    fn def_shoot_att(&mut self) {
        let mut dmg = 0;
        for _n in 0..self.def_type1_count {
            dmg += self.def_type1.roll_against(&self.att_type1);
        }
        self.att_type1_acc_dmg += dmg % (self.att_type1.hull + 1);
        self.att_type1_count -= cmp::min(
            self.att_type1_count,
            self.att_type1_acc_dmg / (self.att_type1.hull + 1),
        );
        if *PRINT_LOGS {
            println!(
                "{} shoots {} for {} damage. {} leftover att ships. {} leftover damage.",
                self.def_type1.name,
                self.att_type1.name,
                dmg,
                self.att_type1_count,
                self.att_type1_acc_dmg
            );
        }
    }
}
