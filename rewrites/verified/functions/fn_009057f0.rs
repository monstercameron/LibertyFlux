// original: 0x009057F0 input_slots_refresh (proposed)

/// Refresh the input slots: refill the cached variant words, run the
/// per-slot worker over every slot, and forward the selected slot's
/// position to the cursor placer.
///
/// Fills three cached words through the fill helper (cdecl, pointer and
/// length 4; a fourth fill lands in unused scratch, and a dead zero store
/// beside it is replicated for fidelity). When the refresh flag is clear,
/// the constructor helper runs and the three words land in `VAR_A/B/C`.
/// Then the worker callee (cdecl, one argument) runs `0x96` times with the
/// flag-clear value. When the flag is set the function returns here.
///
/// Otherwise the helper index from `SEL` is validated by the check helper
/// (a zero answer returns) and resolved by the lookup helper (a negative
/// answer returns); the slot record is `TABLE[answer]`. Three position
/// words are taken from the record: offsets `0x30/0x34/0x38` when its flag
/// byte (`+0x08`) is set, else `0x20/0x24` plus zero. The probe helper
/// (cdecl, pointer, `0x10`, `0xff`) sees the words, and its answer's first
/// word plus the words go to the placer (thiscall on the fixed cursor
/// object, six stack arguments). Returns the last callee answer with its
/// low byte set to 1. Original: 0x009057F0 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_009057F0() -> u32 {
    unsafe {
        const FLAG: u32 = 0x0116D27D;
        const SEL: u32 = 0x01160C4C;
        const TABLE: u32 = 0x0118F6F8;
        const VAR_A: u32 = 0x0103449C;
        const VAR_B: u32 = 0x010344A0;
        const VAR_C: u32 = 0x01034494;
        const CURSOR: u32 = 0x011A32C0;
        const SLOTS: u32 = 0x96;

        let mut frame = [0u32; 12];
        let base = core::hint::black_box(frame.as_mut_ptr() as u32);
        lf_checker_rt::callee_cdecl!(11, u32, base.wrapping_add(0x10), 4u32);
        (base.wrapping_add(6) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_cdecl!(12, u32, base.wrapping_add(0x14), 4u32);
        lf_checker_rt::callee_cdecl!(13, u32, base.wrapping_add(0x18), 4u32);
        lf_checker_rt::callee_cdecl!(14, u32, base.wrapping_add(0x0c), 4u32);

        let flag = (lf_checker_rt::relocated(FLAG) as *const u8).read();
        let mut last = 0u32;
        if flag == 0 {
            last = lf_checker_rt::callee_cdecl!(2, u32,);
            ((lf_checker_rt::relocated(VAR_A)) as *mut u32)
                .write_unaligned((base.wrapping_add(0x10) as *const u32).read_unaligned());
            ((lf_checker_rt::relocated(VAR_B)) as *mut u32)
                .write_unaligned((base.wrapping_add(0x14) as *const u32).read_unaligned());
            ((lf_checker_rt::relocated(VAR_C)) as *mut u32)
                .write_unaligned((base.wrapping_add(0x18) as *const u32).read_unaligned());
        }
        let arg = (flag == 0) as u32;
        for _ in 0..SLOTS {
            last = lf_checker_rt::callee_cdecl!(3, u32, arg);
        }
        if flag == 0 {
            let esi = (lf_checker_rt::relocated(SEL) as *const u32).read_unaligned();
            last = lf_checker_rt::callee_cdecl!(4, u32, esi);
            if (last as u8) != 0 {
                last = lf_checker_rt::callee_cdecl!(5, u32, esi);
                if (last as i32) >= 0 {
                    let tab = lf_checker_rt::relocated(TABLE);
                    let rec = (tab.wrapping_add(last.wrapping_mul(4)) as *const u32)
                        .read_unaligned();
                    let active = (rec.wrapping_add(8) as *const u8).read() != 0;
                    let (f0, f1, f2) = if active {
                        (
                            (rec.wrapping_add(0x30) as *const u32).read_unaligned(),
                            (rec.wrapping_add(0x34) as *const u32).read_unaligned(),
                            (rec.wrapping_add(0x38) as *const u32).read_unaligned(),
                        )
                    } else {
                        (
                            (rec.wrapping_add(0x20) as *const u32).read_unaligned(),
                            (rec.wrapping_add(0x24) as *const u32).read_unaligned(),
                            0u32,
                        )
                    };
                    (base.wrapping_add(0x20) as *mut u32).write_unaligned(f0);
                    (base.wrapping_add(0x24) as *mut u32).write_unaligned(f1);
                    (base.wrapping_add(0x28) as *mut u32).write_unaligned(f2);
                    last = lf_checker_rt::callee_cdecl!(
                        6,
                        u32,
                        base.wrapping_add(0x1c),
                        0x10u32,
                        0xffu32
                    );
                    let w = (last as *const u32).read_unaligned();
                    last = lf_checker_rt::callee_thiscall!(
                        7,
                        u32,
                        lf_checker_rt::relocated(CURSOR),
                        base.wrapping_add(0x20),
                        0u32,
                        0u32,
                        1u32,
                        0u32,
                        w
                    );
                }
            }
        }
        (last & 0xFFFFFF00) | 1
    }
});
