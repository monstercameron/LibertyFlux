// original: 0x008926B0 audsound_voice_position_query
/// Answers the voice position query from the bound bank or the live voice.
///
/// When the bank-use dword at `this+0x98` is zero, uses the sample bank:
/// returns -1 unless the enable byte at `this+0x4d` is set, `this+0x7c`
/// points at a bank, and `key` is below the bank count at `[bank+8]`
/// (unsigned); otherwise returns one plus the interpolate callee's (cdecl:
/// the two dwords of entry `key`) answer. Otherwise the voice path: returns
/// -1 when `this+0x94` is null; reads the inner object and, for each of the
/// two (possibly null, contributing -1) ends, queries virtual slot +0x14
/// (thiscall, no stack words) and +0x18 for the pair; feeds the pair to the
/// combine callee (cdecl: second answer, first answer) and returns -1 when
/// the scaled position `((([this+0xc] - [this+0x10]) >> 1 - 0x800) * key) >> 2`
/// (all wrapping unsigned, shifts logical) reaches the combined answer
/// (unsigned); otherwise returns one plus the interpolate callee's
/// (second... precisely (cdecl: position, first answer)) answer.
/// Original: 0x008926B0 (thiscall, one stack word: key).
export!(thiscall, rw_008926B0(this: *mut u8, key: u32) -> u32 {
    unsafe {
        const QUERY_A: u32 = 1;
        const QUERY_B: u32 = 2;
        const COMBINE: u32 = 3;
        const INTERP: u32 = 4;
        const SLOT_A: u32 = 0x14;
        const SLOT_B: u32 = 0x18;
        if *(this.add(0x98) as *const u32) == 0 {
            if *this.add(0x4d) == 0 {
                return 0xffff_ffff;
            }
            let bank = *(this.add(0x7c) as *const u32);
            if bank == 0 {
                return 0xffff_ffff;
            }
            let n = ((bank.wrapping_add(8)) as *const u32).read_unaligned();
            if key >= n {
                return 0xffff_ffff;
            }
            let arr = (bank as *const u32).read_unaligned();
            let lo = ((arr.wrapping_add(key.wrapping_mul(8))) as *const u32).read_unaligned();
            let hi =
                ((arr.wrapping_add(key.wrapping_mul(8)).wrapping_add(4)) as *const u32)
                    .read_unaligned();
            let r: u32 = callee_cdecl!(INTERP, u32, lo, hi);
            return r.wrapping_add(1);
        }
        let a = *(this.add(0x94) as *const u32);
        if a == 0 {
            return 0xffff_ffff;
        }
        let inner = (a as *const u32).read_unaligned();
        let first = if inner == 0 {
            0xffff_ffff
        } else {
            let vt = (inner as *const u32).read_unaligned();
            let target = ((vt.wrapping_add(SLOT_A)) as *const u32).read_unaligned();
            let q: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
            q(inner)
        };
        let second = if inner == 0 {
            0xffff_ffff
        } else {
            let vt = (inner as *const u32).read_unaligned();
            let target = ((vt.wrapping_add(SLOT_B)) as *const u32).read_unaligned();
            let q: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
            q(inner)
        };
        let top = *(this.add(0x0c) as *const u32);
        let bot = *(this.add(0x10) as *const u32);
        let pos = top
            .wrapping_sub(bot)
            .wrapping_shr(1)
            .wrapping_sub(0x800)
            .wrapping_mul(key)
            .wrapping_shr(2);
        let limit: u32 = callee_cdecl!(COMBINE, u32, second, first);
        if pos >= limit {
            return 0xffff_ffff;
        }
        let r: u32 = callee_cdecl!(INTERP, u32, pos, first);
        r.wrapping_add(1)
    }
});
