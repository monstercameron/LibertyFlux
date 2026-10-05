// original: 0x00B64200 veh_teardown_all
/// Tear down four slots, reset, then visit the 11-entry array twice over.
///
/// For each of `[this+0x14]` and `[this+0x18]`: skips nulls; when the marker
/// byte at +0x22a is 2 keeps the reference (stubbed thiscall/3 with
/// `(slot,1,1)` for the first, thiscall/1 with `(1)` for the second), else
/// detaches (stubbed stdcall/1), drops (stubbed cdecl/1) and zeroes the slot.
/// Runs the reset helper (stubbed, thiscall/0). Drops `[this+0x20]` (stubbed
/// thiscall/0 + cdecl/1) when non-null and zeroes it; detaches and zeroes
/// `[this+0xe8]` likewise. Visits `this+0xa8` then the 11 slots below it
/// (stubbed, thiscall/0 each). Thiscall, no stack words.
export!(thiscall, rw_00b64200(this: u32) -> u32 {
    unsafe {
        const S0: u32 = 0x14;
        const S1: u32 = 0x18;
        const S2: u32 = 0x20;
        const S3: u32 = 0xe8;
        const READY: u32 = 0x22a;
        const CODE: u8 = 2;
        const ARR: u32 = 0xa8;
        for (slot, keep) in [(S0, 1u32), (S1, 2u32)] {
            let addr = this + slot;
            let o = (addr as *const u32).read_unaligned();
            if o == 0 {
                continue;
            }
            if ((o + READY) as *const u8).read() == CODE {
                if keep == 1 {
                    let _: u32 = callee_thiscall!(1, u32, this, addr, 1, 1);
                } else {
                    let _: u32 = callee_thiscall!(4, u32, this, 1);
                }
            } else {
                let _: u32 = callee_stdcall!(2, u32, addr);
                let cur = (addr as *const u32).read_unaligned();
                let _: u32 = callee_cdecl!(3, u32, cur);
                (addr as *mut u32).write_unaligned(0);
            }
        }
        let _: u32 = callee_thiscall!(5, u32, this);
        let o2 = ((this + S2) as *const u32).read_unaligned();
        if o2 != 0 {
            let _: u32 = callee_thiscall!(6, u32, o2);
            let _: u32 = callee_cdecl!(7, u32, o2);
            ((this + S2) as *mut u32).write_unaligned(0);
        }
        let a3 = this + S3;
        if (a3 as *const u32).read_unaligned() != 0 {
            let _: u32 = callee_stdcall!(2, u32, a3);
            (a3 as *mut u32).write_unaligned(0);
        }
        let mut p = this + ARR;
        let _: u32 = callee_thiscall!(8, u32, p);
        for _ in 0..11u32 {
            p = p.wrapping_sub(0xc);
            let _: u32 = callee_thiscall!(8, u32, p);
        }
        0
    }
});
