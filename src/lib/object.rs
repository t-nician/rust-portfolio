use std::ops;

pub struct XYVector {
    pub x: isize,
    pub y: isize
}


impl XYVector {
    pub fn new(x: isize, y: isize) -> XYVector {
        XYVector {
            x: x,
            y: y
        }
    }
}

impl ops::AddAssign<XYVector> for XYVector {
    fn add_assign(&mut self, _rhs: XYVector) {
        self.x += _rhs.x;
        self.y += _rhs.y;
    }
}


pub struct Entity {
    pub size: XYVector,
    pub position: XYVector
}


impl Entity {
    pub fn new(size: XYVector, position: XYVector) -> Entity {
        Entity {
            size: size,
            position: position
        }
    }

    pub fn resize(&mut self, size: XYVector) {
        self.size = size;
    }


    pub fn push(&mut self, offset: XYVector) {
        self.position += offset;
    }
}