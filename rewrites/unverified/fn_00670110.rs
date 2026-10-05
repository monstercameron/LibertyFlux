// original: 0x00670110 rage::fiTokenizer::vf29

/// Start a fresh indented line unless one is already open.
///
/// `this` points to the tokenizer. When the writer state at `+0x14` is not
/// 1, the indent callee writes as many tabs as the level at `+0x220`. The
/// state becomes 1 either way. The return register is untouched, so the
/// contract does not compare it.
///
/// Original: 0x00670110 (thiscall, no stack arguments, up to 1 call).
lf_checker_rt::export!(thiscall, rw_00670110(this: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x14;
        const LEVEL: u32 = 0x220;
        const INDENT_CALLEE: u32 = 1;

        if ((this + STATE) as *const u32).read_unaligned() != 1 {
            let level = ((this + LEVEL) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(INDENT_CALLEE, u32, this, level);
        }
        ((this + STATE) as *mut u32).write_unaligned(1);
        0
    }
});
