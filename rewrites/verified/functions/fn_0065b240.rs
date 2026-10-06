// original: 0x0065B240 pass_count_or_alloc (proposed)

/// Set the pass count, allocating the entry array on first use.
///
/// If the 16-bit tag at `+6` is zero this is first use: the tag is set to
/// the low 16 bits of `n`, then the full 32-bit `n` is tested (a value like
/// 0x10000 stores a zero tag but still allocates): zero writes a zero word
/// at `+4` and a null pointer at `+0` and returns 0, otherwise the array
/// allocator runs (patched callee, stdcall: `n`), the low word goes to `+4`
/// and the block to `+0`, and the block is returned. If the tag is already
/// nonzero only the low word of `n` is stored at `+4`; the low 16 bits of
/// the return value are `n` and the upper half is the entry leftover, so
/// only `ax` is compared (cdecl, two arguments).
lf_checker_rt::export!(cdecl, rw_0065b240(obj: u32, n: u32) -> u32 {
    unsafe {
        const CALLEE_ALLOC: u32 = 1;
        if ((obj + 6) as *const u16).read_unaligned() == 0 {
            ((obj + 6) as *mut u16).write_unaligned(n as u16);
            if n == 0 {
                ((obj + 4) as *mut u16).write_unaligned(0);
                (obj as *mut u32).write_unaligned(0);
                0
            } else {
                let mem: u32 = lf_checker_rt::callee_stdcall!(CALLEE_ALLOC, u32, n);
                ((obj + 4) as *mut u16).write_unaligned(n as u16);
                (obj as *mut u32).write_unaligned(mem);
                mem
            }
        } else {
            ((obj + 4) as *mut u16).write_unaligned(n as u16);
            n & 0xFFFF
        }
    }
});
