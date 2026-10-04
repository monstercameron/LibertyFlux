//! One-line-per-instruction disassembly.
//!
//! [`disassemble`] decodes a whole code segment and [`format_instruction`]
//! renders one instruction as text: zero-padded offset, mnemonic, operands.
//! Native hashes print as names when a [`NativeDb`](crate::natives::NativeDb)
//! that knows them is supplied, otherwise as eight hex digits.

use crate::isa::{Instruction, IsaError, Opcode, Operand, decode_all};
use crate::natives::NativeDb;
use std::fmt::Write as _;

/// Decode a whole code segment with a linear sweep.
///
/// This is [`decode_all`](crate::isa::decode_all) re-exported under the
/// disassembler: every byte must belong to exactly one instruction.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn disassemble(code: &[u8]) -> Result<Vec<Instruction>, IsaError> {
    decode_all(code)
}

/// Render one instruction on a single line.
///
/// Layout: `000000 MNEMONIC operands`. Jump targets and call addresses print
/// as zero-padded offsets; `NATIVE` prints `argc retc name-or-hash`.
#[must_use]
pub fn format_instruction(inst: &Instruction, natives: Option<&NativeDb>) -> String {
    let mut out = format!("{:06} {}", inst.offset, inst.opcode.mnemonic());
    match &inst.operand {
        Operand::None => {}
        Operand::Len(n) => write!(out, " {n}").expect("writing to a String cannot fail"),
        Operand::U16(n) => write!(out, " {n} (0x{n:X})").expect("writing to a String cannot fail"),
        Operand::U32(n) => match inst.opcode {
            Opcode::J | Opcode::Jz | Opcode::Jnz | Opcode::Call => {
                write!(out, " {n:06}").expect("writing to a String cannot fail");
            }
            Opcode::PushConstU32 => {
                write!(out, " {} (0x{n:X})", i32::from_ne_bytes(n.to_ne_bytes()))
                    .expect("writing to a String cannot fail");
            }
            _ => write!(out, " {n}").expect("writing to a String cannot fail"),
        },
        Operand::F32(f) => write!(out, " {f}").expect("writing to a String cannot fail"),
        Operand::Enter { argc, frame_size } => {
            write!(out, " {argc} {frame_size}").expect("writing to a String cannot fail");
        }
        Operand::Leave { argc, retc } => {
            write!(out, " {argc} {retc}").expect("writing to a String cannot fail");
        }
        Operand::Native { argc, retc, hash } => match natives.and_then(|db| db.lookup(*hash)) {
            Some(name) => {
                write!(out, " {argc} {retc} {name}").expect("writing to a String cannot fail");
            }
            None => {
                write!(out, " {argc} {retc} 0x{hash:08X}").expect("writing to a String cannot fail");
            }
        },
        Operand::Switch(cases) => {
            for case in cases {
                write!(out, " {}:{:06}", case.value, case.target)
                    .expect("writing to a String cannot fail");
            }
        }
        Operand::String(bytes) => {
            write!(out, " '{}'", escape(bytes)).expect("writing to a String cannot fail");
        }
    }
    out
}

/// Escape string bytes for one-line display: printable ASCII as-is, common
/// escapes symbolic, everything else as `\xNN`.
#[must_use]
pub fn escape(bytes: &[u8]) -> String {
    let mut out = String::new();
    for &b in bytes {
        match b {
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            b'\'' => out.push_str("\\'"),
            b'\\' => out.push_str("\\\\"),
            0x20..=0x7E => out.push(b as char),
            _ => write!(out, "\\x{b:02X}").expect("writing to a String cannot fail"),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isa::SwitchCase;

    fn inst(opcode: Opcode, operand: Operand) -> Instruction {
        Instruction {
            offset: 7,
            size: 1,
            opcode,
            operand,
        }
    }

    #[test]
    fn lines_render() {
        // Hand-built instructions covering each operand shape.
        assert_eq!(
            format_instruction(&inst(Opcode::Dup, Operand::None), None),
            "000007 DUP"
        );
        assert_eq!(
            format_instruction(&inst(Opcode::J, Operand::U32(0x20)), None),
            "000007 J 000032"
        );
        assert_eq!(
            format_instruction(&inst(Opcode::PushConstU16, Operand::U16(5000)), None),
            "000007 PUSH_CONST_U16 5000 (0x1388)"
        );
        assert_eq!(
            format_instruction(
                &inst(
                    Opcode::Native,
                    Operand::Native {
                        argc: 1,
                        retc: 0,
                        hash: 0xABCD
                    }
                ),
                None
            ),
            "000007 NATIVE 1 0 0x0000ABCD"
        );
        assert_eq!(
            format_instruction(
                &inst(
                    Opcode::Switch,
                    Operand::Switch(vec![SwitchCase {
                        value: 1,
                        target: 9
                    }])
                ),
                None
            ),
            "000007 SWITCH 1:000009"
        );
        assert_eq!(
            format_instruction(
                &inst(Opcode::String, Operand::String(b"a\nb".to_vec())),
                None
            ),
            "000007 STRING 'a\\nb'"
        );
    }

    #[test]
    fn native_names_resolve() {
        let db = NativeDb::from_p0_natives_json(
            r#"[{"hash_int": 43981, "name": "FAKE_WAIT", "alias_names": []}]"#,
        )
        .unwrap();
        let line = format_instruction(
            &inst(
                Opcode::Native,
                Operand::Native {
                    argc: 1,
                    retc: 0,
                    hash: 43981,
                },
            ),
            Some(&db),
        );
        assert_eq!(line, "000007 NATIVE 1 0 FAKE_WAIT");
    }
}
