//! Minimal x86-32 instruction-length decoder.
//!
//! The hook engine only needs to move whole instructions, so this decoder
//! returns lengths plus branch classification, never full disassembly.
//! Bias is fail-safe: unknown encodings return `None` (refuse the hook)
//! rather than a guessed length; where an obsolete encoding is ambiguous
//! the longer, position-independent reading is preferred.

/// How an instruction transfers control, for trampoline rewriting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchKind {
    /// Not a relative branch (fall through).
    None,
    /// `EB` / `70..7F`: 2-byte instruction, rel8 at offset 1.
    Rel8,
    /// `E8` / `E9` / `0F 80..8F`: rel16/32 at the end.
    RelFull,
    /// `E0..E3` (`LOOPcc` / JECXZ): not relocatable, the hook is refused.
    Loop,
}

/// One decoded instruction: length plus branch classification.
#[derive(Clone, Copy, Debug)]
pub struct Decoded {
    /// Total instruction length in bytes.
    pub len: u8,
    /// Branch classification (see [`BranchKind`]).
    pub branch: BranchKind,
    /// Size in bytes of the relative displacement (1, 2 or 4); 0 if none.
    pub rel_size: u8,
    /// `ModRM` (mod, reg, rm) when the instruction has one.
    pub modrm: Option<(u8, u8, u8)>,
    /// Displacement value when a `ModRM` displacement is present.
    pub modrm_disp: Option<i32>,
    /// True for `RET`/`RET imm16`: patching must stop before it.
    pub is_ret: bool,
}

