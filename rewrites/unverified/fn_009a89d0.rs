// original: 0x009a89d0 script_ready_check
/// Test whether the script engine is ready to run.
///
/// Asks the engine (stubbed, cdecl/1) for its state block; a missing
/// block defers to the slot scan below. Otherwise the fast probe
/// (stubbed, thiscall/2 with `(1, 0x7d0)`) decides: ready on a true
/// answer. On a false answer the block's word at `+0x1300` vetoes
/// (non-zero means not ready), else the slow probe (stubbed,
/// thiscall/0) decides. Thiscall, no stack arguments, byte result.
export!(thiscall, rw_009A89D0(this: u32) -> u32 {
    unsafe {
        let st: u32 = callee_cdecl!(1, u32, 0);
        if st == 0 {
            return scan_slots_ready(this);
        }
        let fast: u32 = callee_thiscall!(2, u32, st.wrapping_add(0x10d0), 1, 0x7d0);
        if fast as u8 != 0 {
            return 1;
        }
        if ((st + 0x1300) as *const u32).read_unaligned() != 0 {
            return 0;
        }
        let slow: u32 = callee_thiscall!(3, u32, st);
        if slow as u8 == 0 {
            return 0;
        }
        1
    }
});

/// Scan the entity slots for readiness (second half of the check).
///
/// Walks the nine object pointers at `this+0x2df0`. For each non-null
/// entry whose byte at `+0x29c` has bit 2 set: ready if the flag byte
/// at `this+0x0a` is clear; otherwise the arbiter (stubbed, cdecl/0)
/// decides, and its zero answer means ready. No entry ready means not.
unsafe fn scan_slots_ready(this: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x2df0;
        const COUNT: u32 = 9;
        let mut k = 0u32;
        while k < COUNT {
            let p = ((this + SLOTS + k * 4) as *const u32).read_unaligned();
            if p != 0 && ((p + 0x29c) as *const u8).read() & 4 != 0 {
                if ((this + 0x0a) as *const u8).read() == 0 {
                    return 1;
                }
                let arb: u32 = callee_cdecl!(4, u32,);
                if arb as u8 == 0 {
                    return 1;
                }
            }
            k += 1;
        }
        0
    }
}
