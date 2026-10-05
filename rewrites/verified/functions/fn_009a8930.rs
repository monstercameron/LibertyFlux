// original: 0x009a8930 script_idle_check
/// Test whether the script engine is idle (nothing left to run).
///
/// Asks the engine (stubbed, cdecl/1) for its state block. A missing
/// block, or one whose flag byte at `+0xf17` has bit 3 set, means idle.
/// Otherwise the sub-state at `+0x10d0` is polled (stubbed, thiscall/0):
/// idle unless it answers true while the global quiet flag is clear.
/// When the engine reports busy, the nine entity slots at `this+0x2df0`
/// are scanned instead: idle if any slot holds an object whose state
/// word at `+0xa74` is 1 or 2, whose bytes at `+0x211`/`+0x212` are
/// non-zero, or whose word at `+0x28` has bit 21 set. Finally the spare
/// engine (stubbed, cdecl/0) is idle unless its word at `+0xa70` is 1.
/// Thiscall, no stack arguments, byte result.
export!(thiscall, rw_009A8930(this: u32) -> u32 {
    unsafe {
        const QUIET_FLAG: u32 = 0x1284a51;
        const SLOTS: u32 = 0x2df0;
        const COUNT: u32 = 9;
        let st: u32 = callee_cdecl!(1, u32, 0);
        if st == 0 {
            return scan_slots(this);
        }
        if ((st + 0xf17) as *const u8).read() & 8 != 0 {
            return 1;
        }
        let sub: u32 = callee_thiscall!(2, u32, st.wrapping_add(0x10d0));
        if sub as u8 != 0 && global::<u8>(QUIET_FLAG).read() == 0 {
            return 1;
        }
        scan_slots(this)
    }
});

/// Scan the entity slots and the spare engine for idleness (see above).
unsafe fn scan_slots(this: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x2df0;
        const COUNT: u32 = 9;
        let mut k = 0u32;
        while k < COUNT {
            let p = ((this + SLOTS + k * 4) as *const u32).read_unaligned();
            if p != 0 {
                let s = ((p + 0xa74) as *const u32).read_unaligned();
                if s == 1 || s == 2 {
                    return 1;
                }
                if ((p + 0x212) as *const u8).read() != 0 {
                    return 1;
                }
                if ((p + 0x211) as *const u8).read() != 0 {
                    return 1;
                }
                if ((p + 0x28) as *const u32).read_unaligned() >> 21 & 1 != 0 {
                    return 1;
                }
            }
            k += 1;
        }
        let spare: u32 = callee_cdecl!(3, u32,);
        if spare != 0 && ((spare + 0xa70) as *const u32).read_unaligned() == 1 {
            return 1;
        }
        0
    }
}