/// Decode one instruction at the start of `buf` (32-bit mode).
#[must_use]
pub fn decode(buf: &[u8]) -> Option<Decoded> {
    let mut i = 0usize;
    let mut op16 = false;
    let mut addr16 = false;
    // Legacy prefixes.
    loop {
        let b = *buf.get(i)?;
        match b {
            0xF0 | 0xF2 | 0xF3 | 0x2E | 0x36 | 0x3E | 0x26 | 0x64 | 0x65 => i += 1,
            0x66 => {
                op16 = true;
                i += 1;
            }
            0x67 => {
                addr16 = true;
                i += 1;
            }
            _ => break,
        }
        if i >= 15 {
            return None;
        }
    }
    let start = i;
    let op = *buf.get(i)?;
    i += 1;

    let mut out = Decoded {
        len: 0,
        branch: BranchKind::None,
        rel_size: 0,
        modrm: None,
        modrm_disp: None,
        is_ret: false,
    };
    // Immediate sizes selected by operand size.
    let iz: u8 = if op16 { 2 } else { 4 }; // IZ / rel full
    let moffs: u8 = if addr16 { 2 } else { 4 };

    // Helper closures need the buffer; use small local fns via macro-free code.
    enum Need {
        Fixed(u8),
        ModRm(Imm),
        Rel8(BranchKind),
        RelFull,
        FarPtr,
        Moffs,
        Enter,
        RetImm,
        Escape,
    }
    #[derive(Clone, Copy)]
    enum Imm {
        None,
        I8,
        Iz,
        /// Group 3 (F6/F7): imm only when /reg is 0 or 1.
        Grp3,
    }

    let need = match op {
        0x00..=0x03
        | 0x08..=0x0B
        | 0x10..=0x13
        | 0x18..=0x1B
        | 0x20..=0x23
        | 0x28..=0x2B
        | 0x30..=0x33
        | 0x38..=0x3B => Need::ModRm(Imm::None),
        0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C | 0xA8 => Need::Fixed(2),
        0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D | 0xA9 => Need::Fixed(1 + iz),
        0x62 | 0x63 => Need::ModRm(Imm::None),
        0x69 => Need::ModRm(Imm::Iz),
        0x6B => Need::ModRm(Imm::I8),
        0x80 => Need::ModRm(Imm::I8),
        0x81 => Need::ModRm(Imm::Iz),
        0x83 => Need::ModRm(Imm::I8),
        0x84..=0x8F => Need::ModRm(Imm::None),
        0x9A => Need::FarPtr,
        0xA0..=0xA3 => Need::Moffs,
        0xB0..=0xB7 => Need::Fixed(2),
        0xB8..=0xBF => Need::Fixed(1 + iz),
        0xC0 | 0xC1 => Need::ModRm(Imm::I8),
        0xC2 | 0xCA => Need::RetImm,
        0xC3 | 0xCB | 0xC9 => {
            out.is_ret = true;
            Need::Fixed(1)
        }
        0xC4 | 0xC5 => Need::ModRm(Imm::None),
        0xC6 => Need::ModRm(Imm::I8),
        0xC7 => Need::ModRm(Imm::Iz),
        0xC8 => Need::Enter,
        0xCC | 0xCE | 0xCF => Need::Fixed(1),
        0xCD | 0xD4 | 0xD5 => Need::Fixed(2),
        0xD0..=0xDF => Need::ModRm(Imm::None),
        0xE0..=0xE3 => Need::Rel8(BranchKind::Loop),
        0xE4..=0xE7 => Need::Fixed(2),
        0xE8 => {
            out.branch = BranchKind::RelFull;
            Need::RelFull
        }
        0xE9 => {
            out.branch = BranchKind::RelFull;
            Need::RelFull
        }
        0xEA => Need::FarPtr,
        0xEB => Need::Rel8(BranchKind::Rel8),
        0xF6 => Need::ModRm(Imm::Grp3),
        0xF7 => Need::ModRm(Imm::Grp3),
        0xFE | 0xFF => Need::ModRm(Imm::None),
        0x68 => Need::Fixed(1 + iz),
        0x6A => Need::Fixed(2),
        0x70..=0x7F => Need::Rel8(BranchKind::Rel8),
        0x0F => Need::Escape,
        _ => Need::Fixed(1), // Single-byte opcodes (push/pop/inc/dec/flags/string/io/...).
    };

    match need {
        Need::Fixed(n) => {
            i = start + n as usize;
        }
        Need::Moffs => {
            i = start + 1 + moffs as usize;
        }
        Need::FarPtr => {
            i = start + if op16 { 5 } else { 7 };
        }
        Need::Enter => {
            i = start + 4;
        }
        Need::RetImm => {
            out.is_ret = true;
            i = start + 3;
        }
        Need::Rel8(k) => {
            out.branch = k;
            out.rel_size = 1;
            i = start + 2;
        }
        Need::RelFull => {
            out.rel_size = iz;
            i = start + 1 + iz as usize;
        }
        Need::ModRm(imm) => {
            let (m, reg, rm, extra, disp) = parse_modrm(buf, i, addr16)?;
            out.modrm = Some((m, reg, rm));
            out.modrm_disp = disp;
            i += extra;
            let imm_len = match imm {
                Imm::None => 0,
                Imm::I8 => 1,
                Imm::Iz => iz as usize,
                Imm::Grp3 => {
                    if reg < 2 {
                        (if op == 0xF6 { 1 } else { iz }) as usize
                    } else {
                        0
                    }
                }
            };
            i += imm_len;
        }
        Need::Escape => {
            let op2 = *buf.get(i)?;
            i += 1;
            let start2 = start; // lengths measured from first opcode byte below
            match op2 {
                0x00..=0x03 | 0x0D | 0x18 => {
                    let (_, _, _, extra, disp) = parse_modrm(buf, i, addr16)?;
                    out.modrm_disp = disp;
                    i += extra;
                }
                0x0F => {
                    let (_, _, _, extra, disp) = parse_modrm(buf, i, addr16)?;
                    out.modrm_disp = disp;
                    i += extra + 1; // 3DNow! opcode suffix
                }
                0x10..=0x17 | 0x19..=0x1F | 0x20..=0x24 | 0x26 | 0x28..=0x2F => {
                    let (_, _, _, extra, disp) = parse_modrm(buf, i, addr16)?;
                    out.modrm_disp = disp;
                    i += extra;
                }
                0x38 => {
                    let _op3 = *buf.get(i)?;
                    i += 1;
                    let (_, _, _, extra, disp) = parse_modrm(buf, i, addr16)?;
                    out.modrm_disp = disp;
                    i += extra;
                }
                0x3A => {
                    let _op3 = *buf.get(i)?;
                    i += 1;
                    let (_, _, _, extra, disp) = parse_modrm(buf, i, addr16)?;
                    out.modrm_disp = disp;
                    i += extra + 1; // imm8
                }
                0x40..=0x76 | 0x78 | 0x79 | 0x7E | 0x7F => {
                    let (_, _, _, extra, disp) = parse_modrm(buf, i, addr16)?;
                    out.modrm_disp = disp;
                    i += extra;
                }
                0x80..=0x8F => {
                    out.branch = BranchKind::RelFull;
                    out.rel_size = iz;
                    i = start2 + 2 + iz as usize;
                }
                0x90..=0x9F | 0xA3 | 0xA5 | 0xAB | 0xAD | 0xAE | 0xAF => {
                    let (_, _, _, extra, disp) = parse_modrm(buf, i, addr16)?;
                    out.modrm_disp = disp;
                    i += extra;
                }
                0xA4 | 0xAC | 0xC2 | 0xC4 | 0xC5 | 0xC6 => {
                    let (_, _, _, extra, disp) = parse_modrm(buf, i, addr16)?;
                    out.modrm_disp = disp;
                    i += extra + 1;
                }
                0xB0..=0xB9 | 0xBB..=0xC1 | 0xC3 | 0xC7 | 0xD0..=0xF6 | 0xF8..=0xFE => {
                    if op2 == 0xBA {
                        let (_, reg, _, extra, disp) = parse_modrm(buf, i, addr16)?;
                        out.modrm_disp = disp;
                        i += extra;
                        if reg != 4 {
                            i += 1;
                        }
                    } else {
                        let (_, _, _, extra, disp) = parse_modrm(buf, i, addr16)?;
                        out.modrm_disp = disp;
                        i += extra;
                    }
                }
                0xF7 => {
                    if op16 {
                        // MASKMOVDQU has a ModRM; MASKMOVQ does not.
                        let (_, _, _, extra, disp) = parse_modrm(buf, i, addr16)?;
                        out.modrm_disp = disp;
                        i += extra;
                    }
                }
                _ => {
                    // 0F 04/05/06/07/08/09/0B/0E/25/27/77/A0/A1/A2/A6/A7/A8/A9/AA/
                    // 30..37/FF and friends: no ModRM, 2 bytes total.
                    match op2 {
                        0x05
                        | 0x06
                        | 0x07
                        | 0x08
                        | 0x09
                        | 0x0B
                        | 0x0E
                        | 0x30
                        | 0x31
                        | 0x32
                        | 0x33
                        | 0x34
                        | 0x35
                        | 0x37
                        | 0x77
                        | 0xA0
                        | 0xA1
                        | 0xA2
                        | 0xA8
                        | 0xA9
                        | 0xC8..=0xCF
                        | 0xFF => {}
                        _ => return None,
                    }
                }
            }
            let _ = start2;
        }
    }

    // 0F 70..73 carry an imm8 the range arm above did not add; fix up.
    if op == 0x0F
        && let Some(op2) = buf.get(start + 1).copied()
        && (0x70..=0x73).contains(&op2)
    {
        // The 0x40..=0x76 arm consumed ModRM only; add the imm8.
        // (Decode it again cheaply: length grows by exactly one.)
        i += 1;
    }

    // Decode always starts at buf[0], so `i` is the full length.
    if i == 0 || i > 15 || i > buf.len() {
        return None;
    }
    out.len = i as u8;
    Some(out)
}

