//! The GTA IV script instruction set.
//!
//! One-byte opcodes with inline operands, decoded from a byte slice with
//! explicit little-endian reads. Byte values `0x00` and `0x4F` are not valid
//! instructions. Bytes `0x50..=0xFF` push small integer constants: the pushed
//! value is the opcode byte minus `0x60`, giving `-16..=159`.
//!
//! Operand shapes:
//!
//! | opcode(s) | bytes after opcode | meaning |
//! |---|---|---|
//! | `J`, `JZ`, `JNZ`, `CALL`, `PUSH_CONST_U32` | 4 (`u32`) | absolute code offset, or constant |
//! | `PUSH_CONST_U16` | 2 (`u16`) | constant |
//! | `PUSH_CONST_F` | 4 (`f32`) | constant |
//! | `NATIVE` | 1 + 1 + 4 | arg count, return count, native hash |
//! | `ENTER` | 1 + 2 | arg count, frame size in slots |
//! | `LEAVE` | 1 + 1 | arg count, return count |
//! | `SWITCH` | 1 + 8 per case | case count, then (value `u32`, target `u32`) pairs |
//! | `STRING` | 1 + length | length in bytes including the trailing NUL, then the bytes |
//! | `TEXT_LABEL_*` (4 opcodes) | 1 | maximum text length including the NUL |
//! | everything else | 0 | stack-only operation |
//!
//! The three `_XPROTECT` opcodes exist only in the PC build; they behave like
//! `LOAD` / `STORE` / address-borrow for copy-protected memory.

/// A decoded instruction: its offset, length, opcode and operands.
#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
    /// Byte offset of the opcode within the code segment.
    pub offset: u32,
    /// Total length of the instruction in bytes, opcode included.
    pub size: u32,
    /// The opcode.
    pub opcode: Opcode,
    /// The decoded operands.
    pub operand: Operand,
}

