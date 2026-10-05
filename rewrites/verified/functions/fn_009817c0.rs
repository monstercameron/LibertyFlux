// original: 0x009817c0 audAmbientAudioEntity::vf1 (symbols)

/// Reinitialise an ambient-audio entity and register its slot names.
///
/// `this` points to the entity (about 28 KB; fields below are byte offsets).
/// First, byte 0 of each of 245 sixteen-byte slots is cleared: the slots
/// start at `CLEAR_BASE` and are `SLOT_STRIDE` bytes apart, and four counter
/// words (`ZERO_WORDS`) are zeroed. Then a lookup callee is asked for the
/// entity's voice table by name; when it answers null, or the table's voice
/// count byte (at `TABLE_COUNT`) is zero, the middle step is skipped.
/// Otherwise the count is stored at `VOICE_COUNT`, the head block at
/// `HEAD_BLOCK` is filled in (its allocator is called only when the block's
/// tag word at `+6` is still zero; the tag and generation words at `+6` and
/// `+4` then hold the count), and each voice pointer in the table (dwords
/// starting at `TABLE_FIRST`) is resolved through a second lookup callee and
/// handed with the voice's head (`head + i * VOICE_STRIDE`) to a per-voice
/// initialiser. Finally eight (slot, name) pairs are registered through one
/// shared callee, a tail callee finishes the entity, two state words are
/// zeroed, and control passes to the shared tail routine with `this`, whose
/// result is the return value.
///
/// Two branches in the original are dead: the count is provably non-zero
/// where it is re-tested, and the voice loop always runs once the count is
/// stored. The rewrite omits both tests.
///
/// Original: 0x009817c0 (thiscall, no stack arguments; tail call to the
/// shared routine with `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009817c0(this: u32) -> u32 {
    unsafe {
        const CLEAR_BASE: u32 = 0x5C0C;
        const SLOT_STRIDE: u32 = 0x14;
        const CLEAR_COUNT: u32 = 245;
        const ZERO_WORDS: [u32; 4] = [0x5BF8, 0x5BF0, 0x5BF4, 0x6F24];
        const TABLE_COUNT: u32 = 0x0A;
        const TABLE_FIRST: u32 = 0x0B;
        const VOICE_COUNT: u32 = 0x6F24;
        const HEAD_BLOCK: u32 = 0x6F28;
        const VOICE_STRIDE: u32 = 0x360;
        const LOOKUP_OBJ: u32 = 0x115D9A0;
        const TABLE_NAME: u32 = 0xE8D8D8;
        const LOOKUP: u32 = 1;
        const ALLOC: u32 = 2;
        const RESOLVE: u32 = 3;
        const INIT_VOICE: u32 = 4;
        const REGISTER: u32 = 5;
        const FINISH: u32 = 6;
        const TAIL: u32 = 7;
        const SLOTS: [(u32, u32); 8] = [
            (0x1238810, 0xE8D8EC),
            (0x1231788, 0xE8D908),
            (0x12317B0, 0xE8D920),
            (0x1238760, 0xE8D938),
            (0x1238788, 0xE8D94C),
            (0x12387B0, 0xE8D960),
            (0x12387D8, 0xE8D974),
            (0x12317D8, 0xE8D988),
        ];

        for i in 0..CLEAR_COUNT {
            ((this + CLEAR_BASE + i * SLOT_STRIDE) as *mut u8).write(0);
        }
        for off in ZERO_WORDS {
            ((this + off) as *mut u32).write(0);
        }
        let found: u32 = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            lf_checker_rt::relocated(LOOKUP_OBJ),
            lf_checker_rt::relocated(TABLE_NAME)
        );
        if found != 0 {
            let count = ((found + TABLE_COUNT) as *const u8).read() as u32;
            if count != 0 {
                ((this + VOICE_COUNT) as *mut u32).write(count);
                let head_blk = this + HEAD_BLOCK;
                if ((head_blk + 6) as *const u16).read() == 0 {
                    ((head_blk + 6) as *mut u16).write(count as u16);
                    let obj: u32 =
                        lf_checker_rt::callee_thiscall!(ALLOC, u32, head_blk, count);
                    (head_blk as *mut u32).write(obj);
                }
                ((head_blk + 4) as *mut u16).write(count as u16);
                let base = found + TABLE_FIRST;
                let head = (head_blk as *const u32).read();
                for i in 0..count {
                    let arg = ((base + i * 4) as *const u32).read();
                    let v: u32 = lf_checker_rt::callee_thiscall!(
                        RESOLVE,
                        u32,
                        lf_checker_rt::relocated(LOOKUP_OBJ),
                        arg
                    );
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        INIT_VOICE,
                        u32,
                        head.wrapping_add(i * VOICE_STRIDE),
                        v
                    );
                }
            }
        }
        for (slot, name) in SLOTS {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                REGISTER,
                u32,
                lf_checker_rt::relocated(slot),
                lf_checker_rt::relocated(name)
            );
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(FINISH, u32, this);
        ((this + 0x6F38) as *mut u32).write(0);
        ((this + 0x6F20) as *mut u32).write(0);
        lf_checker_rt::callee_thiscall!(TAIL, u32, this)
    }
});
