// original: 0x00d781e0 CRenderPhaseWarpShadow::vf7

/// Queue one warp-shadow render phase: gate on two global flags and a
/// readiness check, then build four render-command packets and hand each to
/// the command consumer.
///
/// `this` points to the render-phase object; only its vtable pointer (at
/// `+0x00`, slots `+0x20`/`+0x24`) and the descriptor block at `+0xb0` are
/// used. Behaviour:
/// - Return without doing anything when readiness gate 0 is clear and gate 1
///   is set, or when the readiness callee answers with a zero low byte.
/// - Otherwise record `this` in the in-flight global, then four times call
///   the packet allocator with a (size, kind) pair and, unless it answers
///   null, construct the packet in place (first packet: virtual slot `+0x24`
///   result fed to a wrap callee; second: descriptor at `+0xb0` bound by id;
///   third: tagged inline with two relocated vtable constants and a masked
///   mix of the global sequence counter, which is then incremented; fourth:
///   default-constructed) and pass the packet, or null, to the consumer.
/// - Clear the in-flight global and return.
///
/// Original: 0x00d781e0 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00d781e0(this: u32) -> u32 {
    unsafe {
        const GATE0: u32 = 0x103E488;
        const GATE1: u32 = 0x11609F6;
        const IN_FLIGHT: u32 = 0x12FB1B8;
        const SEQ: u32 = 0x10327A0;
        const DESCRIPTOR: u32 = 0xb0;
        const VT_BUILD_FIRST: u32 = 0x24;
        const VT_FINISH: u32 = 0x20;
        const TAG_TENTATIVE: u32 = 0xE7E048;
        const TAG_FINAL: u32 = 0xE8668C;
        const SEQ_MASK: u32 = 0x3fff;
        const READY: u32 = 1;
        const ALLOC_FIRST: u32 = 2;
        const ALLOC_SECOND: u32 = 9;
        const ALLOC_THIRD: u32 = 10;
        const ALLOC_FOURTH: u32 = 11;
        const WRAP_FIRST: u32 = 4;
        const CONSUME: u32 = 5;
        const BIND_SECOND: u32 = 6;
        const BUILD_FOURTH: u32 = 8;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn consume(packet: u32) {
            unsafe {
                lf_checker_rt::callee_cdecl!(CONSUME, u32, packet);
            }
        }
        #[inline(always)]
        unsafe fn vcall(this: u32, slot: u32) -> u32 {
            unsafe {
                let target: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this) + slot) as usize);
                target(this)
            }
        }

        if rd8(lf_checker_rt::relocated(GATE0)) == 0
            && rd8(lf_checker_rt::relocated(GATE1)) != 0
        {
            return 0;
        }
        let ready: u32 = lf_checker_rt::callee_cdecl!(READY, u32,);
        if ready & 0xff == 0 {
            return 0;
        }

        wr32(lf_checker_rt::relocated(IN_FLIGHT), this);

        let first: u32 = lf_checker_rt::callee_cdecl!(ALLOC_FIRST, u32, 0x10u32, 1u32);
        if first == 0 {
            consume(0);
        } else {
            let built = vcall(this, VT_BUILD_FIRST);
            let wrapped: u32 = lf_checker_rt::callee_thiscall!(WRAP_FIRST, u32, first, built);
            consume(wrapped);
        }

        let second: u32 = lf_checker_rt::callee_cdecl!(ALLOC_SECOND, u32, 0x410u32, 0u32);
        if second == 0 {
            consume(0);
        } else {
            let bound: u32 =
                lf_checker_rt::callee_thiscall!(BIND_SECOND, u32, second, this.wrapping_add(DESCRIPTOR));
            consume(bound);
        }

        vcall(this, VT_FINISH);

        let third: u32 = lf_checker_rt::callee_cdecl!(ALLOC_THIRD, u32, 8u32, 0u32);
        if third == 0 {
            consume(0);
        } else {
            let mix = rd32(third.wrapping_add(4));
            wr32(third, lf_checker_rt::relocated(TAG_TENTATIVE));
            let seq = rd32(lf_checker_rt::relocated(SEQ));
            let masked = (mix ^ seq) & SEQ_MASK;
            wr32(third.wrapping_add(4), mix ^ masked);
            wr32(
                lf_checker_rt::relocated(SEQ),
                rd32(lf_checker_rt::relocated(SEQ)).wrapping_add(1),
            );
            wr32(third, lf_checker_rt::relocated(TAG_FINAL));
            consume(third);
        }

        let fourth: u32 = lf_checker_rt::callee_cdecl!(ALLOC_FOURTH, u32, 8u32, 0u32);
        if fourth == 0 {
            consume(0);
        } else {
            let built: u32 = lf_checker_rt::callee_thiscall!(BUILD_FOURTH, u32, fourth);
            consume(built);
        }

        wr32(lf_checker_rt::relocated(IN_FLIGHT), 0);
        0
    }
});
