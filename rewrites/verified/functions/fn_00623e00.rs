// original: 0x00623E00 network_payload_token_match

/// Validate a payload against the receiver's active token and a target record.
/// The receiver status at 0x50 and payload index at 0x18 are signed: status
/// must be 2 or 3 and the index must be nonnegative. A lookup helper receives
/// the active token words at 0xc30/0xc34; its returned pointer supplies the
/// index word, or -1 when null. A second helper receives payload words at
/// 0x1c/0x20 and the receiver. If its low byte is nonzero, the receiver's
/// words at 0x540/0x544 must equal the target's first two words.
lf_checker_rt::export!(thiscall, rw_00623e00(this: u32, payload: u32, target: u32) -> u32 {
    unsafe {
        const STATUS: u32 = 0x50;
        const PAYLOAD_INDEX: u32 = 0x18;
        const PAYLOAD_KEY_LO: u32 = 0x1c;
        const PAYLOAD_KEY_HI: u32 = 0x20;
        const ACTIVE_TOKEN_LO: u32 = 0xc30;
        const ACTIVE_TOKEN_HI: u32 = 0xc34;
        const TARGET_TOKEN_LO: u32 = 0x540;
        const TARGET_TOKEN_HI: u32 = 0x544;
        let read = |base: u32, offset: u32| (base.wrapping_add(offset) as *const u32).read_unaligned();
        let status = read(this, STATUS) as i32;
        if !(2..=3).contains(&status) { return 0; }
        let payload_index = read(payload, PAYLOAD_INDEX) as i32;
        if payload_index < 0 { return 0; }
        let lookup = lf_checker_rt::callee_thiscall!(1, u32, this,
            read(this, ACTIVE_TOKEN_LO), read(this, ACTIVE_TOKEN_HI));
        let looked_up_index = if lookup == 0 { u32::MAX } else { read(lookup, 0) };
        if looked_up_index != payload_index as u32 { return 0; }
        let validated = lf_checker_rt::callee_thiscall!(2, u32, target,
            read(payload, PAYLOAD_KEY_LO), read(payload, PAYLOAD_KEY_HI), this);
        if validated & 0xff == 0 { return 0; }
        let same = read(this, TARGET_TOKEN_LO) == read(target, 0)
            && read(this, TARGET_TOKEN_HI) == read(target, 4);
        u32::from(same)
    }
});
