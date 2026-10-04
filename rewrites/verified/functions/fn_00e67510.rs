// original: 0x00E67510 tail_forward_fixed_this

/// Tail-jump thunk that forwards to a callee with a fixed object pointer.
///
/// Loads the constant `OBJ` into ECX and jumps to the target (which takes no
/// stack arguments and returns its object in EAX). The rewrite performs the
/// same transfer as a call through the intercepted callee and returns its
/// answer. The object address is loader-relocated in the original and derived
/// from the relocated image base here.
///
/// Original: 0x00E67510 (cdecl-shaped entry, no arguments read, tail call, returns its result).
lf_checker_rt::export!(cdecl, rw_00e67510() -> u32 {
    const OBJ: u32 = 0x012F51E0;
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJ))
});
