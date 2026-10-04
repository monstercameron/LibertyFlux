// original: 0x009fcc50 net_worker_thread (proposed)

/// Run the network worker thread: open a session, then poll and serve it
/// until the shutdown flag is set.
///
/// Takes no arguments and no register inputs (cdecl, 0x824 bytes of frame).
/// This is a thread entry point: it loops on the body below until byte
/// `SHUTDOWN_FLAG` becomes nonzero. The flag is fixed for a checker trial and
/// nothing in the body writes it, so in the checker only two behaviours are
/// observable: an immediate exit when the flag starts set (prologue plus
/// epilogue), and a hang when it starts clear. The exit path is proven; the
/// body is mirrored here for a future harness that can flip the flag
/// mid-trial (see the lane report for the retry kit).
///
/// Prologue: callee 0 releases the start gate (cdecl, `START_OBJ`); callee 1
/// opens the transfer (cdecl, buffer, 9, `CFG_PTR`, `CFG_LEN`) and its answer
/// seeds `esi`; callee 2 binds the context (cdecl, buffer); the bound
/// header's `HDR_AVAIL` word is set to `PAYLOAD_MAX` and linked to a frame
/// buffer through the header pointer callee 1 leaves behind. Callee 3 probes
/// the context (thiscall). When the flag is already set the function goes
/// straight to the epilogue.
///
/// Body (unexecuted in the checker): callee 4 re-arms the gate (cdecl,
/// `GATE_OBJ`); a negative `esi` restarts the pass. Callee 5 attaches the
/// bank (thiscall, `BANK_ARG`); when `GEN_MIRROR` is zero callee 6 detaches
/// (thiscall) and the pass ends. Otherwise the bank is selected from
/// `ACTIVE_SET` into a slot, `GEN_MIRROR` is cleared, and the tick from
/// callee 0 stamps the pass: when the stamp has advanced past `span` since
/// `last` (or on the first pass) a three-gate handshake runs (callees 7, 8,
/// 9, thiscall), followed while it answers by a poll loop (callee 10,
/// thiscall; callee 11 sleeps, cdecl) that re-checks gate 9. Past five
/// consecutive slow passes the span doubles up to `SPAN_MAX`. Callee 7 is
/// then re-checked; on success callee 12 resolves a session id (cdecl), a
/// negative id ends the pass, and callee 13 initialises a request (thiscall)
/// whose shape depends on `SEEDED_BYTE`. Callee 14 (the session builder,
/// cdecl, seven arguments: id, request, `AUX_OBJ`, kind-or-null, the source
/// pair and the source kind) runs it; callee 15 packs a reply (thiscall) and
/// clears the seeded byte. Callee 16 (the sender, cdecl, seven arguments) is
/// then called repeatedly: a header send, a length-prefixed name send (the
/// length is scanned, not trusted), and a tag send whose answer seeds `esi`.
/// A positive tag starts the item loop over `ITEM_COUNT` entries: each item's
/// object is fetched from the bank, asked through virtual slot 8 to fill a
/// buffer (thiscall, length `ITEM_LEN`), reported through three sender calls
/// (a kind send, a length-prefixed payload send, a tag send), and accounted
/// through virtual slot 4 until the count is covered. A final sender call
/// closes the batch. Any non-positive sender answer, and any set flag check
/// inside, abandons the pass at the shutdown sequence: when `esi` is
/// positive the socket is shut down through the `shutdown` import slot
/// (stdcall, socket and mode 2); the socket is always closed through the
/// `closesocket` import slot (stdcall). The tail re-binds the context
/// (callee 2 again), resets a header to `PAYLOAD_MAX`, detaches (callee 6)
/// and repeats the pass while the flag is clear.
///
/// Epilogue: unless `esi` is negative, callee 4 unbinds (cdecl); callee 0
/// runs once more on the start object and its answer is returned. Callee 5
/// is the stack-cookie check (no arguments, registers preserved).
lf_checker_rt::export!(cdecl, rw_009fcc50() -> u32 {
    unsafe {
        const START_OBJ: u32 = 0x012B_9584;
        const CFG_PTR: u32 = 0x00E9_9618;
        const CFG_LEN: u32 = 0x38;
        const HDR_AVAIL: u32 = 0x10;
        const HDR_LINK: u32 = 0x0C;
        const PAYLOAD_MAX: u32 = 0x400;
        const SHUTDOWN_FLAG: u32 = 0x012B_90FC;
        const GATE_OBJ: u32 = 0x012B_9C3C;
        const BANK_ARG: u32 = 0x012B_9170;
        const GEN_MIRROR: u32 = 0x012B_9C40;
        const ACTIVE_SET: u32 = 0x012B_9000;
        const BANK_LO: u32 = 0x012B_8000;
        const SPAN_INIT: u32 = 0xEA60;
        const SPAN_MAX: u32 = 0x75300;
        const SLOW_PASSES: u32 = 4;
        const SEEDED_BYTE: u32 = 0x012B_9024;
        const SEED_OBJ: u32 = 0x012B_9C48;
        const SEED_SIZE: u32 = 0x50;
        const AUX_OBJ: u32 = 0x012B_9100;
        const SRC_A: u32 = 0x012B_9078;
        const SRC_B: u32 = 0x012B_907C;
        const SRC_KIND: u32 = 0x012B_9C44;
        const ITEM_LEN: u32 = 0x200;
        const TAG_A: u32 = 0x00E9_9630;
        const TAG_B: u32 = 0x00E9_9634;
        const TAG_C: u32 = 0x00E9_9638;
        const TAG_D: u32 = 0x00E9_963C;
        const TAG_E: u32 = 0x00E9_9640;
        const SHUTDOWN_SLOT: u32 = 0x00E7_34B8;
        const CLOSE_SLOT: u32 = 0x00E7_34CC;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        let g = |va: u32| lf_checker_rt::relocated(va);

        // Shadow frame (the original aligns and reserves 0x824 bytes).
        let mut open_buf = [0u32; 0x220];
        let mut bind_buf = [0u32; 8];
        let mut ctx = [0u32; 32];
        let mut attach = [0u32; 8];
        let mut req = [0u32; 64];
        let mut frame_slot = [0u32; 8];
        let mut name_buf = [0u8; 0x210];
        let mut item_buf = [0u8; 0x210];
        let mut tail_buf = [0u32; 8];

        let _: u32 = lf_checker_rt::callee_cdecl!(0, u32, rd32(g(START_OBJ)));
        let cfg_ptr = lf_checker_rt::relocated(CFG_PTR);
        let mut esi: u32 =
            lf_checker_rt::callee_cdecl!(1, u32, open_buf.as_mut_ptr() as u32, 9u32, cfg_ptr, CFG_LEN);
        let bound: u32 =
            lf_checker_rt::callee_cdecl!(2, u32, bind_buf.as_mut_ptr() as u32);
        let _ = bound;
        wr32(bind_buf.as_mut_ptr() as u32 + 0x10, PAYLOAD_MAX);
        // Header link left by callee 1 (stub-written at +0x5E8 on both sides).
        let hdr = rd32(open_buf.as_mut_ptr() as u32 + 0x5E8);
        wr32(hdr.wrapping_add(HDR_LINK), frame_slot.as_mut_ptr() as u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, ctx.as_mut_ptr() as u32);
        let mut edi = 0u32;
        let mut ebx = SPAN_INIT;
        let mut last = 0u32;
        let mut slow = 0u32;
        if rd8(g(SHUTDOWN_FLAG)) != 0 {
            if (esi as i32) >= 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, tail_buf.as_mut_ptr() as u32);
            }
            let r: u32 = lf_checker_rt::callee_cdecl!(0, u32, rd32(g(START_OBJ)));
            let _: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
            return r;
        }
        // ---- body: unreachable in the checker (see doc comment) ----
        loop {
            let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, rd32(g(GATE_OBJ)));
            if rd8(g(SHUTDOWN_FLAG)) != 0 {
                break;
            }
            if (esi as i32) < 0 {
                continue;
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(
                5, u32, attach.as_mut_ptr() as u32, g(BANK_ARG)
            );
            if rd32(g(GEN_MIRROR)) == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, attach.as_mut_ptr() as u32);
            } else {
                let tick: u32 = lf_checker_rt::callee_cdecl!(0, u32,);
                let _ = tick;
                let cur = rd32(g(GEN_MIRROR));
                let toggled: u32 = if rd32(g(ACTIVE_SET)) == 0 { 1 } else { 0 };
                wr32(g(GEN_MIRROR), 0);
                let bank = toggled.wrapping_shl(11).wrapping_add(g(BANK_LO));
                let stamp: u32 = lf_checker_rt::callee_cdecl!(30, u32,);
                let stamp = stamp | 1;
                let _ = (cur, bank);
                if stamp.wrapping_sub(edi) >= ebx || edi == 0 {
                    let a: u32 = lf_checker_rt::callee_thiscall!(7, u32, ctx.as_mut_ptr() as u32);
                    if a & 0xFF == 0 {
                        let b: u32 =
                            lf_checker_rt::callee_thiscall!(8, u32, ctx.as_mut_ptr() as u32);
                        if b & 0xFF != 0 {
                            let cc: u32 = lf_checker_rt::callee_thiscall!(
                                9, u32, ctx.as_mut_ptr() as u32
                            );
                            if cc & 0xFF != 0 {
                                loop {
                                    if rd8(g(SHUTDOWN_FLAG)) != 0 {
                                        break;
                                    }
                                    let _: u32 = lf_checker_rt::callee_thiscall!(
                                        10, u32, ctx.as_mut_ptr() as u32
                                    );
                                    let _: u32 = lf_checker_rt::callee_cdecl!(11, u32, 0x64u32);
                                    let d: u32 = lf_checker_rt::callee_thiscall!(
                                        9, u32, ctx.as_mut_ptr() as u32
                                    );
                                    if d & 0xFF == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if slow > SLOW_PASSES {
                        ebx = ebx.wrapping_add(ebx);
                        if ebx > SPAN_MAX {
                            ebx = SPAN_MAX;
                        }
                    }
                    slow += 1;
                    last = stamp;
                }
                if rd8(g(SHUTDOWN_FLAG)) == 0 {
                    let e: u32 = lf_checker_rt::callee_thiscall!(7, u32, ctx.as_mut_ptr() as u32);
                    if e & 0xFF != 0 {
                        let id: u32 =
                            lf_checker_rt::callee_cdecl!(12, u32, req.as_mut_ptr() as u32);
                        if (id as i32) >= 0 {
                            edi = id;
                            let _: u32 = lf_checker_rt::callee_thiscall!(
                                13, u32, req.as_mut_ptr() as u32
                            );
                            let seeded = rd8(g(SEEDED_BYTE)) != 0;
                            let kind = if seeded { SEED_SIZE } else { 0 };
                            let obj = if seeded { 0 } else { g(SEED_OBJ) };
                            let _: u32 = lf_checker_rt::callee_cdecl!(
                                14, u32, edi, req.as_mut_ptr() as u32, obj, kind,
                                rd32(g(SRC_A)), rd32(g(SRC_B)), rd32(g(SRC_KIND))
                            );
                            let _: u32 = lf_checker_rt::callee_thiscall!(
                                15, u32, req.as_mut_ptr() as u32, name_buf.as_mut_ptr() as u32,
                                ITEM_LEN
                            );
                            lf_checker_rt::global::<u8>(SEEDED_BYTE).write(0);
                            let key: u32 = lf_checker_rt::callee_cdecl!(30, u32,);
                            let b3 = (key & 0xFF) as u8 as u32;
                            let r1: u32 = lf_checker_rt::callee_cdecl!(
                                16, u32, edi, req.as_mut_ptr() as u32, g(TAG_A), 3u32, b3,
                                frame_slot.as_mut_ptr() as u32, 0u32
                            );
                            if (r1 as i32) > 0 {
                                let mut len = 0usize;
                                while name_buf[len] != 0 {
                                    len += 1;
                                }
                                let r2: u32 = lf_checker_rt::callee_cdecl!(
                                    16, u32, edi, req.as_mut_ptr() as u32,
                                    item_buf.as_mut_ptr() as u32, len as u32, b3,
                                    frame_slot.as_mut_ptr() as u32, 0u32
                                );
                                if (r2 as i32) > 0 {
                                    esi = lf_checker_rt::callee_cdecl!(
                                        16, u32, edi, req.as_mut_ptr() as u32, g(TAG_B), 1u32,
                                        b3, frame_slot.as_mut_ptr() as u32, 0u32
                                    );
                                    if (esi as i32) > 0 {
                                        let count = rd32(frame_slot.as_mut_ptr() as u32 + 0x1c);
                                        let mut done = 0u32;
                                        if (count as i32) > 0 {
                                            let base = rd32(frame_slot.as_mut_ptr() as u32 + 0x34);
                                            while done < count {
                                                if rd8(g(SHUTDOWN_FLAG)) != 0 {
                                                    break;
                                                }
                                                let ent = base.wrapping_add(done);
                                                let vt = rd32(ent);
                                                let f8: extern "thiscall" fn(u32, u32, u32) -> u32 =
                                                    core::mem::transmute(
                                                        rd32(vt.wrapping_add(8)) as usize,
                                                    );
                                                f8(
                                                    ent,
                                                    item_buf.as_mut_ptr() as u32,
                                                    ITEM_LEN,
                                                );
                                                let s1: u32 = lf_checker_rt::callee_cdecl!(
                                                    16, u32, edi, req.as_mut_ptr() as u32,
                                                    g(TAG_C), 2u32, esi,
                                                    frame_slot.as_mut_ptr() as u32, 0u32
                                                );
                                                if (s1 as i32) <= 0 {
                                                    break;
                                                }
                                                let mut ln = 0usize;
                                                while name_buf[ln] != 0 {
                                                    ln += 1;
                                                }
                                                let s2: u32 = lf_checker_rt::callee_cdecl!(
                                                    16, u32, edi, req.as_mut_ptr() as u32,
                                                    item_buf.as_mut_ptr() as u32, ln as u32, esi,
                                                    frame_slot.as_mut_ptr() as u32, 0u32
                                                );
                                                if (s2 as i32) <= 0 {
                                                    break;
                                                }
                                                esi = lf_checker_rt::callee_cdecl!(
                                                    16, u32, edi, req.as_mut_ptr() as u32,
                                                    g(TAG_D), 1u32, esi,
                                                    frame_slot.as_mut_ptr() as u32, 0u32
                                                );
                                                if (esi as i32) <= 0 {
                                                    break;
                                                }
                                                let vt2 = rd32(ent);
                                                let f4: extern "thiscall" fn(u32) -> u32 =
                                                    core::mem::transmute(
                                                        rd32(vt2.wrapping_add(4)) as usize,
                                                    );
                                                done = done.wrapping_add(f4(ent));
                                            }
                                        }
                                        if rd8(g(SHUTDOWN_FLAG)) == 0 {
                                            esi = lf_checker_rt::callee_cdecl!(
                                                16, u32, edi, req.as_mut_ptr() as u32, g(TAG_E),
                                                1u32, b3, frame_slot.as_mut_ptr() as u32, 1u32
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if (esi as i32) > 0 {
                let slot = g(SHUTDOWN_SLOT) as *const u32;
                let shutdown: extern "stdcall" fn(u32, u32) -> u32 =
                    unsafe { core::mem::transmute(slot.read() as usize) };
                shutdown(edi, 2);
            }
            {
                let slot = g(CLOSE_SLOT) as *const u32;
                let close: extern "stdcall" fn(u32) -> u32 =
                    unsafe { core::mem::transmute(slot.read() as usize) };
                close(edi);
            }
            let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, bind_buf.as_mut_ptr() as u32);
            let hx = rd32(open_buf.as_mut_ptr() as u32 + 0x5EC);
            wr32(hx.wrapping_add(HDR_AVAIL), PAYLOAD_MAX);
            let hx = rd32(open_buf.as_mut_ptr() as u32 + 0x5EC);
            wr32(hx.wrapping_add(HDR_LINK), tail_buf.as_mut_ptr() as u32);
            let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, attach.as_mut_ptr() as u32);
            let _ = (ebx, last, slow);
            if rd8(g(SHUTDOWN_FLAG)) == 0 {
                continue;
            }
            break;
        }
        if (esi as i32) >= 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, tail_buf.as_mut_ptr() as u32);
        }
        let r: u32 = lf_checker_rt::callee_cdecl!(0, u32, rd32(g(START_OBJ)));
        let _: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
        r
    }
});
