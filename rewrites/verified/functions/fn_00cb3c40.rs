// original: 0x00CB3C40 CTaskComplexGoToPointAndStandStillTimed::vf18

/// Advance the timed go-to-point task past a finished subtask.
///
/// `this` is the complex task, `ped` the ped. The current subtask's type
/// (virtual slot 3, callee 1) decides: a finished go-to (0x11a) is
/// followed by the stand-still subtask (0x516); a timed wait (0x384)
/// copies bit 5 of the subtask's wait flags (`[this+8]+0xc4`) into bit 4
/// of the task state (`this+0x3c`) and is followed by a fresh go-to
/// (0x11a). Anything else keeps the current subtask (null). Creation goes
/// through the factory (callee 2) as (id, ped).
///
/// Original: 0x00CB3C40 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB3C40(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 8;
        const GET_TYPE_SLOT: u32 = 0x0c;
        const CREATE_SUB: u32 = 2;
        const STATE: u32 = 0x3c;
        const STATE_BIT: u32 = 0x10;
        const WAIT_FLAGS: u32 = 0xc4;
        const GO_TO_DONE: u32 = 0x11a;
        const TIMED_WAIT: u32 = 0x384;
        const STAND_STILL: u32 = 0x516;
        let sub = (this.wrapping_add(SUBTASK) as *const u32).read_unaligned();
        let vtable = (sub as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(GET_TYPE_SLOT) as *const u32).read_unaligned();
        let get_type: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        match get_type(sub) {
            GO_TO_DONE => lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, STAND_STILL, ped),
            TIMED_WAIT => {
                let wait = (sub.wrapping_add(WAIT_FLAGS) as *const u32).read_unaligned();
                let state = (this.wrapping_add(STATE) as *const u32).read_unaligned();
                let moved = (state & !STATE_BIT) | ((wait >> 1) & STATE_BIT);
                (this.wrapping_add(STATE) as *mut u32).write_unaligned(moved);
                lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, GO_TO_DONE, ped)
            }
            _ => 0,
        }
    }
});