/// All valid opcodes. Small integer pushes share one variant carrying the value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Opcode {
    /// `0x01` integer add.
    Iadd,
    /// `0x02` integer subtract.
    Isub,
    /// `0x03` integer multiply.
    Imul,
    /// `0x04` integer divide.
    Idiv,
    /// `0x05` integer modulo.
    Imod,
    /// `0x06` logical negation.
    Inot,
    /// `0x07` integer negation.
    Ineg,
    /// `0x08` integer equality.
    Ieq,
    /// `0x09` integer inequality.
    Ine,
    /// `0x0A` integer greater-than.
    Igt,
    /// `0x0B` integer greater-or-equal.
    Ige,
    /// `0x0C` integer less-than.
    Ilt,
    /// `0x0D` integer less-or-equal.
    Ile,
    /// `0x0E` float add.
    Fadd,
    /// `0x0F` float subtract.
    Fsub,
    /// `0x10` float multiply.
    Fmul,
    /// `0x11` float divide.
    Fdiv,
    /// `0x12` float modulo.
    Fmod,
    /// `0x13` float negation.
    Fneg,
    /// `0x14` float equality.
    Feq,
    /// `0x15` float inequality.
    Fne,
    /// `0x16` float greater-than.
    Fgt,
    /// `0x17` float greater-or-equal.
    Fge,
    /// `0x18` float less-than.
    Flt,
    /// `0x19` float less-or-equal.
    Fle,
    /// `0x1A` vector add.
    Vadd,
    /// `0x1B` vector subtract.
    Vsub,
    /// `0x1C` component-wise vector multiply.
    Vmul,
    /// `0x1D` component-wise vector divide.
    Vdiv,
    /// `0x1E` vector negation.
    Vneg,
    /// `0x1F` bitwise and.
    Iand,
    /// `0x20` bitwise or.
    Ior,
    /// `0x21` bitwise xor.
    Ixor,
    /// `0x22` unconditional jump to an absolute code offset.
    J,
    /// `0x23` jump when the top of stack is zero.
    Jz,
    /// `0x24` jump when the top of stack is non-zero.
    Jnz,
    /// `0x25` integer to float conversion.
    I2f,
    /// `0x26` float to integer conversion.
    F2i,
    /// `0x27` float to vector (triplicated).
    F2v,
    /// `0x28` push a `u16` constant.
    PushConstU16,
    /// `0x29` push a `u32` constant.
    PushConstU32,
    /// `0x2A` push an `f32` constant.
    PushConstF,
    /// `0x2B` duplicate the top of stack.
    Dup,
    /// `0x2C` drop the top of stack.
    Drop,
    /// `0x2D` call a native command by hash.
    Native,
    /// `0x2E` call a code offset, pushing the return address.
    Call,
    /// `0x2F` open a call frame.
    Enter,
    /// `0x30` close a call frame and return.
    Leave,
    /// `0x31` load through an address.
    Load,
    /// `0x32` store through an address.
    Store,
    /// `0x33` store through an address, keeping the address.
    StoreRev,
    /// `0x34` load N values through an address.
    LoadN,
    /// `0x35` store N values through an address.
    StoreN,
    /// `0x36` push the address of frame slot 0.
    Local0,
    /// `0x37` push the address of frame slot 1.
    Local1,
    /// `0x38` push the address of frame slot 2.
    Local2,
    /// `0x39` push the address of frame slot 3.
    Local3,
    /// `0x3A` push the address of frame slot 4.
    Local4,
    /// `0x3B` push the address of frame slot 5.
    Local5,
    /// `0x3C` push the address of frame slot 6.
    Local6,
    /// `0x3D` push the address of frame slot 7.
    Local7,
    /// `0x3E` add a stack index to a frame base address.
    Local,
    /// `0x3F` add a stack index to the statics base address.
    Static,
    /// `0x40` add a stack index to the globals base address.
    Global,
    /// `0x41` index into an array by item size.
    Array,
    /// `0x42` jump table over `u32` case values.
    Switch,
    /// `0x43` push a pointer to an inline NUL-terminated string.
    String,
    /// `0x44` push a null pointer.
    Null,
    /// `0x45` copy a string into a text label with a length cap.
    TextLabelAssignString,
    /// `0x46` format an integer into a text label with a length cap.
    TextLabelAssignInt,
    /// `0x47` append a string to a text label with a length cap.
    TextLabelAppendString,
    /// `0x48` format and append an integer with a length cap.
    TextLabelAppendInt,
    /// `0x49` arm the single catch handler slot.
    Catch,
    /// `0x4A` raise to the catch handler, or kill the thread.
    Throw,
    /// `0x4B` copy one text label to another.
    TextLabelCopy,
    /// `0x4C` PC-only: load through a protected address.
    XprotectLoad,
    /// `0x4D` PC-only: store through a protected address.
    XprotectStore,
    /// `0x4E` PC-only: borrow an unprotected alias for a protected address.
    XprotectRef,
    /// `0x50..=0xFF` push a small integer constant (`-16..=159`).
    PushConst(i16),
}

