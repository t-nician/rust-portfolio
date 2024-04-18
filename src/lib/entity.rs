use std::ops;
use uuid::Uuid;


pub struct XYVector {
    pub x: isize,
    pub y: isize
}


pub struct Entity {
    pub uuid: Uuid,
    pub size: XYVector,
    pub position: XYVector,
}


pub struct EntityPool {
    entities: Vec<Entity>,
}


impl XYVector {
    pub fn new(x: isize, y: isize) -> XYVector {
        XYVector {
            x: x,
            y: y
        }
    }
}


impl ops::Add<XYVector> for XYVector {
    type Output = XYVector;

    fn add(self, _rhs: XYVector) -> XYVector {
        XYVector::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}


impl ops::AddAssign<XYVector> for XYVector {
    fn add_assign(&mut self, _rhs: XYVector) {
        self.x += _rhs.x;
        self.y += _rhs.y;
    }
}


impl Entity {
    pub fn new(size: XYVector, position: XYVector) -> Entity {
        Entity {
            size: size,
            position: position,
            uuid: uuid::Uuid::new_v4()
        }
    }


    pub fn resize(&mut self, size: XYVector) {
        self.size = size;
    }


    pub fn translate(&mut self, offset: XYVector) {
        self.position += offset;
    }


    pub fn relocate(&mut self, position: XYVector) {
        self.position = position;
    }
}


impl EntityPool {
    pub fn new() -> EntityPool {
        EntityPool {
            entities: Vec::new()
        }
    }


    pub fn create_entity(&mut self, size: XYVector, position: XYVector) -> Uuid {
        let entity = Entity::new(size, position);
        let uuid = entity.uuid;

        self.entities.push(entity);

        return uuid;
    }


    pub fn get_entity(&self, uuid: Uuid) -> Option<&Entity> {
        for entity in &self.entities {
            if entity.uuid == uuid {
                return Some(entity);
            }
        }

        None
    }


    pub fn get_mut_entity(&mut self, uuid: Uuid) -> Option<&mut Entity> {
        for entity in &mut self.entities {
            if entity.uuid == uuid {
                return Some(entity);
            }
        }

        None
    }


    pub fn get_colliding_entities(&self, uuid: Uuid) -> Vec<&Entity> {
        let mut result = Vec::new();
        let home_entity = self.get_entity(uuid).unwrap();

        for entity in &self.entities {
            if entity.uuid != uuid {
                if (home_entity.position.x < entity.position.x + entity.size.x) &&
                    (home_entity.position.x + home_entity.size.x > entity.position.x) &&
                    (home_entity.position.y < entity.position.y + entity.size.y) && 
                    (home_entity.position.y + home_entity.size.y > entity.position.y) {
                    
                    result.push(entity);
                }
            }
        }

        return result;
    }


    pub fn would_entity_collide_here(&self, size: XYVector, position: XYVector) -> bool {
        for entity in &self.entities {
            if (position.x < entity.position.x + entity.size.x) &&
                (position.x + size.x > entity.position.x) &&
                (position.y < entity.position.y + entity.size.y) && 
                (position.y + size.y > entity.position.y) {
                    
                return true;
            }
        }

        return false
    }

}