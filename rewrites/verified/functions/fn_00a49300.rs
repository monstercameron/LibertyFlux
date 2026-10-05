// original: 0x00a49300 NativeImpl_GET_CAR_LIVERY
/// Return the livery id of a vehicle, or -1 when its info chain is missing.
///
/// `this+0x34` points at an outer record whose second word points at an
/// inner record; the result is the dword at `+0xD8` of the inner record
/// (thiscall, no stack arguments). A null inner pointer yields -1.
export!(thiscall, rw_00a49300(this: u32) -> u32 {
    unsafe {
        const OUTER_OFF: u32 = 0x34;
        const INNER_OFF: u32 = 0xd8;
        let outer = (this.wrapping_add(OUTER_OFF) as *const u32).read_unaligned();
        let inner = (outer.wrapping_add(4) as *const u32).read_unaligned();
        if inner == 0 {
            return 0xFFFFFFFF;
        }
        (inner.wrapping_add(INNER_OFF) as *const u32).read_unaligned()
    }
});
