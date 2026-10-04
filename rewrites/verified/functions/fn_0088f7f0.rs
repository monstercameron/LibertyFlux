// original: 0x0088f7f0 audio voice accumulate into mix target (unnamed)
#[inline(always)]
unsafe fn r8(base: *mut u8, off: u32) -> u8 {
    *base.add(off as usize)
}

#[inline(always)]
unsafe fn r16(base: *mut u8, off: u32) -> u16 {
    *(base.add(off as usize) as *const u16)
}

#[inline(always)]
unsafe fn r32(base: *mut u8, off: u32) -> u32 {
    *(base.add(off as usize) as *const u32)
}

#[inline(always)]
unsafe fn rf(base: *mut u8, off: u32) -> f32 {
    *(base.add(off as usize) as *const f32)
}

/// Fold one voice's levels into its mix target: resolve the target through
/// the global voice table, add the two gain words and blend the two level
/// floats, merge the two selector words with the -1-means-unset rule, carry
/// over the tag byte, then report the pair to the mixer.
export!(thiscall, rw_0088f7f0(this: *mut u8) -> u32 {
    unsafe {
        const STRIDE_SLOT: u32 = 0x115D964;
        const TABLE_SLOT: u32 = 0x115D988;
        const MIXER: u32 = 0x115D8A0;
        const ROW: u32 = 0x6F40;
        const BIAS: u32 = 0x6F10;
        let stride: u32 = *global::<u32>(STRIDE_SLOT);
        let table: u32 = *global::<u32>(TABLE_SLOT);
        let resolve = |sel: u8| {
            if sel == 0xFF {
                0u32
            } else {
                let row = (r8(this, 0x40) as u32).wrapping_mul(ROW);
                let cell = table.wrapping_add(row).wrapping_add(BIAS);
                stride
                    .wrapping_mul(sel as u32)
                    .wrapping_add(*(cell as *const u32))
            }
        };
        // Merge two selector words: -1 means unset and loses to any value.
        let merge = |a: i16, b: i16| {
            if a == -1 {
                b
            } else if b == -1 {
                a
            } else {
                a.wrapping_add(b)
            }
        };
        let tgt = resolve(r8(this, 0x48));
        *((tgt.wrapping_add(5)) as *mut u8) = r8(this, 5);
        let t20 = *((tgt.wrapping_add(0x20)) as *const u16) as i16;
        *((tgt.wrapping_add(0x20)) as *mut u16) =
            t20.wrapping_add(r16(this, 0x20) as i16) as u16;
        let blended = rf(this, 0x1C) + *((tgt.wrapping_add(0x1C)) as *const f32);
        *((tgt.wrapping_add(0x1C)) as *mut f32) = blended;
        let m = merge(
            *((tgt.wrapping_add(0x0A)) as *const u16) as i16,
            r16(this, 0x0A) as i16,
        );
        *((tgt.wrapping_add(0x0A)) as *mut u16) = m as u16;
        let d = r32(this, 0x88);
        if d != 0xFFFF_FFFF {
            *((tgt.wrapping_add(0x88)) as *mut u32) = d;
        }
        let t22 = *((tgt.wrapping_add(0x22)) as *const u16) as i16;
        *((tgt.wrapping_add(0x22)) as *mut u16) =
            t22.wrapping_add(r16(this, 0x22) as i16) as u16;
        let blended2 = rf(this, 0x24) + *((tgt.wrapping_add(0x24)) as *const f32);
        *((tgt.wrapping_add(0x24)) as *mut f32) = blended2;
        let m2 = merge(
            *((tgt.wrapping_add(0x7C)) as *const u16) as i16,
            r16(this, 0x7C) as i16,
        );
        *((tgt.wrapping_add(0x7C)) as *mut u16) = m2 as u16;
        let tag = r8(this, 4);
        if tag != 0xFF {
            *((tgt.wrapping_add(4)) as *mut u8) = tag;
            *this.add(4) = 0xFF;
        }
        let tgt2 = resolve(r8(this, 5));
        let _: u32 = callee_thiscall!(1, u32, tgt2, this as u32, tgt);
        let idx = r8(this, 0x40) as u32;
        *this.add(0x39) |= 0x40;
        let r: u32 = callee_thiscall!(2, u32, relocated(MIXER), this as u32, idx);
        r
    }
});
