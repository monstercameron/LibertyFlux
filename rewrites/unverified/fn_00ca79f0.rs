// original: 0x00CA79F0 CEventHandler::vf36

/// Poll the event's owner slot until it settles or matches this handler.
///
/// Calls slot `POLL_SLOT` (0x34) of the first stack argument's virtual table
/// with that argument as `this`. When the first result is null the handler
/// is done; when the second result equals the handler's owner pointer at
/// `this+0x04` it is also done; otherwise it calls once more and is done.
/// Makes one to three identical calls and no stores. The other two stack
/// arguments are not read. No return value.
///
/// Original: 0x00CA79F0 (thiscall, three stack words, only the first read).
lf_checker_rt::export!(thiscall, rw_00ca79f0(this: u32, obj: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const OWNER: u32 = 0x04;
        const POLL_SLOT: u32 = 0x34;
        unsafe fn poll(obj: u32) -> u32 {
            unsafe {
                let vtable = (obj as *const u32).read_unaligned();
                let slot = (vtable.wrapping_add(POLL_SLOT) as *const u32).read_unaligned();
                let get: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                get(obj)
            }
        }
        if poll(obj) == 0 {
            return 0;
        }
        let owner = (this.wrapping_add(OWNER) as *const u32).read_unaligned();
        if poll(obj) == owner {
            return 0;
        }
        poll(obj);
        0
    }
});
