// original: 0x009C04A0 ped_event_delay_for_type
/// Returns the default delay 1000 for a null event or an event whose type
/// nibble at offset 0x28 is not 3. A matching event with bit 2 set at byte
/// offset 0x26c and a nonzero dword at 0xb30 returns 500. Otherwise it calls
/// the one-argument cdecl predicate: a nonzero low-byte answer returns 1000,
/// and zero returns 0.
lf_checker_rt::export!(cdecl, rw_009c04a0(event: u32) -> u32 {
    const DEFAULT_DELAY: u32 = 1000;
    const SPECIAL_DELAY: u32 = 500;
    const EXPECTED_KIND: u32 = 3;
    const KIND_SHIFT: u32 = 6;
    const KIND_MASK: u32 = 0x0F;
    const SPECIAL_FLAG: u8 = 0x04;

    if event == 0 {
        return DEFAULT_DELAY;
    }
    let kind = (unsafe { ((event + 0x28) as *const u32).read() } >> KIND_SHIFT) & KIND_MASK;
    if kind != EXPECTED_KIND {
        return DEFAULT_DELAY;
    }
    let flags = unsafe { ((event + 0x26C) as *const u8).read() };
    let token = unsafe { ((event + 0xB30) as *const u32).read() };
    if flags & SPECIAL_FLAG != 0 && token != 0 {
        return SPECIAL_DELAY;
    }
    if lf_checker_rt::callee_cdecl!(1, u8, event) != 0 {
        DEFAULT_DELAY
    } else {
        0
    }
});
