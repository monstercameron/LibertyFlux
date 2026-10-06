// original: 0x00dcfeb0 ui_table_ctor (proposed)

/// Constructor for a ui table: runs the base constructor, installs the
/// vtable and zeroes the four counter words.
///
/// `this` points to the new object. The two incoming words go to the base
/// constructor callee (stdcall, two words; its answer is ignored). Then the
/// vtable is stored at `+0x0` and the words at `+0x1e0`, `+0x1e8`, `+0x1ec`
/// and `+0x1e4` are zeroed, in that order. Returns `this`.
///
/// Original: 0x00DCFEB0 (thiscall, two stack words, returns the object).
lf_checker_rt::export!(thiscall, rw_00dcfeb0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        /// Base constructor callee id.
        const BASE_CTOR: u32 = 1;
        /// Vtable installed by this constructor.
        const VTABLE: u32 = 0xefa8ac;
        lf_checker_rt::callee_stdcall!(BASE_CTOR, u32, a0, a1);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + 0x1e0) as *mut u32).write_unaligned(0);
        ((this + 0x1e8) as *mut u32).write_unaligned(0);
        ((this + 0x1ec) as *mut u32).write_unaligned(0);
        ((this + 0x1e4) as *mut u32).write_unaligned(0);
        this
    }
});
