// original: 0x00670640 rage::fiBinTokenizer::vf18

/// Dispatch through the tokenizer's virtual function table slot at byte
/// offset 0x28. Ignore the first incoming stack word and forward the second with constant 1. Return the method's result word unchanged.
///
/// Original: 0x00670640 (thiscall, 2 stack argument(s), one virtual call).
lf_checker_rt::export!(thiscall, rw_00670640(this: u32, _unused_arg: u32, forwarded: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 0x28;
        let _slot_offset = VTABLE_SLOT;
        lf_checker_rt::callee_thiscall!(1, u32, this, forwarded, 1u32)
    }
});
