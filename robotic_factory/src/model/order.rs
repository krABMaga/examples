//----------------OrderManagement----------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialManagement {
    supply: u32,
    products: u32,
}

impl MaterialManagement {
    pub fn new() -> MaterialManagement {
        MaterialManagement {
            supply: 0,
            products: 0,
        }
    }

    pub fn has_supply(&self) -> bool {
        self.supply > 0
    }
    pub fn get_supply_count(&self) -> u32 {
        self.supply
    }
    pub fn get_products_count(&self) -> u32 {
        self.products
    }
    pub fn has_products(&self) -> bool {
        self.products > 0
    }

    pub fn decrement_supply(&mut self) {
        self.supply -= 1;
    }
    pub fn increment_products(&mut self) {
        self.products += 1;
    }
    pub fn add_supply(&mut self, amount: u32) {
        self.supply += amount;
    }
    pub fn add_products(&mut self, amount: u32) { self.products += amount; }
}
