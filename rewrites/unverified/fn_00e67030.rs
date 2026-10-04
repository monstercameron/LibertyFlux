// original: 0x00e67030 sweep_thiscall_205 (proposed)
/// Call a thiscall/0 callee 205 times over strided global addresses.
///
/// Calls `callee()` with ecx sweeping `0x012BD1E0 + i*0x280` for `i` in
/// 0..205 (the original counts edi down from 0xCC and keeps looping while
/// the decrement is non-negative: 204+1 runs). Callee answers are ignored.
/// No arguments (cdecl/0). Calling convention: cdecl.
lf_checker_rt::export!(cdecl, rw_00e67030() -> u32 {
    unsafe {
        const BASE: u32 = 0x012BD1E0;
        const STRIDE: u32 = 0x280;
        const COUNT: u32 = 205;
        let mut i = 0u32;
        while i < COUNT {
            lf_checker_rt::callee_thiscall!(
                1,
                u32,
                lf_checker_rt::relocated(BASE.wrapping_add(i.wrapping_mul(STRIDE)))
            );
            i += 1;
        }
        0
    }
});
