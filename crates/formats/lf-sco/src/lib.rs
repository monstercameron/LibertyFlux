//! Reader and disassembler for Grand Theft Auto IV compiled scripts (SCO).
//!
//! This crate parses the version-14 script container used by the PC build,
//! optionally decrypting and decompressing it, and decodes the stack-machine
//! bytecode into instructions with a one-line-per-instruction disassembler.
//! Native call hashes can be resolved to names through a JSON table loaded
//! at run time (see [`natives`]).
//!
//! # Container layout (all integers little-endian)
//!
//! ```text
//! offset  size  field
//! 0       4     magic: "SCR\x0E" plain, "scr\x0E" AES, "Scr\x0E" AES + zlib
//! 4       4     code length in bytes
//! 8       4     statics count (32-bit slots)
//! 12      4     globals count (32-bit slots)
//! 16      4     args count (trailing slots of the statics segment)
//! 20      4     globals signature (0 when the script declares no globals)
//! 24      ...   payload, depending on magic
//! ```
//!
//! Payload by magic:
//!
//! - Plain: code bytes, then statics (`count * 4` bytes), then globals.
//!   The game itself does not load this variant; tools use it.
//! - AES: same three segments, each encrypted independently (see [`crypto`]).
//!   This is the variant used by all but one shipped script.
//! - AES + zlib: one `u32` compressed size, then that many bytes which are
//!   decrypted and then zlib-inflated into code + statics + globals
//!   concatenated. Not observed in shipped files; supported for tool output.
//!
//! One leftover file uses the version-13 magic `SCR\r`: same header and
//! segments, unencrypted. See [`container::MAGIC_V13_PLAIN`].
//!
//! The args are not a separate segment: the last `args count` slots of the
//! statics array are the script's launch arguments. Strings live inline in
//! the code stream as operands of the `STRING` instruction; there is no
//! string table.
//!
//! # Bytecode
//!
//! One-byte opcodes, operands inline. See [`isa`] for the opcode table with
//! operand shapes. The program counter addresses bytes from the start of
//! the code segment; jump targets are absolute byte offsets. Values on the
//! stack and in the segments are all 32 bits wide (see [`ScriptValue`]).
//!
//! # Example
//!
//! ```no_run
//! use lf_sco::{container, natives};
//!
//! let bytes = std::fs::read("mission.sco").unwrap();
//! // Plain files pass `None`; encrypted files need the 32-byte key in memory.
//! let script = container::load(&bytes, None).unwrap();
//! println!("code bytes: {}", script.code.len());
//! let db = natives::NativeDb::from_p0_natives_json(
//!     &std::fs::read_to_string("natives.json").unwrap(),
//! ).unwrap();
//! for line in lf_sco::disasm::disassemble(&script.code)
//!     .unwrap()
//!     .iter()
//!     .map(|i| lf_sco::disasm::format_instruction(i, Some(&db)))
//! {
//!     println!("{line}");
//! }
//! ```

pub mod container;
pub mod crypto;
pub mod disasm;
pub mod isa;
pub mod key;
pub mod natives;

pub use container::{Header, LoadError, Script, ScriptValue};
pub use isa::{Instruction, IsaError, Opcode, Operand, SwitchCase, decode_all, decode_one};
pub use natives::NativeDb;
