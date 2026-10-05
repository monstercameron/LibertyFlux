// original: 0x00A4E4F0 vehicle_unlink_pair (proposed)

/// Clears two slots of this object when the argument matches, gated by a
/// link flag.
///
/// When the word at `this + SLOT_A` (4) equals the argument it is released
/// through the first callee (`this` = old value, slot address on the stack;
/// skipped for a zero old value) and zeroed. Then examines the gate link:
/// `aux = [this + SLOT_B]` (8), `gate = [aux + LINK]` (0x6C): a present
/// flagged gate returns 0; otherwise, when the gate value is non-zero, the
/// second callee (`gate` in `ecx`) is asked and a non-zero low byte returns
/// 0. Falling through releases the word at `this + SLOT_C` (0) the same way
/// when non-zero, zeroes it, and returns 1.
///
/// Original: 0x00A4E4F0 (thiscall, one stack word), two callees.
lf_checker_rt::export!(thiscall, rw_00A4E4F0(this: u32, arg: u32) -> u32 {
    unsafe {
        const SLOT_A: u32 = 0x04;
        const SLOT_B: u32 = 0x08;
        const SLOT_C: u32 = 0x00;
        const LINK: u32 = 0x6C;
        const LINK_FLAG: u32 = 0x0E;
        const RELEASE_CALLEE: u32 = 1;
        const GATE_CALLEE: u32 = 2;
        let sa = (this + SLOT_A) as *mut u32;
        if sa.read_unaligned() == arg {
            let old = sa.read_unaligned();
            if old != 0 {
                lf_checker_rt::callee_thiscall!(
                    RELEASE_CALLEE,
                    u32,
                    old,
                    sa as u32
                );
            }
            sa.write_unaligned(0);
        }
        let aux = ((this + SLOT_B) as *const u32).read_unaligned();
        let gate = ((aux + LINK) as *const u32).read_unaligned();
        if gate != 0 && ((gate + LINK_FLAG) as *const u8).read() != 0 {
            return 0;
        }
        if gate != 0 {
            let ans: u32 =
                lf_checker_rt::callee_thiscall!(GATE_CALLEE, u32, gate);
            if (ans & 0xFF) != 0 {
                return 0;
            }
        }
        let sc = (this + SLOT_C) as *mut u32;
        let old = sc.read_unaligned();
        if old != 0 {
            lf_checker_rt::callee_thiscall!(
                RELEASE_CALLEE,
                u32,
                old,
                sc as u32
            );
        }
        sc.write_unaligned(0);
        1
    }
});
