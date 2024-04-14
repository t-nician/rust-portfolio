use rand::Rng;

pub struct ProtectedNumber {
    key: Vec<u8>,
    data: Vec<u8>
}


impl ProtectedNumber {
    pub fn new(number: i64) -> ProtectedNumber {
        let mut protected_number = ProtectedNumber {
            key: Vec::new(),
            data: Vec::new()
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

        self.key.clear();
        self.data.clear();
  
        for index in 0..number_as_bytes.len() {
            let key_byte = rand::thread_rng().gen::<u8>();
            let protected_byte = number_as_bytes[index] ^ key_byte;

            self.key.push(key_byte);
            self.data.push(protected_byte);
        }   
    }
}