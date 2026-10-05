// original: 0x00beac00 slot_zero4_ret_this
/// Zero the four dwords of a small slot object and return its address.
///
/// `this` points to at least 16 bytes. Writes 0 to offsets 0, 4, 8 and 12
/// (the original writes offset 4 first, then 0, 8, 12; the order is
/// unobservable) and returns `this` in EAX. No other state is touched.
/// Thiscall, no stack arguments.
export!(thiscall, rw_00beac00(this: u32) -> u32 {
    unsafe {
        for off in [0u32, 4, 8, 12] {
            ((this + off) as *mut u32).write_unaligned(0);
        }
        this
    }
});
