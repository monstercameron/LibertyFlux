// original: 0x009AAE30 MOBILE_CHAT

/// Mobile-chat voice update: clears finished voice slots, runs a gated
/// chat-event push, then re-balances three voice levels from a queried factor.
///
/// `this` is the chat/voice state object. Part 1 clears up to seven voice
/// slots: three indexed slots (the byte at +0x368/+0x369/+0x36a times 96
/// selects a slot in the banks at +0x8/+0x128/+0x248) and four fixed slots
/// (+0x36c, +0x3cc, +0x42c, +0x48c). A slot whose flag byte is 3 and whose
/// linked object (the dword 8 past the flag) has a zero word at +8 is reset
/// to 0. Part 2 runs a preparatory pass over `this`, then, unless the
/// global chat-request byte is clear (it is cleared when seen), pushes a
/// ten-argument chat event naming the mobile-chat channel. Part 3 runs a
/// second pass, queries a float factor and a selector byte from a global
/// audio object into stack slots, and forms the level L = 2F + (1 - F)
/// (or exactly 1.0 when the selector byte is set). Part 4 stores into each
/// of three global voice objects (skipping null ones): the first gets L,
/// the others get 1.0 / L.
///
/// The return value is the last callee answer observed (the query's, or a
/// voice store's when at least one ran). All float arithmetic keeps the
/// original's operand order.
///
/// Original: 0x009AAE30 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_009aae30(this: u32) -> u32 {
    unsafe {
        const INDEX_BASE: u32 = 0x368;
        const BANK_BASES: [u32; 3] = [0x8, 0x128, 0x248];
        const FIXED_SLOTS: [u32; 4] = [0x36c, 0x3cc, 0x42c, 0x48c];
        const FINISHED: u8 = 3;
        const CHAT_FLAG_WORD: u32 = 0x01284378;
        const VOICE_GLOBALS: [u32; 3] = [0x01284398, 0x0128439C, 0x012843A0];
        const AUDIO_OBJECT: u32 = 0x01165880;
        const ONE: u32 = 0x00FE88E8;
        const TWO: u32 = 0x01038ECC;
        const CHANNEL_NAME: u32 = 0x00E92310;
        const PREPARE: u32 = 1;
        const REQUEST_POLL: u32 = 2;
        const PUSH_EVENT: u32 = 3;
        const POST_PASS: u32 = 4;
        const QUERY_FACTOR: u32 = 5;
        const STORE_LEVEL: u32 = 6;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn glob(va: u32) -> f32 {
            unsafe { rdf(lf_checker_rt::relocated(va)) }
        }

        // Part 1: clear finished indexed slots, then finished fixed slots.
        for i in 0..3u32 {
            let idx = rd8(this.wrapping_add(INDEX_BASE.wrapping_add(i)));
            let slot = this
                .wrapping_add((idx as u32).wrapping_mul(96))
                .wrapping_add(BANK_BASES[i as usize]);
            if rd8(slot) == FINISHED {
                let linked = rd32(slot.wrapping_add(8));
                if rd32(linked.wrapping_add(8)) == 0 {
                    wr8(slot, 0);
                }
            }
        }
        for i in 0..4usize {
            let slot = this.wrapping_add(FIXED_SLOTS[i]);
            if rd8(slot) == FINISHED {
                let linked = rd32(slot.wrapping_add(8));
                if rd32(linked.wrapping_add(8)) == 0 {
                    wr8(slot, 0);
                }
            }
        }

        // Part 2: preparatory pass, then the gated chat-event push.
        let _: u32 = lf_checker_rt::callee_thiscall!(PREPARE, u32, this);
        let flag = rd8(lf_checker_rt::relocated(CHAT_FLAG_WORD).wrapping_add(1));
        if flag != 0 {
            wr8(lf_checker_rt::relocated(CHAT_FLAG_WORD).wrapping_add(1), 0);
            let r1: u32 = lf_checker_rt::callee_cdecl!(REQUEST_POLL, u32, 0u32);
            if r1 != 0 {
                let r2: u32 = lf_checker_rt::callee_cdecl!(REQUEST_POLL, u32, 0u32);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    PUSH_EVENT,
                    u32,
                    r2.wrapping_add(0x570),
                    lf_checker_rt::relocated(CHANNEL_NAME),
                    0u32,
                    0u32,
                    0u32,
                    0xFFFF_FFFFu32,
                    0u32,
                    0u32,
                    0x3F80_0000u32,
                    0u32,
                    0u32
                );
            }
        }

        // Part 3: second pass, then the factor query.
        let _: u32 = lf_checker_rt::callee_thiscall!(POST_PASS, u32, this);
        let mut factor: u32 = 0;
        let mut selector: u32 = 0;
        let mut eax: u32 = lf_checker_rt::callee_thiscall!(
            QUERY_FACTOR,
            u32,
            lf_checker_rt::relocated(AUDIO_OBJECT),
            &mut factor as *mut u32 as u32,
            &mut selector as *mut u32 as u32
        );
        let f = f32::from_bits(factor);
        let level = if (selector & 0xFF) == 0 {
            add(mul(glob(TWO), f), sub(glob(ONE), f))
        } else {
            glob(ONE)
        };

        // Part 4: store the level into each live voice object.
        for i in 0..3usize {
            let obj = rd32(lf_checker_rt::relocated(VOICE_GLOBALS[i]));
            if obj != 0 {
                let x = if i == 0 { level } else { div(glob(ONE), level) };
                eax = lf_checker_rt::callee_thiscall!(STORE_LEVEL, u32, obj, x.to_bits());
            }
        }
        eax
    }
});
