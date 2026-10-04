// original: 0x00ae9260 slot_chain_refresh (proposed)

/// Refresh one table slot's object chain, emitting the convertible members.
///
/// `a1` and `a2` select the chain: the head is read from a global table at
/// index `a1 + a2 * 30`.
///
/// Behaviour: a tick callee is asked for the current time and the function
/// returns when the masked distance to the stored tick reaches `0x10000`. For
/// each chain node (node holds the object at `+0`, the next node at `+4`) the
/// object is skipped unless its flag word (`+0x24`) passes two global-mode
/// gates; an interlocked exchange on the object's stamp (`+0x3C`) whose old
/// value still equals the global frame word also skips it. Otherwise a
/// convert callee's answer is stored at `+8` with its complement at `+0xC`,
/// the flag bit `0x40000` is cleared, and objects with bit 15 of `+0x28` set
/// are published through a second interlocked slot instead of being emitted:
/// a fill callee resolves the object to an emitter (a null answer skips the
/// node), the emitter's virtual slot `+0x6C` prepares a buffer, a step callee
/// consumes it, and two final callees observe the emitter plus a scratch
/// float that reads as zero under interception. The original's software
/// prefetches are omitted (unobservable). Original: cdecl, two stack words,
/// no meaningful return value.
lf_checker_rt::export!(cdecl, rw_00ae9260(a1: u32, a2: u32) -> u32 {
    unsafe {
        const G_TICK: u32 = 0x01593BC4;
        const G_WORD: u32 = 0x011A8908;
        const TABLE: u32 = 0x011D4028;
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
        let scaled = a2.wrapping_shl(4).wrapping_sub(a2);
        let idx = a1.wrapping_add(scaled.wrapping_mul(2));
        let mut link = rd32(lf_checker_rt::relocated(TABLE).wrapping_add(idx.wrapping_mul(4)));
        if link == 0 {
            return 0;
        }
        loop {
            let obj = rd32(link);
            link = rd32(link.wrapping_add(4));
            let al = rd32(obj.wrapping_add(O_FLAGS)) as u8;
            // Flag gates: bit 0x40 needs mode 1 clear or (mode 1 set and the
            // byte negative); a negative byte then needs mode 2 clear.
            let mut proceed = false;
            if al & 0x40 == 0 || rd8(lf_checker_rt::relocated(G_MODE1)) == 0 {
                proceed = al & 0x80 == 0 || rd8(lf_checker_rt::relocated(G_MODE2)) == 0;
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
                    let conv: u32 = lf_checker_rt::callee_cdecl!(C_CONVERT, u32, obj);
                    wr32(obj.wrapping_add(O_FLAGS), rd32(obj.wrapping_add(O_FLAGS)) & 0xFFFB_FFFF);
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
                            // The original reloads the emitter from its scratch
                            // word here (not entry-esi as a first reading
                            // suggested: the slot holds the fill callee's
                            // answer). The scratch float comes from an
                            // unwritten frame slot: zero under the contract's
                            // stack fill on both sides.
                            let _: u32 = lf_checker_rt::callee_cdecl!(
                                C_APPLY,
                                u32,
                                emitter,
                                lf_checker_rt::relocated(IMM_ARG),
                                sink
                            );
                            let _: u32 =
                                lf_checker_rt::callee_cdecl!(C_EMIT, u32, emitter, 0);
                        }
                    }
                }
            }
            if link == 0 {
                return 0;
            }
        }
    }
});
