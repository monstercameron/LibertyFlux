// original: 0x00a0fe70 id_guard_or_fallback (proposed)
/// Accept a positive id with a usable sub-id, else ask the fallback.
///
/// Returns 1 (low byte) when `id` is strictly positive and `sub` is neither
/// 0x3f nor -1. Otherwise calls the fallback with no arguments and returns
/// whether its low byte differs from 1. Cdecl, two stack arguments.
export!(cdecl, rw_00a0fe70(id: u32, sub: u32) -> u32 {
    unsafe {
        const SKIP_SUB: u32 = 0x3f;
        const FALLBACK: u32 = 1;
        if (id as i32) > 0 && sub != SKIP_SUB && sub != 0xffff_ffff {
            1
        } else {
            ((callee_cdecl!(FALLBACK, u32,) & 0xff) != 1) as u32
        }
    }
});
