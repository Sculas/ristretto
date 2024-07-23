use crate::attributes::Instruction;
use crate::Result;
use std::collections::HashMap;
use std::io::Cursor;

/// Converts a vector of instructions to a vector of bytes. Using the instruction enum is a more
/// idiomatic way to represent the instructions, but the JVM uses a byte representation.  This
/// function converts the instruction enums to a byte representation and adjusts offsets where
/// necessary.
#[allow(clippy::too_many_lines)]
pub(crate) fn to_bytes(instructions: &[Instruction]) -> Result<Vec<u8>> {
    let mut bytes = Cursor::new(Vec::new());
    let mut instruction_to_byte_map = HashMap::new();
    for (index, instruction) in instructions.iter().enumerate() {
        let byte_position = u16::try_from(bytes.position())?;
        let instruction_position = u16::try_from(index)?;
        instruction_to_byte_map.insert(instruction_position, byte_position);
        instruction.to_bytes(&mut bytes)?;
    }

    let mut bytes = Cursor::new(Vec::new());
    let mut instructions = instructions.to_owned();
    for (index, instruction) in instructions.iter_mut().enumerate() {
        match instruction {
            Instruction::Ifeq(ref mut offset)
            | Instruction::Ifne(ref mut offset)
            | Instruction::Iflt(ref mut offset)
            | Instruction::Ifge(ref mut offset)
            | Instruction::Ifgt(ref mut offset)
            | Instruction::Ifle(ref mut offset)
            | Instruction::If_icmpeq(ref mut offset)
            | Instruction::If_icmpne(ref mut offset)
            | Instruction::If_icmplt(ref mut offset)
            | Instruction::If_icmpge(ref mut offset)
            | Instruction::If_icmpgt(ref mut offset)
            | Instruction::If_icmple(ref mut offset)
            | Instruction::If_acmpeq(ref mut offset)
            | Instruction::If_acmpne(ref mut offset)
            | Instruction::Goto(ref mut offset)
            | Instruction::Jsr(ref mut offset)
            | Instruction::Ifnull(ref mut offset)
            | Instruction::Ifnonnull(ref mut offset)
            | Instruction::Goto_w(ref mut offset)
            | Instruction::Jsr_w(ref mut offset) => {
                *offset = *instruction_to_byte_map
                    .get(offset)
                    .expect("instruction byte");
            }
            Instruction::Tableswitch {
                ref mut default,
                ref mut offsets,
                ..
            } => {
                let position = u32::try_from(index)?;
                let default_offset = position + u32::try_from(*default)?;
                let byte_default = *instruction_to_byte_map
                    .get(&u16::try_from(default_offset)?)
                    .expect("instruction byte")
                    - u16::try_from(index)?;
                *default = i32::from(byte_default);

                for offset in offsets.iter_mut() {
                    let instruction_offset = position + u32::try_from(*offset)?;
                    let byte_offset = instruction_to_byte_map
                        .get(&u16::try_from(instruction_offset)?)
                        .expect("instruction byte")
                        - u16::try_from(index)?;
                    *offset = i32::from(byte_offset);
                }
            }
            Instruction::Lookupswitch {
                ref mut default,
                ref mut pairs,
            } => {
                let position = u32::try_from(index)?;
                let default_offset = position + u32::try_from(*default)?;
                let byte_default = instruction_to_byte_map
                    .get(&u16::try_from(default_offset)?)
                    .expect("instruction byte")
                    - u16::try_from(index)?;
                *default = i32::from(byte_default);

                for (_match, offset) in pairs.iter_mut() {
                    let instruction_offset = position + u32::try_from(*offset)?;
                    let byte_offset = instruction_to_byte_map
                        .get(&u16::try_from(instruction_offset)?)
                        .expect("instruction byte")
                        - u16::try_from(index)?;
                    *offset = i32::from(byte_offset);
                }
            }
            _ => {}
        }

        instruction.to_bytes(&mut bytes)?;
    }
    Ok(bytes.into_inner())
}