impl Opcode {
    /// Decode the opcode byte. Returns `None` for `0x00` and `0x4F`.
    #[must_use]
    pub fn from_byte(byte: u8) -> Option<Opcode> {
        let op = match byte {
            0x01 => Opcode::Iadd,
            0x02 => Opcode::Isub,
            0x03 => Opcode::Imul,
            0x04 => Opcode::Idiv,
            0x05 => Opcode::Imod,
            0x06 => Opcode::Inot,
            0x07 => Opcode::Ineg,
            0x08 => Opcode::Ieq,
            0x09 => Opcode::Ine,
            0x0A => Opcode::Igt,
            0x0B => Opcode::Ige,
            0x0C => Opcode::Ilt,
            0x0D => Opcode::Ile,
            0x0E => Opcode::Fadd,
            0x0F => Opcode::Fsub,
            0x10 => Opcode::Fmul,
            0x11 => Opcode::Fdiv,
            0x12 => Opcode::Fmod,
            0x13 => Opcode::Fneg,
            0x14 => Opcode::Feq,
            0x15 => Opcode::Fne,
            0x16 => Opcode::Fgt,
            0x17 => Opcode::Fge,
            0x18 => Opcode::Flt,
            0x19 => Opcode::Fle,
            0x1A => Opcode::Vadd,
            0x1B => Opcode::Vsub,
            0x1C => Opcode::Vmul,
            0x1D => Opcode::Vdiv,
            0x1E => Opcode::Vneg,
            0x1F => Opcode::Iand,
            0x20 => Opcode::Ior,
            0x21 => Opcode::Ixor,
            0x22 => Opcode::J,
            0x23 => Opcode::Jz,
            0x24 => Opcode::Jnz,
            0x25 => Opcode::I2f,
            0x26 => Opcode::F2i,
            0x27 => Opcode::F2v,
            0x28 => Opcode::PushConstU16,
            0x29 => Opcode::PushConstU32,
            0x2A => Opcode::PushConstF,
            0x2B => Opcode::Dup,
            0x2C => Opcode::Drop,
            0x2D => Opcode::Native,
            0x2E => Opcode::Call,
            0x2F => Opcode::Enter,
            0x30 => Opcode::Leave,
            0x31 => Opcode::Load,
            0x32 => Opcode::Store,
            0x33 => Opcode::StoreRev,
            0x34 => Opcode::LoadN,
            0x35 => Opcode::StoreN,
            0x36 => Opcode::Local0,
            0x37 => Opcode::Local1,
            0x38 => Opcode::Local2,
            0x39 => Opcode::Local3,
            0x3A => Opcode::Local4,
            0x3B => Opcode::Local5,
            0x3C => Opcode::Local6,
            0x3D => Opcode::Local7,
            0x3E => Opcode::Local,
            0x3F => Opcode::Static,
            0x40 => Opcode::Global,
            0x41 => Opcode::Array,
            0x42 => Opcode::Switch,
            0x43 => Opcode::String,
            0x44 => Opcode::Null,
            0x45 => Opcode::TextLabelAssignString,
            0x46 => Opcode::TextLabelAssignInt,
            0x47 => Opcode::TextLabelAppendString,
            0x48 => Opcode::TextLabelAppendInt,
            0x49 => Opcode::Catch,
            0x4A => Opcode::Throw,
            0x4B => Opcode::TextLabelCopy,
            0x4C => Opcode::XprotectLoad,
            0x4D => Opcode::XprotectStore,
            0x4E => Opcode::XprotectRef,
            0x50..=0xFF => Opcode::PushConst(i16::from(byte) - 0x60),
            0x00 | 0x4F => return None,
        };
        Some(op)
    }

    /// The canonical uppercase mnemonic used in public documentation.
    #[must_use]
    pub fn mnemonic(&self) -> String {
        let name = match self {
            Opcode::Iadd => "IADD",
            Opcode::Isub => "ISUB",
            Opcode::Imul => "IMUL",
            Opcode::Idiv => "IDIV",
            Opcode::Imod => "IMOD",
            Opcode::Inot => "INOT",
            Opcode::Ineg => "INEG",
            Opcode::Ieq => "IEQ",
            Opcode::Ine => "INE",
            Opcode::Igt => "IGT",
            Opcode::Ige => "IGE",
            Opcode::Ilt => "ILT",
            Opcode::Ile => "ILE",
            Opcode::Fadd => "FADD",
            Opcode::Fsub => "FSUB",
            Opcode::Fmul => "FMUL",
            Opcode::Fdiv => "FDIV",
            Opcode::Fmod => "FMOD",
            Opcode::Fneg => "FNEG",
            Opcode::Feq => "FEQ",
            Opcode::Fne => "FNE",
            Opcode::Fgt => "FGT",
            Opcode::Fge => "FGE",
            Opcode::Flt => "FLT",
            Opcode::Fle => "FLE",
            Opcode::Vadd => "VADD",
            Opcode::Vsub => "VSUB",
            Opcode::Vmul => "VMUL",
            Opcode::Vdiv => "VDIV",
            Opcode::Vneg => "VNEG",
            Opcode::Iand => "IAND",
            Opcode::Ior => "IOR",
            Opcode::Ixor => "IXOR",
            Opcode::J => "J",
            Opcode::Jz => "JZ",
            Opcode::Jnz => "JNZ",
            Opcode::I2f => "I2F",
            Opcode::F2i => "F2I",
            Opcode::F2v => "F2V",
            Opcode::PushConstU16 => "PUSH_CONST_U16",
            Opcode::PushConstU32 => "PUSH_CONST_U32",
            Opcode::PushConstF => "PUSH_CONST_F",
            Opcode::Dup => "DUP",
            Opcode::Drop => "DROP",
            Opcode::Native => "NATIVE",
            Opcode::Call => "CALL",
            Opcode::Enter => "ENTER",
            Opcode::Leave => "LEAVE",
            Opcode::Load => "LOAD",
            Opcode::Store => "STORE",
            Opcode::StoreRev => "STORE_REV",
            Opcode::LoadN => "LOAD_N",
            Opcode::StoreN => "STORE_N",
            Opcode::Local0 => "LOCAL_0",
            Opcode::Local1 => "LOCAL_1",
            Opcode::Local2 => "LOCAL_2",
            Opcode::Local3 => "LOCAL_3",
            Opcode::Local4 => "LOCAL_4",
            Opcode::Local5 => "LOCAL_5",
            Opcode::Local6 => "LOCAL_6",
            Opcode::Local7 => "LOCAL_7",
            Opcode::Local => "LOCAL",
            Opcode::Static => "STATIC",
            Opcode::Global => "GLOBAL",
            Opcode::Array => "ARRAY",
            Opcode::Switch => "SWITCH",
            Opcode::String => "STRING",
            Opcode::Null => "NULL",
            Opcode::TextLabelAssignString => "TEXT_LABEL_ASSIGN_STRING",
            Opcode::TextLabelAssignInt => "TEXT_LABEL_ASSIGN_INT",
            Opcode::TextLabelAppendString => "TEXT_LABEL_APPEND_STRING",
            Opcode::TextLabelAppendInt => "TEXT_LABEL_APPEND_INT",
            Opcode::Catch => "CATCH",
            Opcode::Throw => "THROW",
            Opcode::TextLabelCopy => "TEXT_LABEL_COPY",
            Opcode::XprotectLoad => "XPROTECT_LOAD",
            Opcode::XprotectStore => "XPROTECT_STORE",
            Opcode::XprotectRef => "XPROTECT_REF",
            Opcode::PushConst(v) => {
                return if *v < 0 {
                    format!("PUSH_CONST_M{}", -v)
                } else {
                    format!("PUSH_CONST_{v}")
                };
            }
        };
        name.to_string()
    }

