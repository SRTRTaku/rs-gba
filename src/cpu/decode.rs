type Cond = u8;

#[derive(PartialEq, Debug)]
enum Inst {
    B(Cond, i32),
    BL(Cond, i32),
}
pub fn decode(opcode: u32) -> Inst {
    let cond = ((opcode & 0xf0000000) >> 28) as u8;
    if opcode & 0x0e000000 == 0x0a000000 {
        let nn_u24 = opcode & 0x00ffffff;
        let nn = if nn_u24 < 0x008fffff {
            nn_u24 as i32
        } else {
            (nn_u24 as i32) - 0x01000000
        };
        if opcode & 0x01000000 == 0 {
            Inst::B(cond, nn)
        } else {
            Inst::BL(cond, nn)
        }
    } else {
        panic!();
    }
}

#[cfg(test)]
mod testd {
    use super::*;

    #[test]
    fn b() {
        let opcode = 0xaaffffff; // 0b 1010_101_0_1..1
        let inst = decode(opcode);
        assert_eq!(inst, Inst::B(0xa, -1));
    }
}
