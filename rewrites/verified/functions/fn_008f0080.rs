// original: 0x008f0080 gated_buffer_ptr
/// Pointer to the object's primary buffer, gated by a chain of global and
/// per-object flags; any failing gate selects the fallback buffer instead.
export!(thiscall, rw_008f0080(this: u32) -> u32 {
    const PRIMARY: u32 = 0x3290;
    const FALLBACK: u32 = 0x26A8;
    if unsafe { *global::<u32>(0x1160EBC) } != 0 {
        return this.wrapping_add(FALLBACK);
    }
    if unsafe { *((this.wrapping_add(0x328D)) as *const u8) } == 0 {
        return this.wrapping_add(FALLBACK);
    }
    if unsafe { *global::<u32>(0x11F7060) } == 1 {
        return this.wrapping_add(FALLBACK);
    }
    let have = unsafe { *global::<u32>(0x12088B4) };
    let want = unsafe { *global::<u32>(0xF1C040) };
    if have != want {
        return this.wrapping_add(FALLBACK);
    }
    if unsafe { *global::<u32>(0x1037720) } != 0x12 {
        return this.wrapping_add(PRIMARY);
    }
    this.wrapping_add(FALLBACK)
});
