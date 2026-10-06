// original: 0x00D785C0 subobject_mode_equals_two (proposed)

/// Test whether a sub-object's mode field equals 2, keeping the pointer's
/// upper bytes in the result.
///
/// `obj` points to an object whose word at `+0x21c` (`INNER_OFF`) is a
/// pointer to an inner structure. Returns `(inner & ~0xff) | (mode == 2)`
/// where `mode` is the inner word at `+0x12c` (`MODE_OFF`): the original
/// sets only `al` (`sete`), so the upper 24 bits of the loaded inner
/// pointer survive in `eax`. Cdecl, one stack word.
use lf_checker_rt::export;

export!(cdecl, rw_00d785c0(obj: u32) -> u32 {
    unsafe {
        const INNER_OFF: u32 = 0x21c;
        const MODE_OFF: u32 = 0x12c;
        const MODE_MATCH: u32 = 2;
        let inner = ((obj + INNER_OFF) as *const u32).read_unaligned();
        let mode = ((inner + MODE_OFF) as *const u32).read_unaligned();
        (inner & 0xffff_ff00) | u32::from(mode == MODE_MATCH)
    }
});
