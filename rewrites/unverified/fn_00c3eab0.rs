// original: 0x00c3eab0 train_free_subobject_274 (proposed)
/// Destroy the sub-object at `+0x274` if one is attached.
///
/// Reads the pointer at `this+0x274`. When non-null, calls destructor
/// id 1 on it (thiscall) and then deallocator id 2 (cdecl, pointer).
/// Always clears the slot to null. Returns nothing meaningful (EAX is
/// the deallocator's answer, or the untouched entry EAX when the slot
/// was already null), so the contract compares no return channel.
///
/// Original: 0x00c3eab0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c3eab0(this: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x274;
        const DTOR: u32 = 1;
        const FREE: u32 = 2;
        let sub = ((this + SLOT) as *const u32).read_unaligned();
        if sub != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(DTOR, u32, sub);
            let _: u32 = lf_checker_rt::callee_cdecl!(FREE, u32, sub);
        }
        ((this + SLOT) as *mut u32).write_unaligned(0);
        0
    }
});
