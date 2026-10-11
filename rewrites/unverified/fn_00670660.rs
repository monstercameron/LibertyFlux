// original: 0x00670660 rage::fiBinTokenizer::vf16

/// Dispatch through the tokenizer's virtual function table slot at byte
/// offset 0x20. Ignore the first incoming stack word and forward the second with constant 1. Return the method's result word unchanged.
///
/// Original: 0x00670660 (thiscall, 2 stack argument(s), one virtual call).
lf_checker_rt::export!(thiscall, rw_00670660(this: u32, _unused_arg: u32, forwarded: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 0x20;
        let _slot_offset = VTABLE_SLOT;
        lf_checker_rt::callee_thiscall!(1, u32, this, forwarded, 1u32)
    }
});
