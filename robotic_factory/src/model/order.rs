use krabmaga::{rand, Rng};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum OrderState {
    Ready,
    Cut,
    Stitched,
    Finished,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Order {
    state: OrderState,
    is_luxury: bool,
}

impl Order {
    pub fn new() -> Order {
        Order {
            state: OrderState::Ready,
            is_luxury: rand::thread_rng().gen_bool(0.03),
        }
    }
}

//----------------OrderManagement----------------
#[derive(Clone)]
pub struct OrderManagement {
    orders: Vec<Order>,
    products: Vec<Order>,
}

impl OrderManagement {
    pub fn new() -> OrderManagement {
        OrderManagement {
            orders: vec![],
            products: vec![],
        }
    }

    pub fn place_order(&mut self, order: Order) {
        self.orders.push(order);
    }
    pub fn has_next_order(&self) -> bool {
        !self.orders.is_empty()
    }
    pub fn queue_length(&self) -> usize {
        self.orders.len()
    }
    pub fn get_next_product(&mut self) -> Order {
        self.products.pop().unwrap()
    }
    pub fn has_next_product(&self) -> bool {
        !self.products.is_empty()
    }
    pub fn finish_next_order(&mut self) {
        if self.has_next_order() {
            let next = self.get_next_product();
            self.products.push(next);
        }
    }
}
