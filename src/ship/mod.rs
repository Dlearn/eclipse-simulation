use rand::prelude::*;

#[derive(Default)]
pub struct Ship {
    pub name: String,
    pub initiative: u8,
    pub computers: u8,
    pub shields: u8,
    pub hull: u8,
    pub yellow_dice: u8,
    pub orange_dice: u8,
    pub blue_dice: u8,
    pub red_dice: u8,
}

#[allow(dead_code)]
impl Ship {
    pub fn new(
        name: String,
        initiative: u8,
        computers: u8,
        shields: u8,
        hull: u8,
        yellow_dice: u8,
        orange_dice: u8,
        blue_dice: u8,
        red_dice: u8,
    ) -> Ship {
        Ship {
            name,
            initiative,
            computers,
            shields,
            hull,
            yellow_dice,
            orange_dice,
            blue_dice,
            red_dice,
        }
    }

    pub fn interceptor() -> Ship {
        Self::new("interceptor".to_owned(), 2, 0, 0, 0, 1, 0, 0, 0)
    }

    pub fn cruiser() -> Ship {
        Self::new("cruiser".to_owned(), 1, 1, 0, 1, 1, 0, 0, 0)
    }

    pub fn dreadnaught() -> Ship {
        Self::new("dreadnaught".to_owned(), 1, 1, 0, 2, 2, 0, 0, 0)
    }

    pub fn base_ancient() -> Ship {
        Self::new("base_ancient".to_owned(), 2, 1, 0, 1, 2, 0, 0, 0)
    }

    pub fn roll_against(self: &Ship, opp_ship: &Ship) -> u8 {
        let mut rng = rand::thread_rng();
        let mut damage = 0;
        for _n in 0..self.yellow_dice {
            let v = rng.gen_range(1..=6);
            // print!("{} ", v);
            if v == 6 {
                damage += 1;
                // print!("HIT!");
            } else if v > 1 {
                if v + self.computers - opp_ship.shields >= 6 {
                    damage += 1;
                    // print!("HIT!");
                }
            }
            // println!();
        }
        damage
    }
}
