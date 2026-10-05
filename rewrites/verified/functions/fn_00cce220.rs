// original: 0x00cce220 CTaskSimpleDie::vf5
/// Run the simple-die shutdown, returning 1 on success, else 0.
///
/// When the second stack argument is 2, the ped helper runs (thiscall on
/// the first argument, one zero) and the float slot takes -1000.0 instead
/// of -4.0. The third argument, when non-null, is probed up to four times
/// through its virtual slot at `+4`: 0xa, then 0x78, then 9 with bit 1 of
/// `+0x38` set, then 0x7c with a non-zero byte at `+0x14` each reach the
/// sink directly. Otherwise the sink runs only when the kind was 2. The
/// sink calls the follow-up (thiscall on the first argument, 1), clears
/// bit 12 of `[arg0+0x26c]`, forwards the float slot (thiscall on `this`),
/// sets `this+0x34` and returns 1 in AL. Thiscall, three stack arguments.
export!(thiscall, rw_00cce220(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const DEF_F: u32 = 0x00fe8dc8;
        const ALT_F: u32 = 0x00fe8e04;
        const SINK_ARG_OFF: u32 = 0x26c;
        const SINK_BIT: u32 = 0x1000;
        const DONE_OFF: u32 = 0x34;
        let mut fbits = *global::<u32>(DEF_F);
        let mut armed = false;
        if a1 == 2 {
            let _: u32 = callee_thiscall!(1, u32, a0, 0u32);
            armed = true;
            fbits = *global::<u32>(ALT_F);
        }
        let mut sink = armed;
        if a2 != 0 {
            let vt = (a2 as *const u32).read_unaligned();
            let s4 = (vt.wrapping_add(4) as *const u32).read_unaligned();
            let f4: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(s4 as usize);
            if f4(a2) == 0x0a {
                sink = true;
            } else if f4(a2) == 0x78 {
                sink = true;
            } else if f4(a2) == 9 {
                if (a2.wrapping_add(0x38) as *const u8).read() & 2 != 0 {
                    sink = true;
                }
            } else if f4(a2) == 0x7c {
                if (a2.wrapping_add(0x14) as *const u8).read() != 0 {
                    sink = true;
                }
            }
        }
        if !sink {
            return 0;
        }
        let _: u32 = callee_thiscall!(3, u32, a0, 1u32);
        let w = (a0.wrapping_add(SINK_ARG_OFF) as *const u32).read_unaligned();
        (a0.wrapping_add(SINK_ARG_OFF) as *mut u32).write_unaligned(w & !SINK_BIT);
        let _: u32 = callee_thiscall!(4, u32, this, fbits);
        (this.wrapping_add(DONE_OFF) as *mut u8).write(1);
        1
    }
});
