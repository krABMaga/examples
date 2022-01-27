use crate::model::bird::Bird;
use crate::{DISCRETIZATION, TOROIDAL};

use rust_ab::mpi::*;
use rust_ab::mpi::traits::*;

use rust_ab::engine::fields::field::Field;
use rust_ab::engine::fields::field_2d::Field2D;
use rust_ab::engine::location::Real2D;
use rust_ab::engine::schedule::Schedule;
use rust_ab::engine::state::State;
use rust_ab::rand;
use rust_ab::rand::Rng;
use std::any::Any;
use std::sync::Mutex;
use rust_ab::mpi::environment::Universe;


pub struct Flocker {
    pub step: u64,
    pub field1: Field2D<Bird>,
    pub dim: (f32, f32),
    pub initial_flockers:u32,
    pub universe: Universe,
    pub partition: (f32,f32),
    pub l_aoi: Mutex<Vec<Bird>>,
    pub r_aoi: Mutex<Vec<Bird>>
}

impl Flocker {
    #[allow(dead_code)]
    pub fn new(dim: (f32, f32),initial_flockers: u32,universe: Universe) -> Self {
        Flocker {
            step: 0,
            field1: Field2D::new(dim.0, dim.1, DISCRETIZATION, TOROIDAL),
            initial_flockers,
            dim,
            universe,
            partition:dim,
            l_aoi: Mutex::new(Vec::new()),
            r_aoi: Mutex::new(Vec::new())
        }
    }

    // pub fn rank(&self) -> i32{
    //     self.universe.world().rank()
    // }
}


impl State for Flocker {
    fn reset(&mut self) {
        self.step = 0;
        self.field1 = Field2D::new(self.dim.0, self.dim.1, DISCRETIZATION, TOROIDAL);
    }

    fn init(&mut self, schedule: &mut Schedule){
        let world = self.universe.world();
        let rank = world.rank();
        let size = world.size();
        let partition_length = self.dim.0 / size as f32;
        let partition_start = rank as f32 * partition_length;
        let partition_end = (rank+1) as f32 * partition_length;
        self.partition = (partition_start,partition_end);

        //master process initializes birds
        if rank == 0 {
            let mut partitions = std::iter::repeat(Vec::new()).take(size as usize).collect::<Vec<Vec<Bird>>>();
            let mut rng = rand::thread_rng();
            
            for bird_id in 0..self.initial_flockers{
                let r1: f32 = rng.gen();
                let r2: f32 = rng.gen();
                let last_d = Real2D { x: 0., y: 0. };
                let pos = Real2D {
                    x: self.dim.0 * r1,
                    y: self.dim.1 * r2,
                };
                let bird = Bird::new(bird_id, pos, last_d);
                let destination_rank = (bird.pos.x / partition_length).floor() as usize;
                partitions[destination_rank].push(bird);
            }

            //each partition is sent to a process
            for (destination,partition) in partitions[1..].iter().enumerate(){
                println!("master sending to process {}, {} birds",destination+1,partition.len());
                world.process_at_rank(destination as i32 + 1).send(&partition[..]);
            }

            //master process initializes its partition
            for bird in partitions[0].iter(){
                self.field1.set_object_location(*bird,bird.pos);
                schedule.schedule_repeating(Box::new(*bird),0.0,0);
            }

        }
        //worker processes receive their partition
        else{
            let (birds,_) = world.process_at_rank(0).receive_vec::<Bird>();
            println!("process {} received {} birds",rank,birds.len());
            for bird in birds{
                self.field1.set_object_location(bird,bird.pos);
                schedule.schedule_repeating(Box::new(bird),0.0,0);
            }
        }
       
    }

    fn after_step(&mut self,schedule:&mut Schedule){
        let world = self.universe.world();
        let rank = world.rank();
        let size = world.size();
       
        let r_process =  world.process_at_rank((rank+1) % size);
        let l_process = world.process_at_rank(((rank-1)%size + size)%size);
        let r = Real2D{x:0.,y:0.};
        let mut r_birds: Vec<Bird> = std::iter::repeat(Bird::new(0,r,r)).take( self.initial_flockers as usize).collect();
        let mut l_birds: Vec<Bird> = std::iter::repeat(Bird::new(0,r,r)).take( self.initial_flockers as usize).collect();

        let r_aoi: Vec<Bird> = self.r_aoi.lock().unwrap().iter().map( |b| b.clone() ).collect();
        let l_aoi: Vec<Bird> = self.l_aoi.lock().unwrap().iter().map( |b| b.clone() ).collect();

        self.r_aoi.lock().unwrap().clear();
        self.l_aoi.lock().unwrap().clear();

        //processes exchange information about agents around the border (l_aoi and r_aoi)
        let s1 = rust_ab::mpi::point_to_point::send_receive_into(&l_aoi[..],&l_process,&mut r_birds[..],&r_process);
        
        let s2 = rust_ab::mpi::point_to_point::send_receive_into(&r_aoi[..],&r_process,&mut l_birds[..],&l_process);

        
        let received_r = s1.count(Bird::equivalent_datatype()) as usize;
        for mut bird in r_birds.into_iter().take(received_r){
            // println!("p{} received {} from RIGHT step {}, {}",rank+1,bird,self.step ,if bird.migrated {"MIGRATED"}else{"AOI"});
            if bird.migrated {
                bird.migrated = false;
                // println!("p{} new bird from right {}",rank,bird);
                schedule.schedule_repeating(Box::new(bird), schedule.time + 1.0,0);
            }

            self.field1.set_object_location(bird,bird.pos);
        }

        let received_l = s2.count(Bird::equivalent_datatype()) as usize;
        for mut bird in l_birds.into_iter().take(received_l){
            // println!("p{} received {} from LEFT step {}, {}",rank+1,bird,self.step, if bird.migrated {"MIGRATED"}else{"AOI"});
            if bird.migrated{
                
                bird.migrated = false;
                // println!("p{} new bird from left {}",rank,bird);
                schedule.schedule_repeating(Box::new(bird), schedule.time + 1.0,0);
            }
            self.field1.set_object_location(bird,bird.pos);
        }
        self.step+=1;
        
    }

    fn update(&mut self, _step: u64) {
        self.field1.lazy_update();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }  

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }
}
