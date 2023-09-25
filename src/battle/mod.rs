mod ship_type_def;

use ship_type_def::ShipType;

use std::cmp;

use crate::{ship::Ship, PRINT_LOGS};

#[derive(Default)]
pub struct Battle {
    pub att_ships: Vec<ShipType>,
    pub def_ships: Vec<ShipType>,
    pub att_won: bool,
}

impl Battle {
    pub fn new(att_ships: Vec<ShipType>, def_ships: Vec<ShipType>) -> Battle {
        Battle {
            att_ships,
            def_ships,
            ..Default::default()
        }
    }

    pub fn resolve(&mut self) {
        // While attackers and defenders have forces
        while self.att_ships.into_iter().any(|ship| ship.count > 0)
            && self.def_ships.into_iter().any(|ship| ship.count > 0)
        {
            let mut all_ships = self.att_ships.clone();
            all_ships.append(&mut self.def_ships.clone());
            all_ships.sort_by(|a, b| a.ship_type.initiative.cmp(&b.ship_type.initiative))
        }
        //     if self.def_type1.initiative >= self.att_type1.initiative {
        //         self.def_shoot_att();
        //         self.att_shoot_def();
        //     } else {
        //         self.att_shoot_def();
        //         self.def_shoot_att();
        //     }
        // }
        // if self.att_type1_count == 0 {
        //     if *PRINT_LOGS {
        //         println!("Defender {} wins", self.def_type1.name);
        //         println!("================================================")
        //     }
        //     self.att_won = false;
        // } else {
        //     if *PRINT_LOGS {
        //         println!("Attacker {} wins", self.att_type1.name);
        //         println!("================================================")
        //     }
        //     self.att_won = true;
        // }
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

#[derive(Default)]
pub struct BattleBuilder {
    att_type1: Ship,
    att_type1_count: u8,
    def_type1: Ship,
    def_type1_count: u8,
    att_type2: Ship,
    att_type2_count: u8,
}

impl BattleBuilder {
    pub fn new(
        att_type1: Ship,
        att_type1_count: u8,
        def_type1: Ship,
        def_type1_count: u8,
    ) -> BattleBuilder {
        BattleBuilder {
            att_type1,
            att_type1_count,
            def_type1,
            def_type1_count,
            ..Default::default()
        }
    }

    pub fn set_att_type2(mut self, att_type2: Ship, att_type2_count: u8) -> BattleBuilder {
        self.att_type2 = att_type2;
        self.att_type2_count = att_type2_count;
        self
    }

    pub fn build(self) -> Battle {
        Battle {
            att_type1: self.att_type1,
            att_type1_count: self.att_type1_count,
            def_type1: self.def_type1,
            def_type1_count: self.def_type1_count,
            att_type2: self.att_type2,
            att_type2_count: self.att_type2_count,
            ..Default::default()
        }
    }
}
