// original: 0x00b55800 drain_work_queue_b
/// Drain pending work items until the +0x04 flag clears (second queue).
///
/// Same shape as fn_00b557d0 with a different fetch helper: while the flag
/// at +0x04 is set, fetches the next item with helper B and releases it
/// through the first slot of its own function table with argument 1.
/// Returns the last helper answer. Same checker note: single passes are
/// verified, the re-loop edge is by review.
export!(thiscall, rw_00b55800(this: u32) -> u32 {
    unsafe {
        if ((this + 4) as *const u32).read() == 0 {
            return 0;
        }
        let mut answer: u32 = 0;
        loop {
            let obj: u32 = callee_thiscall!(1, u32, this);
            if obj != 0 {
                let vtable = (obj as *const u32).read();
                let slot = (vtable as *const u32).read();
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                answer = release(obj, 1);
            } else {
                answer = 0;
            }
            if ((this + 4) as *const u32).read() == 0 {
                break;
            }
        }
        answer
    }
});