    /// Whether this opcode changes control flow (jumps, calls, returns, throws, switches).
    #[must_use]
    pub fn is_control_flow(&self) -> bool {
        matches!(
            self,
            Opcode::J
                | Opcode::Jz
                | Opcode::Jnz
                | Opcode::Call
                | Opcode::Leave
                | Opcode::Switch
                | Opcode::Throw
        )
    }
}

/// Decoded operands of one instruction.
#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    /// No operands.
    None,
    /// One length byte (text label opcodes).
    Len(u8),
    /// A `u16` constant.
    U16(u16),
    /// A `u32` constant or absolute code offset.
    U32(u32),
    /// An `f32` constant.
    F32(f32),
    /// `ENTER`: argument count and frame size in slots.
    Enter {
        /// Number of stack arguments to the function.
        argc: u8,
        /// Frame size in 32-bit slots.
        frame_size: u16,
    },
    /// `LEAVE`: argument count and return count.
    Leave {
        /// Number of stack arguments taken.
        argc: u8,
        /// Number of return values left on the stack.
        retc: u8,
    },
    /// `NATIVE`: argument count, return count and native hash.
    Native {
        /// Number of stack arguments passed to the native.
        argc: u8,
        /// Number of return values the native pushes.
        retc: u8,
        /// Native command hash.
        hash: u32,
    },
    /// `SWITCH`: ordered (value, target offset) pairs.
    Switch(Vec<SwitchCase>),
    /// `STRING`: raw string bytes without the trailing NUL.
    String(Vec<u8>),
}

/// One `SWITCH` case: jump to `target` when the top of stack equals `value`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwitchCase {
    /// Case value compared against the top of stack.
    pub value: u32,
    /// Absolute code offset to jump to on a match.
    pub target: u32,
}

/// Errors from [`decode_one`] and [`decode_all`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IsaError {
    /// The opcode byte is not a valid instruction.
    InvalidOpcode {
        /// Offset of the bad byte.
        offset: u32,
        /// The byte value.
        byte: u8,
    },
    /// The instruction runs past the end of the code.
    Truncated {
        /// Offset where the instruction starts.
        offset: u32,
        /// Bytes needed past `offset`.
        needed: u32,
        /// Bytes available past `offset`.
        available: u32,
    },
    /// A `STRING` operand is missing its trailing NUL.
    UnterminatedString {
        /// Offset where the instruction starts.
        offset: u32,
    },
}