/// Parse `ModRM` (+SIB + displacement) at `buf[i]`.
///
/// Returns (mod, reg, rm, bytes consumed past `ModRM` incl. SIB/disp, disp value).
fn parse_modrm(buf: &[u8], i: usize, addr16: bool) -> Option<(u8, u8, u8, usize, Option<i32>)> {
    let b = *buf.get(i)?;
    let m = (b >> 6) & 3;
    let reg = (b >> 3) & 7;
    let rm = b & 7;
    let mut n = 1usize;
    let mut disp: Option<i32> = None;
    if addr16 {
        match (m, rm) {
            (0, 6) => {
                disp = Some(i32::from(read_i16(buf, i + n)?));
                n += 2;
            }
            (1, _) => {
                disp = Some(i32::from(*buf.get(i + n)? as i8));
                n += 1;
            }
            (2, _) => {
                disp = Some(i32::from(read_i16(buf, i + n)?));
                n += 2;
            }
            _ => {}
        }
        return Some((m, reg, rm, n, disp));
    }
    let mut has_sib = false;
    if m != 3 && rm == 4 {
        has_sib = true;
        n += 1;
    }
    match m {
        0 => {
            if rm == 5 {
                disp = Some(read_i32(buf, i + n)?);
                n += 4;
            } else if has_sib {
                let sib = *buf.get(i + 1)?;
                if sib & 7 == 5 {
                    disp = Some(read_i32(buf, i + n)?);
                    n += 4;
                }
            }
        }
        1 => {
            disp = Some(i32::from(*buf.get(i + n)? as i8));
            n += 1;
        }
        2 => {
            disp = Some(read_i32(buf, i + n)?);
            n += 4;
        }
        _ => {}
    }
    Some((m, reg, rm, n, disp))
}

