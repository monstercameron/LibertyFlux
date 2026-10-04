// original: 0x00d69600 forward_flags_if_present
// s16f02: forward fixed flags to the slot step when a record is present
// (thiscall/0). Does nothing when the slot is empty. The step's answer is
// passed through in EAX on the taken path; EAX is untouched (caller
// garbage) on the empty path, so the return channel is unchecked.
export!(thiscall, rw_s16f02(this: *const u8) -> u32 {
    unsafe {
        let inner = *((this.add(4)) as *const u32);
        if inner == 0 {
            return 0; // original leaves entry EAX here; callers ignore it
        }
        let step: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        // Push order 1,0,1,1 means arg0 is the last push: (1,1,0,1).
        step(inner, 1, 1, 0, 1)
    }
});
