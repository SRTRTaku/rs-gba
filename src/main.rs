mod cpu;

fn main() {
    let mut x = cpu::Cpu::new();
    x.step();
    println!("Hello, world!");
}
