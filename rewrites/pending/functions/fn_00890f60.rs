// original: 0x00890f60 DEFAULT_ROLLOFF
/// Apply a parameter block to a target record with scaling and id caching.
///
/// Copies the id dword, two float-scaled word parameters (each multiplied by
/// a fixed factor, and additionally by the target's current value unless a
/// bypass flag is set or the source tag is 0xff), a flag byte (merged or
/// stored), and the low 6 bits of a second flag byte. When the source tag is
/// 0xff and its id is 0, the id comes from a lazily resolved cached value
/// (resolved once through the name-lookup helper, stubbed by the checker).
/// Returns the incidental EAX the original leaves behind (built from the last
/// parameter word and flag bytes), reproduced exactly.
export!(thiscall, rw_00890f60(this: u32, tgt: u32, src: u32) -> u32 {
    // Fixed scale factor, measured from the original's data section.
    const K: f32 = f32::from_bits(0x3C23_D70A);
    unsafe {
        let bypass = ((this + 5) as *const u8).read() == 0xff;
        let id = ((src + 0x2d) as *const u32).read_unaligned();
        if bypass || id != 0 {
            (tgt as *mut u32).write(id);
        }
        let w1 = ((src + 0x2b) as *const u16).read_unaligned() as f32;
        let v1 = if !bypass && ((((this + 8) as *const u8).read()) & 0x80) == 0 {
            w1 * K * ((tgt + 0x0c) as *const f32).read_unaligned()
        } else {
            w1 * K
        };
        ((tgt + 0x0c) as *mut f32).write_unaligned(v1);
        let eid = ((src + 0x31) as *const u32).read_unaligned();
        if !bypass {
            if eid != 0 {
                ((tgt + 4) as *mut u32).write(eid);
            }
        } else if eid != 0 {
            ((tgt + 4) as *mut u32).write(eid);
        } else {
            let flagp = global::<u32>(0x0115D99C);
            if (*flagp & 1) != 0 {
                ((tgt + 4) as *mut u32)
                    .write(*global::<u32>(0x0115D998));
            } else {
                *flagp |= 1;
                let ans: u32 =
                    callee_cdecl!(1, u32, relocated(0x00E78500), 0);
                *global::<u32>(0x0115D998) = ans;
                ((tgt + 4) as *mut u32).write(ans);
            }
        }
        let w2 = ((src + 0x35) as *const u16).read_unaligned() as f32;
        let v2 = if !bypass && ((((this + 9) as *const u8).read()) & 2) == 0 {
            w2 * K * ((tgt + 8) as *const f32).read_unaligned()
        } else {
            w2 * K
        };
        ((tgt + 8) as *mut f32).write_unaligned(v2);
        let flag37 = ((src + 0x37) as *const u8).read();
        if !bypass && ((((this + 9) as *const u8).read()) & 4) == 0 {
            let p = (tgt + 0x14) as *mut u8;
            p.write(p.read() | flag37);
        } else {
            ((tgt + 0x14) as *mut u8).write(flag37);
        }
        let w2bits = ((src + 0x35) as *const u16).read_unaligned() as u32;
        let mut eax = (w2bits & !0xff) | (flag37 as u32);
        let cur15 = ((tgt + 0x15) as *const u8).read();
        if (cur15 & 0x3f) == 0 {
            let new15 = (cur15 & 0xc0)
                | ((((src + 0x38) as *const u8).read()) & 0x3f);
            ((tgt + 0x15) as *mut u8).write(new15);
            eax = (eax & !0xff) | (new15 as u32);
        }
        eax
    }
});