fn read_i16(buf: &[u8], i: usize) -> Option<i16> {
    let b = buf.get(i..i + 2)?;
    Some(i16::from_le_bytes([b[0], b[1]]))
}

fn read_i32(buf: &[u8], i: usize) -> Option<i32> {
    let b = buf.get(i..i + 4)?;
    Some(i32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn len_of(bytes: &[u8]) -> Option<u8> {
        decode(bytes).map(|d| d.len)
    }

    #[test]
    fn single_byte_opcodes() {
        assert_eq!(len_of(&[0x90]), Some(1)); // nop
        assert_eq!(len_of(&[0x55]), Some(1)); // push ebp
        assert_eq!(len_of(&[0x53]), Some(1)); // push ebx
        assert_eq!(len_of(&[0x50]), Some(1)); // push eax
        assert_eq!(len_of(&[0x5D]), Some(1)); // pop ebp
        assert_eq!(len_of(&[0xC3]), Some(1)); // ret
        assert_eq!(len_of(&[0xCC]), Some(1)); // int3
        assert_eq!(len_of(&[0xF4]), Some(1)); // hlt
    }

    #[test]
    fn common_prologues() {
        // push ebp; mov ebp,esp; sub esp,0x10
        assert_eq!(len_of(&[0x55]), Some(1));
        assert_eq!(len_of(&[0x8B, 0xEC]), Some(2));
        assert_eq!(len_of(&[0x83, 0xEC, 0x10]), Some(3));
        assert_eq!(len_of(&[0x81, 0xEC, 0x00, 0x01, 0x00, 0x00]), Some(6));
        // mov edi,edi (hot-patchable prologue)
        assert_eq!(len_of(&[0x8B, 0xFF]), Some(2));
    }

    #[test]
    fn immediates() {
        assert_eq!(len_of(&[0x68, 1, 2, 3, 4]), Some(5)); // push imm32
        assert_eq!(len_of(&[0x6A, 0xFF]), Some(2)); // push imm8
        assert_eq!(len_of(&[0xB8, 1, 2, 3, 4]), Some(5)); // mov eax,imm32
        assert_eq!(len_of(&[0xB0, 0x11]), Some(2)); // mov al,imm8
        assert_eq!(len_of(&[0xC2, 0x08, 0x00]), Some(3)); // ret 8
        assert_eq!(len_of(&[0xC8, 0x10, 0x00, 0x00]), Some(4)); // enter
        assert_eq!(len_of(&[0xCD, 0x2E]), Some(2)); // int
        assert_eq!(len_of(&[0xE8, 1, 2, 3, 4]), Some(5)); // call rel32
        assert_eq!(len_of(&[0xE9, 1, 2, 3, 4]), Some(5)); // jmp rel32
        assert_eq!(len_of(&[0xEB, 0xFE]), Some(2)); // jmp rel8
        assert_eq!(len_of(&[0x74, 0x10]), Some(2)); // jz rel8
        assert_eq!(len_of(&[0xA1, 1, 2, 3, 4]), Some(5)); // mov eax,moffs32
    }

    #[test]
    fn branches_classified() {
        let d = decode(&[0xE8, 0, 0, 0, 0]).unwrap();
        assert_eq!(d.branch, BranchKind::RelFull);
        assert_eq!(d.rel_size, 4);
        let d = decode(&[0x74, 0]).unwrap();
        assert_eq!(d.branch, BranchKind::Rel8);
        let d = decode(&[0x0F, 0x84, 0, 0, 0, 0]).unwrap();
        assert_eq!((d.len, d.branch), (6, BranchKind::RelFull));
        let d = decode(&[0xE2, 0xFE]).unwrap();
        assert_eq!(d.branch, BranchKind::Loop);
        let d = decode(&[0xC3]).unwrap();
        assert!(d.is_ret);
    }

    #[test]
    fn modrm_sib_disp() {
        // mov eax,[ebp+8]
        assert_eq!(len_of(&[0x8B, 0x45, 0x08]), Some(3));
        // mov eax,[0x12345678]
        assert_eq!(len_of(&[0x8B, 0x05, 0x78, 0x56, 0x34, 0x12]), Some(6));
        // jmp dword ptr [0x11223344]  (import thunk shape)
        let d = decode(&[0xFF, 0x25, 0x44, 0x33, 0x22, 0x11]).unwrap();
        assert_eq!(d.len, 6);
        assert_eq!(d.modrm, Some((0, 4, 5)));
        assert_eq!(d.modrm_disp, Some(0x1122_3344));
        // mov eax,[ebx+ecx*4+0x10]  (SIB + disp8)
        assert_eq!(len_of(&[0x8B, 0x44, 0x8B, 0x10]), Some(4));
        // mov eax,[ecx*4+0x11223344]  (SIB, no base)
        assert_eq!(len_of(&[0x8B, 0x04, 0x8D, 0x44, 0x33, 0x22, 0x11]), Some(7));
    }

    #[test]
    fn groups() {
        // test eax,eax
        assert_eq!(len_of(&[0x85, 0xC0]), Some(2));
        // push dword ptr [eax]  (FF /6)
        assert_eq!(len_of(&[0xFF, 0x30]), Some(2));
        // call dword ptr [eax]  (FF /2)
        assert_eq!(len_of(&[0xFF, 0x10]), Some(2));
        // test byte ptr [eax],1  (F6 /0 imm8)
        assert_eq!(len_of(&[0xF6, 0x00, 0x01]), Some(3));
        // not eax  (F7 /2, no imm)
        assert_eq!(len_of(&[0xF7, 0xD0]), Some(2));
        // mul eax  (F7 /4)
        assert_eq!(len_of(&[0xF7, 0xE0]), Some(2));
        // shl eax,1  (D1 /4)
        assert_eq!(len_of(&[0xD1, 0xE0]), Some(2));
        // rol eax,8  (C1 /0 imm8)
        assert_eq!(len_of(&[0xC1, 0xC0, 0x08]), Some(3));
    }

    #[test]
    fn operand_size_prefix() {
        // mov ax,0x1234
        assert_eq!(len_of(&[0x66, 0xB8, 0x34, 0x12]), Some(4));
        // push 0x1234
        assert_eq!(len_of(&[0x66, 0x68, 0x34, 0x12]), Some(4));
        // rep movsd
        assert_eq!(len_of(&[0xF3, 0xA5]), Some(2));
        // fs:mov eax,[0]
        assert_eq!(len_of(&[0x64, 0xA1, 0, 0, 0, 0]), Some(6));
    }

    #[test]
    fn sse_two_byte() {
        // movaps xmm0,xmm1
        assert_eq!(len_of(&[0x0F, 0x28, 0xC1]), Some(3));
        // pxor xmm0,xmm0
        assert_eq!(len_of(&[0x66, 0x0F, 0xEF, 0xC0]), Some(4));
        // cmpps xmm0,xmm1,0
        assert_eq!(len_of(&[0x0F, 0xC2, 0xC1, 0x00]), Some(4));
        // setz al
        assert_eq!(len_of(&[0x0F, 0x94, 0xC0]), Some(3));
        // bswap eax
        assert_eq!(len_of(&[0x0F, 0xC8]), Some(2));
        // rdtsc
        assert_eq!(len_of(&[0x0F, 0x31]), Some(2));
        // ud2
        assert_eq!(len_of(&[0x0F, 0x0B]), Some(2));
    }

    #[test]
    fn truncated_and_invalid() {
        assert_eq!(len_of(&[]), None);
        assert_eq!(len_of(&[0x8B]), None); // needs ModRM
        assert_eq!(len_of(&[0xE8, 1, 2]), None); // truncated rel32
        assert_eq!(len_of(&[0x0F]), None);
        assert_eq!(len_of(&[0x0F, 0x04]), None); // invalid second byte
    }
}
