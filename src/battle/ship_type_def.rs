use crate::ship::Ship;

#[derive(Default, Clone)]
pub struct ShipType {
    pub ship_type: Ship,
    pub count: u8,
    pub acc_dmg: u8,
}
