// original: 0x00B65C80 veh_guard_forward_2
/// Forward `(a1, a2)` unless a three-link guard chain says to return.
///
/// With `n = [a0+0x6c]`: calls the target (stubbed, thiscall/2) when `a0` is
/// null, when `n` is null, when byte `[n+0x0e]` is clear, or when the flag at
/// `[this+0x109]` is set; otherwise returns without calling. Thiscall, three
/// stack words; entry registers except ECX are ignored. No meaningful return.
export!(thiscall, rw_00b65c80(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const NEXT: u32 = 0x6c;
        const READY: u32 = 0x0e;
        const FLAG: u32 = 0x109;
        let fire = if a0 == 0 {
            true
        } else {
            let n = ((a0 + NEXT) as *const u32).read_unaligned();
            if n == 0 {
                true
            } else if ((n + READY) as *const u8).read() == 0 {
                true
            } else {
                ((this + FLAG) as *const u8).read() != 0
            }
        };
        if fire {
            let _: u32 = callee_thiscall!(1, u32, this, a1, a2);
        }
        0
    }
});
