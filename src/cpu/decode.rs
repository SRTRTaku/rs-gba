type Cond = u8;
type Reg = usize;

#[derive(PartialEq, Debug)]
enum Inst {
    B(Cond, i32),
    BL(Cond, i32),
    BX(Cond, Reg),
}
pub fn decode(opcode: u32) -> Inst {
    let cond = ((opcode & 0xf0000000) >> 28) as Cond;
    if opcode & 0x0ffffff0 == 0x012fff10 {
        let rn = (opcode & 0x0000000f) as Reg;
        Inst::BX(cond, rn)
    } else if opcode & 0x0e000000 == 0x0a000000 {
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
    #[test]
    fn bl() {
        let opcode = 0xabffffff; // 0b 1010_101_1_1..1
        let inst = decode(opcode);
        assert_eq!(inst, Inst::BL(0xa, -1));
    }
    #[test]
    fn bx() {
        // 0b 1010_
        // 0001_0010_1111_1111_1111_0001
        // 0101
        let opcode = 0xa12fff15;
        let inst = decode(opcode);
        assert_eq!(inst, Inst::BX(0xa, 5));
    }
}
