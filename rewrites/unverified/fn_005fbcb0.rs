// original: 0x005fbcb0 euphoria_init_forwarder

/// Forward two plain arguments to the reference initialiser as thiscall.
///
/// `target` becomes the `this` pointer and `arg` the single stack word of the
/// initialiser call; its 32-bit result is returned unchanged. The caller owns
/// both stack words.
///
/// Original: 0x005FBCB0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_005fbcb0(target: u32, arg: u32) -> u32 {
    unsafe {
        const INIT_CALLEE: u32 = 1;
        lf_checker_rt::callee_thiscall!(INIT_CALLEE, u32, target, arg)
    }

});