/// Converts a vector of bytes to a vector of instructions. Using the instruction enum is a more
/// idiomatic way to represent the instructions, but the JVM uses a byte representation.  This
/// function converts bytes to instruction enums and adjusts offsets where necessary.
#[allow(clippy::too_many_lines)]
pub(crate) fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let mut byte_to_instruction_map = HashMap::new();
    let mut instruction_to_byte_map = HashMap::new();
    while bytes.position() < bytes.get_ref().len() as u64 {
        let byte_position = u16::try_from(bytes.position())?;
        let instruction_position = u16::try_from(instructions.len())?;
        byte_to_instruction_map.insert(byte_position, instruction_position);
        instruction_to_byte_map.insert(instruction_position, byte_position);
        let instruction = Instruction::from_bytes(bytes)?;
        instructions.push(instruction);
    }

    for (index, instruction) in instructions.iter_mut().enumerate() {
        match instruction {
            Instruction::Ifeq(ref mut offset)
            | Instruction::Ifne(ref mut offset)
            | Instruction::Iflt(ref mut offset)
            | Instruction::Ifge(ref mut offset)
            | Instruction::Ifgt(ref mut offset)
            | Instruction::Ifle(ref mut offset)
            | Instruction::If_icmpeq(ref mut offset)
            | Instruction::If_icmpne(ref mut offset)
            | Instruction::If_icmplt(ref mut offset)
            | Instruction::If_icmpge(ref mut offset)
            | Instruction::If_icmpgt(ref mut offset)
            | Instruction::If_icmple(ref mut offset)
            | Instruction::If_acmpeq(ref mut offset)
            | Instruction::If_acmpne(ref mut offset)
            | Instruction::Goto(ref mut offset)
            | Instruction::Jsr(ref mut offset)
            | Instruction::Ifnull(ref mut offset)
            | Instruction::Ifnonnull(ref mut offset)
            | Instruction::Goto_w(ref mut offset)
            | Instruction::Jsr_w(ref mut offset) => {
                *offset = *byte_to_instruction_map
                    .get(offset)
                    .expect("byte instruction");
            }
            Instruction::Tableswitch {
                ref mut default,
                ref mut offsets,
                ..
            } => {
                let position = instruction_to_byte_map
                    .get(&u16::try_from(index)?)
                    .expect("instruction byte");
                let position = u32::from(*position);
                let default_offset = position + u32::try_from(*default)?;
                let instruction_default = byte_to_instruction_map
                    .get(&u16::try_from(default_offset)?)
                    .expect("byte instruction")
                    - u16::try_from(index)?;
                *default = i32::from(instruction_default);

                for offset in offsets.iter_mut() {
                    let byte_offset = position + u32::try_from(*offset)?;
                    let instruction_offset = byte_to_instruction_map
                        .get(&u16::try_from(byte_offset)?)
                        .expect("byte instruction")
                        - u16::try_from(index)?;
                    *offset = i32::from(instruction_offset);
                }
            }
            Instruction::Lookupswitch {
                ref mut default,
                ref mut pairs,
            } => {
                let position = instruction_to_byte_map
                    .get(&u16::try_from(index)?)
                    .expect("instruction byte");
                let position = u32::from(*position);
                let default_offset = position + u32::try_from(*default)?;
                let instruction_default = byte_to_instruction_map
                    .get(&u16::try_from(default_offset)?)
                    .expect("byte instruction")
                    - u16::try_from(index)?;
                *default = i32::from(instruction_default);

                for (_match, offset) in pairs.iter_mut() {
                    let byte_offset = position + u32::try_from(*offset)?;
                    let instruction_offset = byte_to_instruction_map
                        .get(&u16::try_from(byte_offset)?)
                        .expect("byte instruction")
                        - u16::try_from(index)?;
                    *offset = i32::from(instruction_offset);
                }
            }
            _ => {}
        }
    }
    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bytes() -> Result<()> {
        let instructions = vec![
            Instruction::Iconst_0,
            Instruction::Istore_0,
            Instruction::Iload_0,
            Instruction::Iconst_1,
            Instruction::Iadd,
            Instruction::Ireturn,
        ];
        let bytes = to_bytes(&instructions)?;
        let mut cursor = Cursor::new(bytes);
        let result = from_bytes(&mut cursor)?;

        assert_eq!(instructions, result);

        Ok(())
    }

    #[test]
    fn test_from_bytes() -> Result<()> {
        let bytes = vec![
            3,   // Iconst_0
            59,  // Istore_0
            26,  // Iload_0
            4,   // Iconst_1
            96,  // Iadd
            172, // Ireturn
        ];
        let mut cursor = Cursor::new(bytes);
        let result = from_bytes(&mut cursor)?;
        let instructions = vec![
            Instruction::Iconst_0,
            Instruction::Istore_0,
            Instruction::Iload_0,
            Instruction::Iconst_1,
            Instruction::Iadd,
            Instruction::Ireturn,
        ];

        assert_eq!(instructions, result);

        Ok(())
    }

    fn test_instruction(instruction: Instruction) -> Result<()> {
        let expected_bytes = [Instruction::Nop.code(), instruction.code(), 255, 255];
        let instructions = [Instruction::Nop, instruction];

        let bytes = to_bytes(instructions.as_slice())?;
        assert_eq!(expected_bytes, bytes.as_slice());

        let instructions_from_bytes = from_bytes(&mut Cursor::new(bytes))?;
        assert_eq!(instructions, instructions_from_bytes.as_slice());
        Ok(())
    }

    #[test]
    fn test_ifeq() -> Result<()> {
        test_instruction(Instruction::Ifeq(0))
    }

    #[test]
    fn test_ifne() -> Result<()> {
        test_instruction(Instruction::Ifne(0))
    }

    #[test]
    fn test_iflt() -> Result<()> {
        test_instruction(Instruction::Iflt(0))
    }

    #[test]
    fn test_ifge() -> Result<()> {
        test_instruction(Instruction::Ifge(0))
    }

    #[test]
    fn test_ifgt() -> Result<()> {
        test_instruction(Instruction::Ifgt(0))
    }

    #[test]
    fn test_ifle() -> Result<()> {
        test_instruction(Instruction::Ifle(0))
    }

    #[test]
    fn test_if_icmpeq() -> Result<()> {
        test_instruction(Instruction::If_icmpeq(0))
    }

    #[test]
    fn test_if_icmpne() -> Result<()> {
        test_instruction(Instruction::If_icmpne(0))
    }

    #[test]
    fn test_if_icmplt() -> Result<()> {
        test_instruction(Instruction::If_icmplt(0))
    }

    #[test]
    fn test_if_icmpge() -> Result<()> {
        test_instruction(Instruction::If_icmpge(0))
    }

    #[test]
    fn test_if_icmpgt() -> Result<()> {
        test_instruction(Instruction::If_icmpgt(0))
    }

    #[test]
    fn test_if_icmple() -> Result<()> {
        test_instruction(Instruction::If_icmple(0))
    }

    #[test]
    fn test_if_acmpeq() -> Result<()> {
        test_instruction(Instruction::If_acmpeq(0))
    }

    #[test]
    fn test_if_acmpne() -> Result<()> {
        test_instruction(Instruction::If_acmpne(0))
    }

    #[test]
    fn test_goto() -> Result<()> {
        test_instruction(Instruction::Goto(0))
    }

    #[test]
    fn test_jsr() -> Result<()> {
        test_instruction(Instruction::Jsr(0))
    }

    #[test]
    fn test_ifnull() -> Result<()> {
        test_instruction(Instruction::Ifnull(0))
    }

    #[test]
    fn test_ifnonnull() -> Result<()> {
        test_instruction(Instruction::Ifnonnull(0))
    }

    #[test]
    fn test_goto_w() -> Result<()> {
        let instruction = Instruction::Goto_w(1);
        let expected_bytes = [instruction.code(), 0, 0, 0, 5, Instruction::Nop.code()];
        let instructions = [instruction, Instruction::Nop];

        let bytes = to_bytes(instructions.as_slice())?;
        assert_eq!(expected_bytes, bytes.as_slice());

        let instructions_from_bytes = from_bytes(&mut Cursor::new(bytes))?;
        assert_eq!(instructions, instructions_from_bytes.as_slice());
        Ok(())
    }

    #[test]
    fn test_jsr_w() -> Result<()> {
        let instruction = Instruction::Jsr_w(1);
        let expected_bytes = [instruction.code(), 0, 0, 0, 5, Instruction::Nop.code()];
        let instructions = [instruction, Instruction::Nop];

        let bytes = to_bytes(instructions.as_slice())?;
        assert_eq!(expected_bytes, bytes.as_slice());

        let instructions_from_bytes = from_bytes(&mut Cursor::new(bytes))?;
        assert_eq!(instructions, instructions_from_bytes.as_slice());
        Ok(())
    }

    #[test]
    fn test_tableswitch() -> Result<()> {
        let instruction = Instruction::Tableswitch {
            default: 3,
            low: 3,
            high: 4,
            offsets: vec![1, 2],
        };
        let expected_bytes = [
            instruction.code(),
            0,
            0,
            0,
            0,
            0,
            0,
            26,
            0,
            0,
            0,
            3,
            0,
            0,
            0,
            4,
            0,
            0,
            0,
            24,
            0,
            0,
            0,
            25,
            Instruction::Nop.code(),
            Instruction::Nop.code(),
            Instruction::Nop.code(),
        ];
        let instructions = [
            instruction,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
        ];

        let bytes = to_bytes(instructions.as_slice())?;
        assert_eq!(expected_bytes, bytes.as_slice());

        let instructions_from_bytes = from_bytes(&mut Cursor::new(bytes))?;
        assert_eq!(instructions, instructions_from_bytes.as_slice());
        Ok(())
    }

    #[test]
    fn test_lookupswitch() -> Result<()> {
        let instruction = Instruction::Lookupswitch {
            default: 3,
            pairs: vec![(1, 2)],
        };
        let expected_bytes = [
            instruction.code(),
            0,
            0,
            0,
            0,
            0,
            0,
            22,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            21,
            Instruction::Nop.code(),
            Instruction::Nop.code(),
            Instruction::Nop.code(),
        ];
        let instructions = [
            instruction,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
        ];

        let bytes = to_bytes(instructions.as_slice())?;
        assert_eq!(expected_bytes, bytes.as_slice());

        let instructions_from_bytes = from_bytes(&mut Cursor::new(bytes))?;
        assert_eq!(instructions, instructions_from_bytes.as_slice());
        Ok(())
    }
}
