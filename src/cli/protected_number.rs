use rand::Rng;

pub struct ProtectedNumber {
    key: [u8; 8],
    data: [u8; 8]
}


impl ProtectedNumber {
    pub fn new(number: i64) -> ProtectedNumber {
        let mut protected_number = ProtectedNumber {
            key: [0 as u8; 8],
            data: [0 as u8; 8]
        };

        protected_number.set_number(number);

        return protected_number;
    }

    pub fn get_number(&self) -> i64 {
        let mut real_number = [0 as u8; 8];

        for index in 0..8 {
            real_number[index] = self.data[index] ^ self.key[index]
        }

        return i64::from_be_bytes(real_number)
    }
    

    pub fn set_number(&mut self, number: i64) {
        let number_as_bytes = number.to_be_bytes();
  
        for index in 0..8 {
            let key_byte = rand::thread_rng().gen::<u8>();
            let protected_byte = number_as_bytes[index] ^ key_byte;

            self.key[index] = key_byte;
            self.data[index] = protected_byte;
        }   
    }
}