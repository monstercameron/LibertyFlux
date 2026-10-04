// original: 0x00dfc0bc srand
/// MSVC `srand`: store `seed` into the per-thread seed slot.
///
/// Returns the context pointer the helper handed back (whatever the
/// original left in EAX on this path).
export!(cdecl, rw_00dfc0bc(seed: u32) -> u32 {
    unsafe {
        let ctx = callee_cdecl!(1, u32,);
        *((ctx.wrapping_add(0x14)) as *mut u32) = seed;
        ctx
    }
});
