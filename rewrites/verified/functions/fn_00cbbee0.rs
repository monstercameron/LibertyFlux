// original: 0x00CBBEE0 CTaskComplexMoveCrowdAroundLocation::vf9

/// Copy four raw words from the second stack argument into this object's 16-byte position record beginning at offset 0x0c, preserving floating-point payload bits, and return the last copied word. The thiscall method pops three stack words; its first and third arguments are unused.
lf_checker_rt::export!(thiscall, rw_00cbbee0(this: u32, _unused0: u32, source: u32, _unused2: u32) -> u32 {
unsafe {
        for offset in [0u32, 4, 8, 12] {
            let word = ((source + offset) as *const u32).read_unaligned();
            ((this + 0x0c + offset) as *mut u32).write_unaligned(word);
        }
        ((source + 12) as *const u32).read_unaligned()
    }
});