impl core::fmt::Display for IsaError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IsaError::InvalidOpcode { offset, byte } => {
                write!(f, "invalid opcode 0x{byte:02X} at offset {offset}")
            }
            IsaError::Truncated {
                offset,
                needed,
                available,
            } => write!(
                f,
                "truncated instruction at offset {offset}: need {needed} bytes, have {available}"
            ),
            IsaError::UnterminatedString { offset } => {
                write!(f, "string at offset {offset} lacks its trailing NUL")
            }
        }
    }
}

impl std::error::Error for IsaError {}

fn read_u16(code: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([code[at], code[at + 1]])
}

fn read_u32(code: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([code[at], code[at + 1], code[at + 2], code[at + 3]])
}

/// Decode the single instruction starting at `offset`.
///
/// Returns the instruction with its size in bytes, or an [`IsaError`]
/// describing why the bytes do not decode. Never panics.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
// One arm per opcode; splitting would scatter the ISA table.
#[allow(clippy::too_many_lines)]
pub fn decode_one(code: &[u8], offset: usize) -> Result<Instruction, IsaError> {
    let avail = u32::try_from(code.len().saturating_sub(offset)).unwrap_or(u32::MAX);
    let Some(&first) = code.get(offset) else {
        return Err(IsaError::Truncated {
            offset: u32::try_from(offset).unwrap_or(u32::MAX),
            needed: 1,
            available: 0,
        });
    };
    let Some(opcode) = Opcode::from_byte(first) else {
        return Err(IsaError::InvalidOpcode {
            offset: u32::try_from(offset).unwrap_or(u32::MAX),
            byte: first,
        });
    };
    let rest = &code[offset + 1..];
    // Helper: require n operand bytes to be present.
    let need = |n: u32| -> Result<(), IsaError> {
        if avail > n {
            Ok(())
        } else {
            Err(IsaError::Truncated {
                offset: u32::try_from(offset).unwrap_or(u32::MAX),
                needed: 1 + n,
                available: avail,
            })
        }
    };
    let (size, operand) = match opcode {
        Opcode::J | Opcode::Jz | Opcode::Jnz | Opcode::Call | Opcode::PushConstU32 => {
            need(4)?;
            (5, Operand::U32(read_u32(code, offset + 1)))
        }
        Opcode::PushConstU16 => {
            need(2)?;
            (3, Operand::U16(read_u16(code, offset + 1)))
        }
        Opcode::PushConstF => {
            need(4)?;
            (
                5,
                Operand::F32(f32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]])),
            )
        }
        Opcode::Native => {
            need(6)?;
            (
                7,
                Operand::Native {
                    argc: rest[0],
                    retc: rest[1],
                    hash: read_u32(code, offset + 3),
                },
            )
        }
        Opcode::Enter => {
            need(3)?;
            (
                4,
                Operand::Enter {
                    argc: rest[0],
                    frame_size: read_u16(code, offset + 2),
                },
            )
        }
        Opcode::Leave => {
            need(2)?;
            (
                3,
                Operand::Leave {
                    argc: rest[0],
                    retc: rest[1],
                },
            )
        }
        Opcode::Switch => {
            need(1)?;
            let count = u32::from(rest[0]);
            let total = 2u32.saturating_add(count.saturating_mul(8));
            if avail < total {
                return Err(IsaError::Truncated {
                    offset: u32::try_from(offset).unwrap_or(u32::MAX),
                    needed: total,
                    available: avail,
                });
            }
            let mut cases = Vec::with_capacity(count as usize);
            for i in 0..count as usize {
                let base = offset + 2 + i * 8;
                cases.push(SwitchCase {
                    value: read_u32(code, base),
                    target: read_u32(code, base + 4),
                });
            }
            (total, Operand::Switch(cases))
        }
        Opcode::String => {
            need(1)?;
            let len = u32::from(rest[0]);
            let total = 2 + len;
            if avail < total {
                return Err(IsaError::Truncated {
                    offset: u32::try_from(offset).unwrap_or(u32::MAX),
                    needed: total,
                    available: avail,
                });
            }
            let mut bytes = rest[1..=(len as usize)].to_vec();
            if len == 0 || bytes[(len - 1) as usize] != 0 {
                return Err(IsaError::UnterminatedString {
                    offset: u32::try_from(offset).unwrap_or(u32::MAX),
                });
            }
            bytes.pop();
            (total, Operand::String(bytes))
        }
        Opcode::TextLabelAssignString
        | Opcode::TextLabelAssignInt
        | Opcode::TextLabelAppendString
        | Opcode::TextLabelAppendInt => {
            need(1)?;
            (2, Operand::Len(rest[0]))
        }
        _ => (1, Operand::None),
    };
    Ok(Instruction {
        offset: u32::try_from(offset).unwrap_or(u32::MAX),
        size,
        opcode,
        operand,
    })
}

