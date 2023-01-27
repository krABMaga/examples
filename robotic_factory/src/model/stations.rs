use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::state::State;
use krabmaga::{rand, Rng};

use crate::model::order::Order;
use crate::model::order::OrderManagement;

pub(crate) trait Station {
    fn get_order_management(&mut self) -> &mut OrderManagement;
    fn place_order(&mut self, order: Order) {
        self.get_order_management().place_order(order)
    }
    fn get_next_product(&mut self) -> Order {
        self.get_order_management().get_next_product()
    }
    fn has_next_product(&mut self) -> bool {
        self.get_order_management().has_next_product()
    }
}

//----------------SticherStation----------------
#[derive(Clone)]
pub struct SticherStation {
    order_management: OrderManagement,
    location: Real2D,
}

impl SticherStation {
    pub fn new(location: Real2D) -> SticherStation {
        SticherStation {
            order_management: OrderManagement::new(),
            location,
        }
    }
}

impl Station for SticherStation {
    fn get_order_management(&mut self) -> &mut OrderManagement {
        &mut self.order_management
    }
}

impl Agent for SticherStation {
    fn step(&mut self, _state: &mut dyn State) {
        self.order_management.finish_next_order();
    }
}

//----------------CuttingStation----------------
#[derive(Clone)]
pub struct CuttingStation {
    order_management: OrderManagement,
    location: Real2D,
}

impl CuttingStation {
    pub fn new(location: Real2D) -> CuttingStation {
        CuttingStation {
            order_management: OrderManagement::new(),
            location,
        }
    }
}

impl Station for CuttingStation {
    fn get_order_management(&mut self) -> &mut OrderManagement {
        &mut self.order_management
    }
}

impl Agent for CuttingStation {
    fn step(&mut self, _state: &mut dyn State) {
        self.order_management.finish_next_order();
    }
}

//----------------Finisher----------------

#[derive(Clone)]
pub struct FinisherStation {
    order_management: OrderManagement,
    location: Real2D,
    progress_time: u32,
    process_time: u32,
}

impl FinisherStation {
    fn new(process_time: u32, location: Real2D) -> FinisherStation {
        if process_time <= 0 {
            panic!("process_time must be greater than 0");
        }
        FinisherStation {
            order_management: OrderManagement::new(),
            location,
            progress_time: 0,
            process_time,
        }
    }
}

impl Station for FinisherStation {
    fn get_order_management(&mut self) -> &mut OrderManagement {
        &mut self.order_management
    }
}

impl Agent for FinisherStation {
    fn step(&mut self, _state: &mut dyn State) {
        if self.order_management.has_next_order() {
            if self.progress_time < self.process_time {
                self.progress_time += 1;
            } else {
                self.order_management.finish_next_order();
                self.progress_time = 0;
            }
        }
    }
}

//----------------LoadingDock----------------

#[derive(Clone)]
pub struct LoadingDock {
    order_management: OrderManagement,
    location: Real2D,
}

impl LoadingDock {
    pub fn new(location: Real2D) -> LoadingDock {
        LoadingDock {
            order_management: OrderManagement::new(),
            location,
        }
    }
}

impl Station for LoadingDock {
    fn get_order_management(&mut self) -> &mut OrderManagement {
        &mut self.order_management
    }
}

impl Agent for LoadingDock {
    fn step(&mut self, _state: &mut dyn State) {
        if self.order_management.queue_length() < 3 && rand::thread_rng().gen_bool(0.03) {
            self.order_management.place_order(Order::new());
        }
    }
}

//----------------Storage Room----------------

#[derive(Clone)]
pub struct StorageRoom {
    pub finished_orders: u32,
    location: Real2D,
}

impl StorageRoom {
    pub fn new(location: Real2D) -> StorageRoom {
        StorageRoom {
            finished_orders: 0,
            location,
        }
    }

    pub fn close_order(&mut self) {
        self.finished_orders += 1;
    }
}

//----------------Robot Room---------------- aka charging station

#[derive(Clone)]
pub struct RobotRoom {
    location: Real2D,
}
