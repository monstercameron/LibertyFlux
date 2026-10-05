// original: 0x00884050 stream_channel_start (proposed)
/// Start a channel: latch the mode byte, publish the rate ratio, notify every live slot.
///
/// Stores the low byte of `mode` at `this+0x60`, then publishes the ratio of
/// the table size (through the manager at `this+0x20`, size at `manager+0x80`)
/// over the channel width at `this+0x10` — both converted from `u32` through
/// `f64` exactly as the original's `cvtdq2pd`/`addsd`/`cvtpd2ps` chain, then
/// divided with a single-precision division in the original's operand order
/// — to the rate setter (intercepted callee 1, cdecl: the rate slot at
/// `this+0x28`, then the ratio). Then walks the eight half-word slots at
/// `this+0x4c`, skipping `FREE` (`0xffff`) entries: each live index is
/// resolved through the pool lookup (intercepted callee 2, cdecl, one
/// argument) and the object's payload (`[obj+0x08]`), kind nibble times four
/// (low 4 bits of `[obj+0x10]`), and owner pointer (payload `+ 4` when the
/// kind is 1, `[obj+0x0c]` otherwise) are reported to the notify routine
/// (intercepted callee 3, cdecl, three arguments). Finishes by setting the
/// started flag at `this+0x24`.
///
/// Original: thiscall, one stack argument, callee cleans 4, no return value.
lf_checker_rt::export!(thiscall, rw_00884050(this: u32, mode: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x20;
        const TABLE_SIZE: u32 = 0x80;
        const WIDTH: u32 = 0x10;
        const RATE_SLOT: u32 = 0x28;
        const STARTED: u32 = 0x24;
        const MODE_BYTE: u32 = 0x60;
        const SLOT_TABLE: u32 = 0x4c;
        const SLOT_COUNT: u32 = 8;
        const PAYLOAD: u32 = 0x08;
        const PARTNER: u32 = 0x0c;
        const KIND_WORD: u32 = 0x10;
        const KIND_MASK: u32 = 0x0f;
        const FREE: u16 = 0xffff;
        const RATE_CALLEE: u32 = 1;
        const LOOKUP_CALLEE: u32 = 2;
        const NOTIFY_CALLEE: u32 = 3;
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        ((this + MODE_BYTE) as *mut u8).write(mode as u8);
        let manager = ((this + MANAGER) as *const u32).read_unaligned();
        let size = ((manager + TABLE_SIZE) as *const u32).read_unaligned();
        let width = ((this + WIDTH) as *const u32).read_unaligned();
        let ratio = div((size as f64) as f32, (width as f64) as f32);
        let _: u32 = lf_checker_rt::callee_cdecl!(
            RATE_CALLEE,
            u32,
            this.wrapping_add(RATE_SLOT),
            ratio.to_bits()
        );
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let index = ((this + SLOT_TABLE + i.wrapping_mul(2)) as *const u16).read_unaligned();
            if index != FREE {
                let obj: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, index as u32);
                let kind =
                    ((obj + KIND_WORD) as *const u32).read_unaligned() & KIND_MASK;
                let payload = ((obj + PAYLOAD) as *const u32).read_unaligned();
                let owner = if kind == 1 {
                    payload.wrapping_add(4)
                } else {
                    ((obj + PARTNER) as *const u32).read_unaligned()
                };
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    NOTIFY_CALLEE,
                    u32,
                    owner,
                    payload,
                    kind.wrapping_mul(4)
                );
            }
            i += 1;
        }
        ((this + STARTED) as *mut u32).write_unaligned(1);
        0
    }
});
