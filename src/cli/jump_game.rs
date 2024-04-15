struct Display {

}


impl Display {
    pub fn new() -> Display {
        Display {

        }
    }
}


pub struct JumpGame {
    display: Display
}


impl JumpGame {
    pub fn new() -> JumpGame {
        JumpGame {
            display: Display::new(),
        }
    }

    pub fn update_and_display(&mut self) {
        
    }
}