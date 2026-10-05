// original: 0x00A4D060 NativeImpl_SET_CAR_LIVERY

/// Stores the argument at slot 0xD8 of the object behind a two-step chain.
///
/// `inner = [this + LINK]` (always dereferenced), `slot = [inner + SLOT_PTR]`;
/// when `slot` is non-null the argument is written to `slot + VALUE` (0xD8).
/// A null `slot` skips the store. Returns `inner` in `eax` on every path.
///
/// Original: 0x00A4D060 (thiscall, one stack word), leaf, no globals.
lf_checker_rt::export!(thiscall, rw_00A4D060(this: u32, val: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x34;
        const SLOT_PTR: u32 = 0x04;
        const VALUE: u32 = 0xD8;
        let inner = ((this + LINK) as *const u32).read_unaligned();
        let slot = ((inner + SLOT_PTR) as *const u32).read_unaligned();
        if slot != 0 {
            ((slot + VALUE) as *mut u32).write_unaligned(val);
        }
        inner
    }
});
