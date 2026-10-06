// original: 0x00C68CD0 sentinel_chain_or_table_scan (proposed)

/// Match a key against a chain of eighteen sentinel globals, else scan an
/// indexed pointer table for a flagged entry.
///
/// `this` is a controller object (only forwarded to helpers); `arg0` is
/// the key. Each sentinel compares the key against its global: on a match
/// it calls the probe helper one to three times with pairs of globals (or
/// a carried value) and returns the probe answer's high bytes with bit 0
/// set as soon as one answers nonzero, otherwise it falls through to the
/// next sentinel. When no sentinel matches, or every probe answered zero,
/// the key indexes a pointer table (`TABLE_ADDR`): when the entry's flag
/// word at `+0x70` is not 1 the function returns the entry's high bytes;
/// otherwise it asks a count helper and, for a positive SIGNED count,
/// asks an index helper per slot and returns the first index whose entry
/// carries flag 1 (high bytes with bit 0 set), or the last index's high
/// bytes when no slot matches. A non-positive count returns the count's
/// high bytes. All comparisons are SIGNED where the original branches on
/// them (the count bound); sentinel identity is exact u32 equality.
///
/// Original: 0x00C68CD0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00C68CD0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const S1: u32 = 0x012F_9E28;
        const S2: u32 = 0x012F_A020;
        const S3: u32 = 0x012F_A53C;
        const S4: u32 = 0x012F_A608;
        const S5: u32 = 0x012F_A26C;
        const S6: u32 = 0x012F_9FD8;
        const S7: u32 = 0x012F_A2E4;
        const S8: u32 = 0x012F_9DD4;
        const S9: u32 = 0x012F_A23C;
        const S10: u32 = 0x012F_A5FC;
        const S11: u32 = 0x012F_A590;
        const S12: u32 = 0x012F_9EB8;
        const S13: u32 = 0x012F_9E10;
        const S14: u32 = 0x012F_9DEC;
        const S15: u32 = 0x012F_9F84;
        const S16: u32 = 0x012F_A338;
        const S17: u32 = 0x012F_9FC0;
        const S18: u32 = 0x012F_A0F8;
        const G_ADDR: u32 = 0x012B_4138;
        const TABLE_ADDR: u32 = 0x0129_5CD8;
        const FLAG_OFF: u32 = 0x70;
        const HI_MASK: u32 = 0xFFFF_FF00;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn probe(x: u32, g: u32) -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(1, u32, x, g) }
        }

        let key = arg0;
        let g = g32(G_ADDR);
        if key == g32(S1) {
            let a = probe(g32(S2), g);
            if (a as u8) != 0 {
                return (a & HI_MASK) | 1;
            }
            let b = probe(g32(S3), g);
            if (b as u8) != 0 {
                return (b & HI_MASK) | 1;
            }
        }
        let mut eax = g32(S2);
        if key == eax {
            let a = probe(g32(S3), g);
            if (a as u8) != 0 {
                return (a & HI_MASK) | 1;
            }
            let b = probe(g32(S3), g);
            if (b as u8) != 0 {
                return (b & HI_MASK) | 1;
            }
        }
        if key == g32(S3) {
            let a = probe(eax, g);
            if (a as u8) != 0 {
                return (a & HI_MASK) | 1;
            }
        }
        eax = g32(S4);
        if key == eax {
            let a = probe(g32(S5), g);
            if (a as u8) != 0 {
                return (a & HI_MASK) | 1;
            }
            let b = probe(g32(S6), g);
            if (b as u8) != 0 {
                return (b & HI_MASK) | 1;
            }
            let c = probe(g32(S7), g);
            if (c as u8) != 0 {
                return (c & HI_MASK) | 1;
            }
        }
        eax = g32(S4);
        if key == g32(S5) {
            let a = probe(eax, g);
            if (a as u8) != 0 {
                return (a & HI_MASK) | 1;
            }
            let b = probe(g32(S6), g);
            if (b as u8) != 0 {
                return (b & HI_MASK) | 1;
            }
            let c = probe(g32(S7), g);
            if (c as u8) != 0 {
                return (c & HI_MASK) | 1;
            }
        }
        eax = g32(S4);
        if key == g32(S6) || key == g32(S7) {
            let a = probe(eax, g);
            if (a as u8) != 0 {
                return (a & HI_MASK) | 1;
            }
            let b = probe(g32(S5), g);
            if (b as u8) != 0 {
                return (b & HI_MASK) | 1;
            }
        } else {
            eax = g32(S8);
            if key == eax {
                let a = probe(g32(S9), g);
                if (a as u8) != 0 {
                    return (a & HI_MASK) | 1;
                }
                let b = probe(g32(S10), g);
                if (b as u8) != 0 {
                    return (b & HI_MASK) | 1;
                }
            }
            if key == g32(S9) {
                let a = probe(eax, g);
                if (a as u8) != 0 {
                    return (a & HI_MASK) | 1;
                }
            }
            if key == g32(S10) {
                let a = probe(eax, g);
                if (a as u8) != 0 {
                    return (a & HI_MASK) | 1;
                }
                let b = probe(g32(S9), g);
                if (b as u8) != 0 {
                    return (b & HI_MASK) | 1;
                }
            }
            eax = g32(S11);
            if key == eax {
                let a = probe(g32(S12), g);
                if (a as u8) != 0 {
                    return (a & HI_MASK) | 1;
                }
            }
            if key == g32(S12) {
                let a = probe(eax, g);
                if (a as u8) != 0 {
                    return (a & HI_MASK) | 1;
                }
            }
            eax = g32(S13);
            if key == eax {
                let a = probe(g32(S14), g);
                if (a as u8) != 0 {
                    return (a & HI_MASK) | 1;
                }
            }
            if key == g32(S14) {
                let a = probe(eax, g);
                if (a as u8) != 0 {
                    return (a & HI_MASK) | 1;
                }
            }
            eax = g32(S15);
            if key == eax {
                let a = probe(g32(S16), g);
                if (a as u8) != 0 {
                    return (a & HI_MASK) | 1;
                }
            }
            if key == g32(S16) {
                let a = probe(eax, g);
                if (a as u8) != 0 {
                    return (a & HI_MASK) | 1;
                }
            }
            eax = g32(S17);
            if key == eax {
                let a = probe(g32(S18), g);
                if (a as u8) != 0 {
                    return (a & HI_MASK) | 1;
                }
            }
            if key == g32(S18) {
                let a = probe(eax, g);
                if (a as u8) != 0 {
                    return (a & HI_MASK) | 1;
                }
            }
        }
        let entry = g32(TABLE_ADDR.wrapping_add(key.wrapping_mul(4)));
        if rd32(entry.wrapping_add(FLAG_OFF)) != 1 {
            return entry & HI_MASK;
        }
        let count = lf_checker_rt::callee_thiscall!(2, u32, this) as i32;
        if count <= 0 {
            return (count as u32) & HI_MASK;
        }
        let mut last: u32 = 0;
        let mut i: i32 = 0;
        loop {
            if i >= count {
                break;
            }
            let idx = lf_checker_rt::callee_thiscall!(3, u32, this, i as u32);
            last = idx;
            let e = g32(TABLE_ADDR.wrapping_add(idx.wrapping_mul(4)));
            if rd32(e.wrapping_add(FLAG_OFF)) == 1 {
                return (idx & HI_MASK) | 1;
            }
            i += 1;
        }
        last & HI_MASK
    }
});
