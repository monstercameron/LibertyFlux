// original: 0x009cf800 lazy_singleton_init
/// Lazily allocate and construct the singleton, once.
///
/// Returns 0 when the slot is already set; otherwise allocates 0x1d80
/// bytes, constructs in place, stores the pointer, and returns 0, or
/// 0x8007000E when allocation fails.
export!(cdecl, rw_009cf800() -> u32 {
    const SINGLETON_SLOT: u32 = 0x01295888;
    let slot = global::<u32>(SINGLETON_SLOT);
    if unsafe { slot.read() } != 0 {
        return 0;
    }
    let p: u32 = callee_cdecl!(1, u32, 0x1d80);
    if p == 0 {
        unsafe { slot.write(0) };
        return 0x8007000E;
    }
    callee_thiscall!(2, u32, p);
    unsafe { slot.write(p) };
    0
});
