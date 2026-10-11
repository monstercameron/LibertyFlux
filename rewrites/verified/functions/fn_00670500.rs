// original: 0x00670500 rage::fiTokenizer::vf38

/// Dispatch through the tokenizer's virtual function table slot at byte
/// offset 0x94. Forward the caller's value and trailing constant 0. Return the method's result word unchanged.
///
/// Original: 0x00670500 (thiscall, 1 stack argument(s), one virtual call).
lf_checker_rt::export!(thiscall, rw_00670500(this: u32, forwarded: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 0x94;
        let _slot_offset = VTABLE_SLOT;
        lf_checker_rt::callee_thiscall!(1, u32, this, forwarded, 0u32)
    }
});