/// Decode the whole code segment with a linear sweep.
///
/// Stops at the first undecodable instruction and returns the error.
/// An empty segment yields an empty vector.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn decode_all(code: &[u8]) -> Result<Vec<Instruction>, IsaError> {
    let mut out = Vec::new();
    let mut offset = 0usize;
    while offset < code.len() {
        let inst = decode_one(code, offset)?;
        offset += inst.size as usize;
        out.push(inst);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_const_range_maps() {
        assert_eq!(Opcode::from_byte(0x50), Some(Opcode::PushConst(-16)));
        assert_eq!(Opcode::from_byte(0x5F), Some(Opcode::PushConst(-1)));
        assert_eq!(Opcode::from_byte(0x60), Some(Opcode::PushConst(0)));
        assert_eq!(Opcode::from_byte(0xFF), Some(Opcode::PushConst(159)));
        assert_eq!(Opcode::from_byte(0x00), None);
        assert_eq!(Opcode::from_byte(0x4F), None);
        assert_eq!(Opcode::PushConst(-3).mnemonic(), "PUSH_CONST_M3");
        assert_eq!(Opcode::PushConst(42).mnemonic(), "PUSH_CONST_42");
        assert_eq!(Opcode::Native.mnemonic(), "NATIVE");
    }

    #[test]
    fn fixed_operands_decode() {
        // Hand-built: ENTER 2 args frame 5; NATIVE 1 0 hash; J to 0x10.
        let code = [
            0x2F, 0x02, 0x05, 0x00, 0x2D, 0x01, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
        ];
        let a = decode_one(&code, 0).unwrap();
        assert_eq!(
            a.operand,
            Operand::Enter {
                argc: 2,
                frame_size: 5
            }
        );
        assert_eq!(a.size, 4);
        let b = decode_one(&code, 4).unwrap();
        assert_eq!(
            b.operand,
            Operand::Native {
                argc: 1,
                retc: 0,
                hash: 0xDEADBEEF
            }
        );
        assert_eq!(b.size, 7);
        let c = decode_one(&[0x22, 0x10, 0x00, 0x00, 0x00], 0).unwrap();
        assert_eq!(c.operand, Operand::U32(0x10));
    }

    #[test]
    fn switch_and_string_decode() {
        // Hand-built SWITCH with 2 cases.
        let code = [
            0x42, 0x02, 0x01, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
            0x30, 0x00, 0x00, 0x00,
        ];
        let inst = decode_one(&code, 0).unwrap();
        assert_eq!(inst.size, 18);
        assert_eq!(
            inst.operand,
            Operand::Switch(vec![
                SwitchCase {
                    value: 1,
                    target: 0x20
                },
                SwitchCase {
                    value: 2,
                    target: 0x30
                },
            ])
        );
        // Hand-built STRING "hi\0" (length 3 incl. NUL).
        let s = decode_one(&[0x43, 0x03, b'h', b'i', 0x00], 0).unwrap();
        assert_eq!(s.size, 5);
        assert_eq!(s.operand, Operand::String(b"hi".to_vec()));
        // Missing NUL is an error, not silent acceptance.
        assert!(matches!(
            decode_one(&[0x43, 0x02, b'h', b'i'], 0),
            Err(IsaError::UnterminatedString { .. })
        ));
    }

    #[test]
    fn errors_carry_offsets() {
        assert_eq!(
            decode_one(&[0x00], 0),
            Err(IsaError::InvalidOpcode { offset: 0, byte: 0 })
        );
        assert_eq!(
            decode_one(&[0x22, 0x01], 0),
            Err(IsaError::Truncated {
                offset: 0,
                needed: 5,
                available: 2
            })
        );
        assert!(decode_all(&[]).unwrap().is_empty());
        assert_eq!(
            decode_all(&[0x60, 0x00]).unwrap_err(),
            IsaError::InvalidOpcode { offset: 1, byte: 0 }
        );
    }
}
