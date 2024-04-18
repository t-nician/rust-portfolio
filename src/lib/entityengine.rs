use crate::EntityPool;


pub struct EntityEngine {
    pub pool: EntityPool
}


impl EntityEngine {
    pub fn new() -> EntityEngine {
        EntityEngine {
            pool: EntityPool::new()
        }
    }

    
    
}