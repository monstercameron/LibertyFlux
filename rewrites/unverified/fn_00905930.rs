// original: 0x00905930 input_slot_configure (proposed)

/// Configure one input slot from freshly sampled values: fill the frame
/// with samples, resolve the slot, and fan the samples out to the slot's
/// record and the per-field setters.
///
/// Samples the value helper (cdecl, pointer and length) into the frame:
/// the selector word, the record index, a word, a 16-byte block, a float
/// and another word; picks one of three sample paths by the mode flags
/// (a 30-byte block plus a compare helper call, with or without a second
/// fill and a flag pulse, or a 60-byte block); then samples eight more
/// words, the table-2 index and the shared 8-byte global. The record
/// index selects `TABLE` entries whose flag byte (`+0x08`) gates stores.
///
/// When the enable byte argument is zero, or the selector is negative,
/// or the resolve helper (cdecl, index and selector) answers -1, returns
/// 0. Else the record takes the low word at `+0x00` and a dword at
/// `+0x04`; the combine helper (cdecl, selector and block pointer) runs;
/// a clear flag on the index record returns 1; else the record takes a
/// word at `+0x20`, three field setters run, a set flag on the sample-28
/// record stores a float at `+0x40` (otherwise the field call below takes
/// the sample), three more setters run, a set flag on the selector
/// record stores a float at `+0x50`, two more setters run, and the finish
/// helper (cdecl, selector and 0, or the table-2 entry with the index
/// decremented) runs before returning 1. Returns the last helper answer
/// with its low byte forced to 1 (0 on the early paths). Ends with the
/// standard cookie check. Original: 0x00905930 (cdecl, one byte word).
lf_checker_rt::export!(cdecl, rw_00905930(arg: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0118F6F8;
        const TABLE2: u32 = 0x0118F4EC;
        const FLAG_A: u32 = 0x0116D27D;
        const FLAG_B: u32 = 0x0116D27E;
        const FLAG_PULSE: u32 = 0x0116D27F;
        const SHARED: u32 = 0x01193C5C;
        const COOKIE: u32 = 0x01057FB4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn entry(table: u32, i: u32) -> u32 {
            unsafe { rd32(table.wrapping_add(i.wrapping_mul(4))) }
        }

        let tab = lf_checker_rt::relocated(TABLE);
        let tab2 = lf_checker_rt::relocated(TABLE2);
        let mut frame = [0u32; 56];
        let base = core::hint::black_box(frame.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(1, u32, base.wrapping_add(0x10));
        lf_checker_rt::callee_cdecl!(11, u32, base.wrapping_add(4), 4u32);
        lf_checker_rt::callee_cdecl!(12, u32, base.wrapping_add(0x0c), 1u32);
        lf_checker_rt::callee_cdecl!(13, u32, base.wrapping_add(0x30), 2u32);
        lf_checker_rt::callee_cdecl!(14, u32, base.wrapping_add(0x40), 0x10u32);
        lf_checker_rt::callee_cdecl!(15, u32, base.wrapping_add(0x50), 4u32);
        lf_checker_rt::callee_cdecl!(16, u32, base.wrapping_add(0x34), 4u32);

        let fa = ((lf_checker_rt::relocated(FLAG_A)) as *const u8).read();
        if fa != 0 {
            lf_checker_rt::callee_cdecl!(17, u32, base.wrapping_add(0xbc), 0x1eu32);
            lf_checker_rt::callee_cdecl!(
                2,
                u32,
                base.wrapping_add(0xbc),
                base.wrapping_add(0x70)
            );
            ((lf_checker_rt::relocated(FLAG_PULSE)) as *mut u8).write(1);
            lf_checker_rt::callee_cdecl!(17, u32, base.wrapping_add(0xbc), 0x1eu32);
            ((lf_checker_rt::relocated(FLAG_PULSE)) as *mut u8).write(0);
        } else {
            let fb = ((lf_checker_rt::relocated(FLAG_B)) as *const u8).read();
            if fb != 0 {
                lf_checker_rt::callee_cdecl!(18, u32, base.wrapping_add(0x70), 0x3cu32);
            } else {
                lf_checker_rt::callee_cdecl!(17, u32, base.wrapping_add(0xbc), 0x1eu32);
                lf_checker_rt::callee_cdecl!(
                    2,
                    u32,
                    base.wrapping_add(0xbc),
                    base.wrapping_add(0x70)
                );
            }
        }

        lf_checker_rt::callee_cdecl!(19, u32, base.wrapping_add(0x10), 2u32);
        lf_checker_rt::callee_cdecl!(20, u32, base.wrapping_add(0x14), 4u32);
        lf_checker_rt::callee_cdecl!(21, u32, base.wrapping_add(0x54), 4u32);
        lf_checker_rt::callee_cdecl!(22, u32, base.wrapping_add(0x58), 4u32);
        lf_checker_rt::callee_cdecl!(23, u32, base.wrapping_add(0x5c), 4u32);
        lf_checker_rt::callee_cdecl!(24, u32, base.wrapping_add(0x60), 4u32);
        lf_checker_rt::callee_cdecl!(25, u32, base.wrapping_add(0x64), 4u32);
        lf_checker_rt::callee_cdecl!(26, u32, base.wrapping_add(0x68), 1u32);
        lf_checker_rt::callee_cdecl!(27, u32, base.wrapping_add(8), 4u32);
        let mut last =
            lf_checker_rt::callee_cdecl!(28, u32, lf_checker_rt::relocated(SHARED), 8u32);

        let cookie = rd32(lf_checker_rt::relocated(COOKIE));
        if (arg as u8) == 0 {
            lf_checker_rt::callee_thiscall!(34, u32, cookie);
            return last & 0xFFFFFF00;
        }
        let s4 = rd32(base.wrapping_add(4));
        if (s4 as i32) < 0 {
            lf_checker_rt::callee_thiscall!(34, u32, cookie);
            return last & 0xFFFFFF00;
        }
        last = lf_checker_rt::callee_cdecl!(3, u32, rd32(base.wrapping_add(0x0c)), s4);
        wr32(base.wrapping_add(4), last);
        if last == 0xFFFFFFFF {
            lf_checker_rt::callee_thiscall!(34, u32, cookie);
            return last & 0xFFFFFF00;
        }
        let rec = entry(tab, last);
        wr16(rec, rd16(base.wrapping_add(0x10)));
        let rec = entry(tab, rd32(base.wrapping_add(4)));
        wr32(rec.wrapping_add(4), rd32(base.wrapping_add(0x14)));
        last = lf_checker_rt::callee_cdecl!(
            4,
            u32,
            rd32(base.wrapping_add(4)),
            base.wrapping_add(0x40)
        );
        let rec = entry(tab, rd32(base.wrapping_add(0x0c)));
        if ((rec.wrapping_add(8) as *const u8).read()) == 0 {
            lf_checker_rt::callee_thiscall!(34, u32, cookie);
            return (last & 0xFFFFFF00) | 1;
        }
        wr16(rec.wrapping_add(0x20), rd16(base.wrapping_add(0x30)));
        last = lf_checker_rt::callee_cdecl!(
            5,
            u32,
            rd32(base.wrapping_add(4)),
            rd32(base.wrapping_add(0x34))
        );
        last = lf_checker_rt::callee_cdecl!(
            6,
            u32,
            rd32(base.wrapping_add(4)),
            base.wrapping_add(0x70)
        );
        last = lf_checker_rt::callee_cdecl!(7, u32, rd32(base.wrapping_add(8)), 0x40u32);
        let mut farg = rd32(base.wrapping_add(0x1c));
        let rec = entry(tab, farg);
        if ((rec.wrapping_add(8) as *const u8).read()) != 0 {
            wr32(rec.wrapping_add(0x40), rd32(base.wrapping_add(0x50)));
            farg = rd32(base.wrapping_add(4));
        }
        last = lf_checker_rt::callee_cdecl!(8, u32, farg, rd32(base.wrapping_add(0x54)));
        last = lf_checker_rt::callee_cdecl!(
            9,
            u32,
            rd32(base.wrapping_add(4)),
            rd32(base.wrapping_add(0x58))
        );
        last = lf_checker_rt::callee_cdecl!(
            10,
            u32,
            rd32(base.wrapping_add(4)),
            rd32(base.wrapping_add(0x5c))
        );
        let mut eax = rd32(base.wrapping_add(4));
        let rec = entry(tab, eax);
        if ((rec.wrapping_add(8) as *const u8).read()) != 0 {
            wr32(rec.wrapping_add(0x50), rd32(base.wrapping_add(0x60)));
            eax = rd32(base.wrapping_add(4));
        }
        last = lf_checker_rt::callee_cdecl!(31, u32, eax, rd32(base.wrapping_add(0x64)));
        last = lf_checker_rt::callee_cdecl!(
            32,
            u32,
            rd32(base.wrapping_add(4)),
            rd32(base.wrapping_add(0x68))
        );
        let e = rd32(base.wrapping_add(8));
        if e == 0 {
            last = lf_checker_rt::callee_cdecl!(33, u32, rd32(base.wrapping_add(4)), 0u32);
        } else {
            let t = entry(tab2, e);
            let e1 = e.wrapping_sub(1);
            last = lf_checker_rt::callee_cdecl!(33, u32, rd32(base.wrapping_add(4)), t);
            wr32(base.wrapping_add(8), e1);
        }
        lf_checker_rt::callee_thiscall!(34, u32, cookie);
        (last & 0xFFFFFF00) | 1
    }
});
