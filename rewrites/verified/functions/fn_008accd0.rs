// original: 0x008accd0 audEffectChain_store_arg
/// Calls `inner(this)`, then `outer(inner_result, this)`, stores `arg` at
/// `this+0x30` and returns `arg`.
export!(thiscall, rw_008accd0(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        let inner = callee_thiscall!(1, u32, this as u32);
        callee_thiscall!(2, u32, inner, this as u32);
        *(this.add(0x30) as *mut u32) = arg;
        arg
    }
});

