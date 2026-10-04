// original: 0x00ae9400 range_chain_refresh (proposed)

/// Refresh every table slot's object chain in an enumerated range.
///
/// `a1` and `a2` are forwarded to an enumerator callee that fills a scratch
/// array with chain heads and returns how many. Each head is dereferenced
/// (null heads and null dereferences are skipped) and the resulting chain is
/// processed exactly like the single chain of the neighbouring slot routine:
/// per node, flag gates, an interlocked stamp exchange, convert-and-store or
/// publish by bit 15, else fill-to-emitter, virtual prepare, step, and two
/// final calls observing the emitter. The scratch float forwarded to the last
/// call is the live array-pointer slot, i.e. incidental stack-address bits
/// that legitimately differ between the two sides' frames; the contract
/// skips that one argument. The outer index can never be `-1` (it starts at
/// zero and only increments), so that guard is dead but transcribed.
/// Software prefetches are omitted (unobservable). Original: cdecl, two stack
/// words, no meaningful return value.
lf_checker_rt::export!(cdecl, rw_00ae9400(a1: u32, a2: u32) -> u32 {
    unsafe {
        const G_TICK: u32 = 0x01593BC4;
        const G_WORD: u32 = 0x011A8908;
        const G_MODE1: u32 = 0x015B0E6D;
        const G_MODE2: u32 = 0x015B0E6E;
        const G_ARG: u32 = 0x0118D760;
        const G_OBJ: u32 = 0x0118D7E0;
        const IMM_ARG: u32 = 0x0118D760;
        const IMM_PUB: u32 = 0x0159AF1C;
        const O_FLAGS: u32 = 0x24;
        const O_OUT: u32 = 0x08;
        const O_OUT_C: u32 = 0x0C;
        const O_KIND: u32 = 0x28;
        const O_STAMP: u32 = 0x3C;
        const VT_SLOT: u32 = 0x6C;
        const C_TICK: u32 = 1;
        const C_STAMP: u32 = 2;
        const C_CONVERT: u32 = 3;
        const C_FILL: u32 = 4;
        const C_EMIT_V: u32 = 5;
        const C_STEP: u32 = 6;
        const C_APPLY: u32 = 7;
        const C_EMIT: u32 = 8;
        const C_PUB: u32 = 9;
        const C_ENUM: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let tick = rd32(lf_checker_rt::relocated(G_TICK));
        let now: u32 = lf_checker_rt::callee_cdecl!(C_TICK, u32,);
        if tick.wrapping_sub(now) & 0xFFFF_FFF8 >= 0x10000 {
            return 0;
        }
        let edx_saved = rd16(lf_checker_rt::relocated(G_WORD)) as u32;
        let mut heads = [0u32; 4];
        let count: u32 = lf_checker_rt::callee_cdecl!(
            C_ENUM,
            u32,
            a1,
            a2,
            core::ptr::addr_of_mut!(heads[0]) as u32
        );
        if count == 0 {
            return 0;
        }
        let mut remaining = count;
        let mut walk = core::ptr::addr_of_mut!(heads[0]) as u32;
        let mut index = 0u32;
        loop {
            let mut link = rd32(walk);
            remaining = remaining.wrapping_sub(1);
            if link != 0 && index != 0xFFFF_FFFF {
                link = rd32(link);
                if link != 0 {
                    loop {
                        let obj = rd32(link);
                        link = rd32(link.wrapping_add(4));
                        let al = rd32(obj.wrapping_add(O_FLAGS)) as u8;
                        let mut proceed = false;
                        if al & 0x40 == 0 || rd8(lf_checker_rt::relocated(G_MODE1)) == 0 {
                            proceed = al & 0x80 == 0
                                || rd8(lf_checker_rt::relocated(G_MODE2)) == 0;
                        } else if al & 0x80 != 0 {
                            proceed = rd8(lf_checker_rt::relocated(G_MODE2)) == 0;
                        }
                        if proceed {
                            let old: u32 = lf_checker_rt::callee_stdcall!(
                                C_STAMP,
                                u32,
                                obj.wrapping_add(O_STAMP),
                                edx_saved
                            );
                            if old != edx_saved {
                                let conv: u32 =
                                    lf_checker_rt::callee_cdecl!(C_CONVERT, u32, obj);
                                wr32(
                                    obj.wrapping_add(O_FLAGS),
                                    rd32(obj.wrapping_add(O_FLAGS)) & 0xFFFB_FFFF,
                                );
                                wr32(obj.wrapping_add(O_OUT), conv);
                                wr32(obj.wrapping_add(O_OUT_C), !conv);
                                if rd32(obj.wrapping_add(O_KIND)) >> 15 & 1 != 0 {
                                    let slot: u32 = lf_checker_rt::callee_stdcall!(
                                        C_PUB,
                                        u32,
                                        lf_checker_rt::relocated(IMM_PUB),
                                        8
                                    );
                                    wr32(slot, obj);
                                } else {
                                    let arg = rd32(lf_checker_rt::relocated(G_ARG));
                                    let mut p1 = [0u32; 4];
                                    p1[0] = obj;
                                    let mut p2 = [0u32; 4];
                                    let _: u32 = lf_checker_rt::callee_cdecl!(
                                        C_FILL,
                                        u32,
                                        core::ptr::addr_of_mut!(p1[0]) as u32,
                                        core::ptr::addr_of_mut!(p2[0]) as u32,
                                        arg
                                    );
                                    let emitter = p1[0];
                                    if emitter != 0 {
                                        let vtable = rd32(emitter);
                                        let target = rd32(vtable.wrapping_add(VT_SLOT));
                                        let sink = rd32(lf_checker_rt::relocated(G_OBJ));
                                        let mut vbuf = [0u32; 4];
                                        let hook: extern "thiscall" fn(u32, u32) -> u32 =
                                            core::mem::transmute(target as usize);
                                        hook(emitter, core::ptr::addr_of_mut!(vbuf[0]) as u32);
                                        let _: u32 = lf_checker_rt::callee_cdecl!(
                                            C_STEP,
                                            u32,
                                            p1[0],
                                            lf_checker_rt::relocated(IMM_ARG),
                                            sink,
                                            core::ptr::addr_of_mut!(vbuf[0]) as u32
                                        );
                                        let _: u32 = lf_checker_rt::callee_cdecl!(
                                            C_APPLY,
                                            u32,
                                            emitter,
                                            lf_checker_rt::relocated(IMM_ARG),
                                            sink
                                        );
                                        // The original forwards its live
                                        // array-pointer slot (frame-address
                                        // bits); the rewrite forwards its own
                                        // walk pointer. Compared skipped: the
                                        // two frames legitimately differ.
                                        let _: u32 = lf_checker_rt::callee_cdecl!(
                                            C_EMIT, u32, emitter, walk
                                        );
                                    }
                                }
                            }
                        }
                        if link == 0 {
                            break;
                        }
                    }
                }
            }
            index = index.wrapping_add(1);
            walk = walk.wrapping_add(4);
            if remaining == 0 {
                return 0;
            }
        }
    }
});
