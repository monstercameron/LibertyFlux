// original: 0x00884110 stream_channel_update (proposed)
/// Run one channel update tick: countdown, notify live slots, disarm.
///
/// Does nothing unless the armed flag at `this+0x62` is set. When the repeat
/// flag at `this+0x61` is set, restarts a zero countdown at `this+0x14` to 2
/// and decrements it; a countdown reaching zero raises the done flag, clears
/// the started word at `this+0x24` and the repeat flag. (That also skips the
/// notify walk below, which provably leaves the done arm of the notify call
/// unreachable; see below.) When the started word is non-zero, walks the
/// eight half-word slots at `this+0x4c`, skipping `FREE` (`0xffff`) entries:
/// each live index is resolved through the pool lookup (intercepted callee
/// 1, cdecl, one argument) and reported — payload (`[obj+0x08]`), owner
/// (payload `+ 4` when the kind nibble, low 4 bits of `[obj+0x10]`, is 1,
/// `[obj+0x0c]` otherwise), and kind times four — either to the done
/// routine (intercepted callee 2, cdecl: owner, null, kind) when the done
/// flag is set, or to the notify routine (intercepted callee 3, cdecl:
/// owner, payload, kind) otherwise. Finishes by clearing the armed flag.
///
/// The done arm never fires: the done flag is only set together with writing
/// 0 to the started word, and the walk only runs when that word is non-zero.
/// The rewrite keeps the arm for faithfulness; the contract exempts callee 2
/// from the must-fire rule.
///
/// Original: thiscall, no stack arguments, no return value.
lf_checker_rt::export!(thiscall, rw_00884110(this: u32) -> u32 {
    unsafe {
        const COUNTDOWN: u32 = 0x14;
        const STARTED: u32 = 0x24;
        const SLOT_TABLE: u32 = 0x4c;
        const SLOT_COUNT: u32 = 8;
        const REPEAT: u32 = 0x61;
        const ARMED: u32 = 0x62;
        const PAYLOAD: u32 = 0x08;
        const PARTNER: u32 = 0x0c;
        const KIND_WORD: u32 = 0x10;
        const KIND_MASK: u32 = 0x0f;
        const FREE: u16 = 0xffff;
        const RESTART: u32 = 2;
        const LOOKUP_CALLEE: u32 = 1;
        const DONE_CALLEE: u32 = 2;
        const NOTIFY_CALLEE: u32 = 3;
        if ((this + ARMED) as *const u8).read() == 0 {
            return 0;
        }
        let mut done = false;
        if ((this + REPEAT) as *const u8).read() != 0 {
            let count = (this + COUNTDOWN) as *mut u32;
            if count.read_unaligned() == 0 {
                count.write_unaligned(RESTART);
            }
            let left = count.read_unaligned().wrapping_sub(1);
            count.write_unaligned(left);
            if left == 0 {
                done = true;
                ((this + STARTED) as *mut u32).write_unaligned(0);
                ((this + REPEAT) as *mut u8).write(0);
            }
        }
        if ((this + STARTED) as *const u32).read_unaligned() != 0 {
            let mut i = 0u32;
            while i < SLOT_COUNT {
                let index =
                    ((this + SLOT_TABLE + i.wrapping_mul(2)) as *const u16).read_unaligned();
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
                    if done {
                        let _: u32 = lf_checker_rt::callee_cdecl!(
                            DONE_CALLEE,
                            u32,
                            owner,
                            0,
                            kind.wrapping_mul(4)
                        );
                    } else {
                        let _: u32 = lf_checker_rt::callee_cdecl!(
                            NOTIFY_CALLEE,
                            u32,
                            owner,
                            payload,
                            kind.wrapping_mul(4)
                        );
                    }
                }
                i += 1;
            }
        }
        ((this + ARMED) as *mut u8).write(0);
        0
    }
});
