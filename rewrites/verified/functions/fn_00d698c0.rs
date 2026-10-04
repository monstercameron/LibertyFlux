// original: 0x00d698c0 activate_if_present
// s16f11: run the activation sequence when a record is present (thiscall/0).
//
// Null-checks this record's head pointer, then runs the two activation
// calls the original reaches through a tail jump: a setup call followed by
// the mode-2 step. Empty head does nothing. EAX is caller garbage on the
// empty path, so the return channel is unchecked.
export!(thiscall, rw_s16f11(this: *const u8) -> u32 {
    unsafe {
        let head = *(this as *const u32);
        if head == 0 {
            return 0; // original leaves entry EAX here; callers ignore it
        }
        callee_cdecl!(1, u32, 0);
        let step: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        step(head, 2)
    }
});
