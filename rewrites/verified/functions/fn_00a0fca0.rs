// original: 0x00a0fca0 readiness_probe (proposed)
/// Report whether the object's linked state is ready.
///
/// Returns 0 (low byte) when `obj` or its link at `+0x34` is null. When the
/// secondary link at `+0x30` is non-null, asks it through the probe callee
/// and returns 1 when the callee's low byte is non-zero. Otherwise decodes
/// bits 6-9 of the word at `link + 0x28`: value 3 returns whether the byte
/// at `link + 0xa60` is 2, value 2 returns 1, anything else 0. Cdecl.
export!(cdecl, rw_00a0fca0(obj: u32) -> u32 {
    unsafe {
        const PROBE: u32 = 1;
        const LINK_OFF: u32 = 0x34;
        const SECONDARY_OFF: u32 = 0x30;
        const KIND_OFF: u32 = 0x28;
        const STATE_OFF: u32 = 0xa60;
        if obj == 0 {
            return 0;
        }
        let link = ((obj + LINK_OFF) as *const u32).read_unaligned();
        if link == 0 {
            return 0;
        }
        let secondary = ((obj + SECONDARY_OFF) as *const u32).read_unaligned();
        if secondary != 0 && (callee_thiscall!(PROBE, u32, secondary) & 0xff) != 0 {
            return 1;
        }
        match (((link + KIND_OFF) as *const u32).read_unaligned() >> 6) & 0xf {
            3 => (((link + STATE_OFF) as *const u8).read() == 2) as u32,
            2 => 1,
            _ => 0,
        }
    }
});
