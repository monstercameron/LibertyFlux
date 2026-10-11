// original: 0x00670500 rage::fiTokenizer::vf38

/// Dispatch through the tokenizer's virtual function table slot at byte
/// offset 0x94. The method receives the same this pointer and forwarded
/// stack value as the original, followed by constant 0; its return
/// word is returned unchanged.
///
/// Original: 0x00670500 (thiscall, 1 stack argument(s), one virtual call).
lf_checker_rt::export!(thiscall, rw_00670500(this: u32, arg0: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 0x94;
        let _slot_offset = VTABLE_SLOT;
        lf_checker_rt::callee_thiscall!(1, u32, this, arg0, 0u32)
    }
});
