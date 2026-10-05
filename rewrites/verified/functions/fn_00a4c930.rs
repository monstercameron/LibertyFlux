// original: 0x00A4C930 vehicle_attach_links (proposed)

/// Links the argument object into two slots of this object, gated by link
/// flags.
///
/// When the argument's linked object at `[arg + LINK]` (0x6C) is absent or
/// its byte at `+0x0E` is zero, stores the argument into `this + SLOT_A` (4)
/// and registers through the first callee (`arg` in `ecx`, slot address on
/// the stack). Then examines the second link: `aux = [this + SLOT_B]` (8),
/// `gate = [aux + LINK]`: a present flagged gate returns 0; otherwise, when
/// the gate value is non-zero, the second callee (`gate` in `ecx`, no stack
/// words) is asked and a non-zero low byte returns 0. Falling through stores
/// the argument into `this + SLOT_C` (0), registers through the first callee
/// again, and returns 1.
///
/// Original: 0x00A4C930 (thiscall, one pointer stack word), two callees.
lf_checker_rt::export!(thiscall, rw_00A4C930(this: u32, arg: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x6C;
        const LINK_FLAG: u32 = 0x0E;
        const SLOT_A: u32 = 0x04;
        const SLOT_B: u32 = 0x08;
        const SLOT_C: u32 = 0x00;
        const REGISTER_CALLEE: u32 = 1;
        const GATE_CALLEE: u32 = 2;
        let link1 = ((arg + LINK) as *const u32).read_unaligned();
        if link1 == 0 || ((link1 + LINK_FLAG) as *const u8).read() == 0 {
            let slot = (this + SLOT_A) as *mut u32;
            slot.write_unaligned(arg);
            lf_checker_rt::callee_thiscall!(
                REGISTER_CALLEE,
                u32,
                arg,
                slot as u32
            );
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
        let slot = (this + SLOT_C) as *mut u32;
        slot.write_unaligned(arg);
        lf_checker_rt::callee_thiscall!(REGISTER_CALLEE, u32, arg, slot as u32);
        1
    }
});
