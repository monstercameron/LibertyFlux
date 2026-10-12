// original: 0x006601f0 rage::snConnectToPeerTask::vf2
/// Poll a connect request: wait while the session is handshaking.
///
/// Marks the task started (`+0xc`). While the session state (`+0x50` of
/// `this+0x60`) is 2 or 3 the task simply returns and is polled again
/// later; in any other state it completes through virtual slot 7 with
/// `(0, 0)`. Returns the completion answer, or the handshake state.
export!(thiscall, rw_006601f0(this: u32) -> u32 {
    unsafe {
        if ((this + 0xc) as *const u32).read() == 0 {
            ((this + 0xc) as *mut u32).write(1);
        }
        let session = ((this + 0x60) as *const u32).read();
        let state = ((session + 0x50) as *const u32).read();
        if state == 2 || state == 3 {
            state
        } else {
            task_complete(this, 0, 0)
        }
    }
});

/// Call the task's virtual slot 7 (`[vtable+0x1c]`) with two words.
///
/// Every task object in this family carries its vtable pointer at offset 0 and
/// reports completion through slot 7, a thiscall taking the task in ECX. Both
/// sides of the check land on the same planted recorder stub.
#[inline(always)]
unsafe fn task_complete(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let vtable = *(this as *const u32);
        let slot = ((vtable + 0x1c) as *const u32).read();
        let target: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        target(this, arg0, arg1)
    }
}
