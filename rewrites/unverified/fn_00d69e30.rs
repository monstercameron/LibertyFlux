// original: 0x00d69e30 CReplayRolloverMessage::vf1
/// Refresh the rollover message when its source is live (original 0x00D69E30,
/// thiscall/0).
///
/// Pokes the member at `this+4` through callee 1 with 0, then passes three
/// gates: callee 2's low byte must be nonzero, callee 3's low byte (on the
/// member) must be zero, and the flag byte at `member+0x1b` must be nonzero.
/// Past them, it notifies the sink object at file VA 0x0103E498 (callee 4),
/// picks kind 0x1a when the mode byte (byte 2 of the dword at file VA
/// 0x01037758) is zero and the member state dword at `+0x38c` equals 9, else
/// 0x14, reformats through callee 5, marks `this+0x1c` seen and re-pokes the
/// member through callee 1 with 1. All comparisons are equality (or
/// test-byte) checks. Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d69e30(this_ptr: u32) -> u32 {
    unsafe {
        const MEMBER_OFF: u32 = 4;
        const SEEN_OFF: u32 = 0x1c;
        const HAS_MSG_OFF: u32 = 0x1b;
        const STATE_OFF: u32 = 0x38c;
        const SINK_OBJ: u32 = 0x0103E498;
        const MODE_DWORD: u32 = 0x01037758;
        const MODE_SHIFT: u32 = 16;
        const KIND_FULL: u32 = 0x1a;
        const KIND_SHORT: u32 = 0x14;
        let member = ((this_ptr + MEMBER_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(1, u32, member, 0);
        let alive: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        if (alive & 0xFF) == 0 {
            return 0;
        }
        let ready: u32 = lf_checker_rt::callee_thiscall!(3, u32, member);
        if (ready & 0xFF) != 0 {
            return 0;
        }
        if ((member + HAS_MSG_OFF) as *const u8).read() == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(4, u32,
            lf_checker_rt::relocated(SINK_OBJ));
        let mode = (lf_checker_rt::relocated(MODE_DWORD) as *const u32)
            .read_unaligned();
        let state =
            ((member + STATE_OFF) as *const u32).read_unaligned();
        let kind = if ((mode >> MODE_SHIFT) & 0xFF) == 0 && state == 9 {
            KIND_FULL
        } else {
            KIND_SHORT
        };
        lf_checker_rt::callee_thiscall!(5, u32, this_ptr, kind);
        ((this_ptr + SEEN_OFF) as *mut u8).write(1);
        lf_checker_rt::callee_thiscall!(1, u32, member, 1);
        0
    }
});
