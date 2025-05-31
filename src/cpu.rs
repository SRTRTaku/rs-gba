mod decode;

pub struct Cpu {
    x: u32,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { x: 1 }
    }
    pub fn step(&mut self) {
        //decode::decode();
        //execute();
    }
}
