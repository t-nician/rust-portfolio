use uuid::Uuid;


#[derive(Clone, Copy)]
pub enum ObjectType {
    Platform,
    Player,
    Empty
}


pub struct Dimensions {
    pub x: isize,
    pub y: isize
}


impl Dimensions { 
    pub fn new(x: isize, y: isize) -> Dimensions { 

        return Dimensions { x: x, y: y } 
    }

    pub fn add(&mut self, dimension: &Dimensions) {
        self.x += dimension.x;
        self.y += dimension.y;
    }
}


pub struct EngineObject {
    pub object_uuid: Uuid,
    pub object_type: ObjectType,
    pub object_size: Dimensions,
    pub object_position: Dimensions
}


impl EngineObject { 
    pub fn new(
        object_type: ObjectType, 
        object_size: Dimensions, 
        object_position: Dimensions
    ) -> EngineObject {

        return EngineObject {
            object_uuid: Uuid::new_v4(),
            object_type: object_type,
            object_size: object_size,
            object_position: object_position
        }
    } 
}


pub struct Engine {
    objects: Vec<EngineObject>,
    size: Dimensions
}


impl Engine {
    pub fn new(size: Dimensions) -> Engine {

        return Engine {
            objects: Vec::new(),
            size: size
        }
    }

    pub fn get_object_position(&self, object_uuid: Uuid) -> Option<&Dimensions> {
        for object in &self.objects {
            if object.object_uuid == object_uuid {
                return Some(&object.object_position);
            }
        }

        None
    }

    pub fn get_object_by_uuid(&mut self, object_uuid: Uuid) -> Option<&mut EngineObject> {
        for object in &mut self.objects {
            if object.object_uuid == object_uuid {
                return Some(object);
            }
        }

        None
    }

    pub fn create_platform(&mut self, size: Dimensions, position: Dimensions) -> Uuid {
        let object = EngineObject::new(
            ObjectType::Platform,
            size,
            position
        );

        let uuid = object.object_uuid;

        self.objects.push(object);

        return uuid;
    }


    pub fn translate_object(&mut self, object_uuid: Uuid, translate_by: &Dimensions) {
        for object in &mut self.objects {
            if object.object_uuid == object_uuid {
                object.object_position.x = translate_by.x;
                object.object_position.y = translate_by.y//add(&translate_by);
            }
        }
    }

    pub fn output_engine(&self) {
        let mut output = String::new();
        let mut pixels: Vec<Vec<ObjectType>> = Vec::new();

        for x in 0..self.size.x as usize {
            pixels.push(Vec::new());
            for _ in 0..self.size.y {
                pixels[x].push(ObjectType::Empty);
            }
        }

        for object in &self.objects {
            for offset_x in 0..object.object_size.x {
                for offset_y in 0..object.object_size.y {
                    let target_dimension = self.bound_index(
                        Dimensions::new(
                            object.object_position.x + offset_x, 
                            object.object_position.y + offset_y
                        )
                    );

                    pixels[target_dimension.x as usize][target_dimension.y as usize] = object.object_type;
                }
            }
        }

        for x in 0..self.size.x as usize {
            for y in 0..self.size.y as usize {
                match pixels[x][y] {
                    ObjectType::Empty => { output = output + " " }
                    ObjectType::Player => { output = output + "X" }
                    ObjectType::Platform => { output = output + "#" }
                } 
            }
            output = output + "\n";
        }

        //print!("\x1B[2J\x1B[1;1H");
        //println!("{output}")
    }

    fn bound_index(&self, index: Dimensions) -> Dimensions {
        let mut new_index = Dimensions::new(index.x, index.y);

        if index.x > self.size.x - 1 { new_index.x = self.size.x - 1 }
        if index.y > self.size.y - 1 { new_index.y = self.size.y - 1 }

        if index.x < 0 { new_index.x = 0 }
        if index.y < 0 { new_index.y = 0 }

        return new_index
    }
}