// original: 0x009B7F60 CCamScriptInstruction_state_update
/// Calls two thiscall helpers. It stores the first helper's result in the instruction at offset 0x52C and through the output pointer, then clears offset 0x78 in the object returned by the second helper. The second result is returned in EAX.
lf_checker_rt::export!(thiscall, rw_009b7f60(this: u32, output: u32) -> u32 {
    const LAST_RESULT: u32 = 0x52C;
    const OBJECT_FIELD: u32 = 0x78;
    let state = lf_checker_rt::callee_thiscall!(1, u32, this);
    unsafe {
        (this.wrapping_add(LAST_RESULT) as *mut u32).write_unaligned(state);
        (output as *mut u32).write_unaligned(state);
    }
    let object = lf_checker_rt::callee_thiscall!(2, u32, this);
    unsafe { (object.wrapping_add(OBJECT_FIELD) as *mut u32).write_unaligned(0); }
    object
});
