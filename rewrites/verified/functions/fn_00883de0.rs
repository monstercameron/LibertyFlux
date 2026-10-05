// original: 0x00883de0 stream_channel_push (proposed)
/// Push a value pair through a channel when the channel is armed.
///
/// Does nothing unless the armed flag at `this+0x62` is set. Otherwise reads
/// the descriptor `ref`: its kind nibble (low 4 bits of the word at
/// `ref+0x10`) selects the payload vector (`[ref+0x08] + 4` when the kind is
/// 1, `[ref+0x0c]` otherwise), and kind 1 forwards the two floats at the
/// base and partner vectors plus `first`, `second` and the channel word at
/// `[this]` to the five-argument blender (intercepted callee 1, cdecl);
/// any other kind forwards the kind itself and the two pointers instead to
/// the six-argument blender (intercepted callee 2, cdecl).
///
/// Original: thiscall, three stack arguments, callee cleans 12, no return value.
lf_checker_rt::export!(thiscall, rw_00883de0(this: u32, first: u32, second: u32, obj: u32) -> u32 {
    unsafe {
        const ARMED: u32 = 0x62;
        const PAYLOAD: u32 = 0x08;
        const PARTNER: u32 = 0x0c;
        const KIND_WORD: u32 = 0x10;
        const KIND_MASK: u32 = 0x0f;
        const BLEND5_CALLEE: u32 = 1;
        const BLEND6_CALLEE: u32 = 2;
        if ((this + ARMED) as *const u8).read() == 0 {
            return 0;
        }
        let kind = ((obj + KIND_WORD) as *const u32).read_unaligned() & KIND_MASK;
        let base = ((obj + PAYLOAD) as *const u32).read_unaligned();
        let partner = if kind == 1 {
            base.wrapping_add(4)
        } else {
            ((obj + PARTNER) as *const u32).read_unaligned()
        };
        if kind == 1 {
            let f0 = (base as *const u32).read_unaligned();
            let f1 = (partner as *const u32).read_unaligned();
            let head = (this as *const u32).read_unaligned();
            let _: u32 =
                lf_checker_rt::callee_cdecl!(BLEND5_CALLEE, u32, head, first, second, f1, f0);
        } else {
            let head = (this as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_cdecl!(
                BLEND6_CALLEE,
                u32,
                head,
                first,
                second,
                kind,
                partner,
                base
            );
        }
        0
    }
});
