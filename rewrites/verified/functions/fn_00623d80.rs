// original: 0x00623D80 network_payload_token_match

/// Validate a network payload against the receiver's active token and the
/// supplied target record. The receiver status at 0x50 and the payload index
/// at 0x18 are signed guards: status must be 2 or 3, and the index must be
/// nonnegative. A lookup helper is called with the active token words at
/// 0xc30/0xc34; its returned pointer contributes its first word, or -1 when
/// null, which must equal the payload index. A validator then receives the
/// payload words at 0x1c/0x20 and the receiver. If it returns a nonzero low
/// byte, the receiver's words at 0x540/0x544 must equal the target's first two
/// words. The stack arguments are the payload pointer and target pointer.
lf_checker_rt::export!(thiscall, rw_00623d80(this: u32, payload: u32, target: u32) -> u32 {
    unsafe {
        const STATUS: u32 = 0x50;
        const PAYLOAD_INDEX: u32 = 0x18;
        const PAYLOAD_KEY_LO: u32 = 0x1c;
        const PAYLOAD_KEY_HI: u32 = 0x20;
        const ACTIVE_TOKEN_LO: u32 = 0xc30;
        const ACTIVE_TOKEN_HI: u32 = 0xc34;
        const TARGET_TOKEN_LO: u32 = 0x540;
        const TARGET_TOKEN_HI: u32 = 0x544;
        let read = |address: u32| (address as *const u32).read_unaligned();
        let status = read(this.wrapping_add(STATUS)) as i32;
        if !(2..=3).contains(&status) {
            return 0;
        }
        let payload_index = read(payload.wrapping_add(PAYLOAD_INDEX)) as i32;
        if payload_index < 0 {
            return 0;
        }
        let lookup = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            this,
            read(this.wrapping_add(ACTIVE_TOKEN_LO)),
            read(this.wrapping_add(ACTIVE_TOKEN_HI)),
        );
        let looked_up_index = if lookup == 0 { u32::MAX } else { read(lookup) };
        if looked_up_index != payload_index as u32 {
            return 0;
        }
        let validated = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            target,
            read(payload.wrapping_add(PAYLOAD_KEY_LO)),
            read(payload.wrapping_add(PAYLOAD_KEY_HI)),
            this,
        );
        if validated & 0xff == 0 {
            return 0;
        }
        u32::from(
            read(this.wrapping_add(TARGET_TOKEN_LO)) == read(target)
                && read(this.wrapping_add(TARGET_TOKEN_HI)) == read(target.wrapping_add(4)),
        )
    }
});
