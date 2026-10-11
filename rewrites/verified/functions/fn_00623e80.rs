// original: 0x00623E80 network_payload_target_token_match

/// The receiver status at 0x50 is signed and must be 2 or 3. The payload
/// index at 0x18 is signed; this variant checks it before the helper, while its
/// sibling checks it after helper acceptance. The helper receives the payload
/// words at 0x1c/0x20 and the receiver, with the target as `this`. Its low return
/// byte controls whether the receiver token words at 0x540/0x544 are compared
/// with the first two target words. The stack arguments are payload and target.
lf_checker_rt::export!(thiscall, rw_00623e80(this: u32, payload: u32, target: u32) -> u32 {
    unsafe {
        const STATUS: u32 = 0x50;
        const PAYLOAD_INDEX: u32 = 0x18;
        const PAYLOAD_KEY_LO: u32 = 0x1c;
        const PAYLOAD_KEY_HI: u32 = 0x20;
        const ACTIVE_TOKEN_LO: u32 = 0x540;
        const ACTIVE_TOKEN_HI: u32 = 0x544;
        let read = |base: u32, offset: u32| (base.wrapping_add(offset) as *const u32).read_unaligned();
        let status = read(this, STATUS) as i32;
        if !(2..=3).contains(&status) { return 0; }
    if (read(payload, PAYLOAD_INDEX) as i32) < 0 { return 0; }
        let validated = lf_checker_rt::callee_thiscall!(1, u32, target,
            read(payload, PAYLOAD_KEY_LO), read(payload, PAYLOAD_KEY_HI), this);
        if validated & 0xff == 0 { return 0; }
        let same = read(this, ACTIVE_TOKEN_LO) == read(target, 0)
            && read(this, ACTIVE_TOKEN_HI) == read(target, 4);
        u32::from(same)
    }
});
