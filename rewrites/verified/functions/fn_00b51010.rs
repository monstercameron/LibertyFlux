// original: 0x00B51010 id_list_init

/// Initialise a small inline id list: header -1, three zero links, kind 3.
///
/// Layout at `this`: `+0x00` marker (0xFFFFFFFF), `+0x04`, `+0x08`, `+0x0C`
/// zero, `+0x10` kind tag 3. Returns `this`.
///
/// Original: 0x00B51010 (thiscall, `this` in ecx, no stack words).
export!(thiscall, rw_00b51010(this: u32) -> u32 {
    const MARKER: u32 = 0xFFFF_FFFF;
    const KIND: u32 = 3;
    unsafe {
        let p = this as *mut u32;
        p.write_unaligned(MARKER);
        p.add(1).write_unaligned(0);
        p.add(2).write_unaligned(0);
        p.add(3).write_unaligned(0);
        p.add(4).write_unaligned(KIND);
    }
    this
});
