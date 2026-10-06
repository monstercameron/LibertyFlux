// original: 0x006228E0 net_session_join_result_apply (proposed)

/// Apply a session join/add result: notify the registered callback (or
/// build a fallback event), resolve the address record, initialise a
/// channel, then walk the session table notifying matches.
///
/// `this` is the session manager, `arg1` the result record (key words at
/// `+0x38`/`+0x3c`, id pair at `+0x40`), `arg2`/`arg3` opaque values passed
/// through to the channel initialiser. The function makes up to thirteen
/// outgoing calls (all intercepted by the checker):
///
/// * id 1 (`0x622ea0`, thiscall/2): key lookup, tried twice; answers the
///   record or null. Its first answer is saved and reused as the walk's
///   skip key.
/// * id 2 (register-indirect through `this+8`, thiscall/3): the result
///   callback, called with a frame scratch buffer, `arg1` and `this` when
///   the slot is set. When `this+4` is zero the original takes a second
///   call site that falls into a shared stack adjustment, leaving the
///   stack pointer 12 bytes higher than on entry plus the callee pop; that
///   path is excluded from the stage-A contract and proven separately.
/// * id 8/9/10 (thiscall/1, thiscall/2, cdecl/3): the fallback path when
///   no callback is set: construct an event object on the frame, run it
///   through a first-megabyte helper, then a script-data helper.
/// * id 3 (`0x61cf90`, thiscall/0): endpoint initialiser over frame
///   scratch, followed by a 31-step zeroing loop over `[+0x868, +0x998)`.
/// * id 4 (`0x6ce9c0`, thiscall/1): address-record lookup; a record
///   whose flag word is zero or whose handle is 0/-1 is replaced by the
///   skip key's record.
/// * id 5 (`0x622d90`, thiscall/1): session find by id pair; a miss yields
///   handle -1.
/// * id 6 (`0x61d3b0`, thiscall/9): channel initialiser; its answer is
///   left in eax (the walk, when it runs, overwrites eax with scan and
///   probe residue, so the final value is meaningful only on no-walk
///   paths; the sole caller ignores it and the contract compares no
///   return channel).
/// * id 11/12/13 (thiscall/3, thiscall/1-noclean, thiscall/5) plus id 10:
///   the table walk over `this+0x2e24` (`this+0x2ea4` entries, compared
///   SIGNED with `jle`/`jl`): for each entry that is not the skip key,
///   scan its pointer array at `+0x68` (`+0x6c` entries) for the first
///   pointer whose target has bit 0 of byte `+0x80` set, probe it, and
///   when the probe sizes fit (unsigned `ja` against `0x394`) forward the
///   data. Entries equal to the skip key are skipped.
/// * id 7: the CRT security-cookie check (preserves registers).
///
/// Several `lea` displacements are push-shifted (computed while earlier
/// arguments sit on the stack): the word tested and passed to the channel
/// call is the zero written to `+0x5c8`, its object is the endpoint at
/// `+0x5d0`, its record argument is the record at `+0x18`, the event
/// object is at `+0xa68`, and the probe object is the endpoint too. The
/// cookie computation itself is frame scratch feeding only the stubbed
/// check and is not modelled.
///
/// Original: 0x006228E0 (thiscall, ecx = this, three stack words).
lf_checker_rt::export!(thiscall, rw_006228E0(this: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    unsafe {
        const CB_KEY: u32 = 1;
        const CB_NOTIFY: u32 = 2;
        const CB_ENDPOINT: u32 = 3;
        const CB_ADDR: u32 = 4;
        const CB_FIND: u32 = 5;
        const CB_CHANNEL: u32 = 6;
        const CB_COOKIE: u32 = 7;
        const CB_EVENT_CTOR: u32 = 8;
        const CB_EVENT_RUN: u32 = 9;
        const CB_SCRIPT_DATA: u32 = 10;
        const CB_PROBE: u32 = 11;
        const CB_SIZEOF: u32 = 12;
        const CB_FORWARD: u32 = 13;

        const KEY0_OFF: u32 = 0x38;
        const KEY1_OFF: u32 = 0x3c;
        const IDPAIR_OFF: u32 = 0x40;
        const CB_ARG_OFF: u32 = 0x04;
        const CB_TGT_OFF: u32 = 0x08;
        const ADDR_OBJ_OFF: u32 = 0x24;
        const CHAN_A0_OFF: u32 = 0x540;
        const CHAN_A1_OFF: u32 = 0x544;
        const TABLE_OFF: u32 = 0x2e24;
        const TABLE_COUNT_OFF: u32 = 0x2ea4;
        const EVENT_RUN_OBJ_OFF: u32 = 0x32c4;
        const FORWARD_OBJ_OFF: u32 = 0xc6c;
        const FIND_HANDLE_OFF: u32 = 0x70;
        const MISS_HANDLE: u32 = 0xFFFF_FFFF;
        const FORWARD_LIMIT: u32 = 0x394;

        const F_SAVED_KEY: u32 = 0x10;
        const F_LOOP_I: u32 = 0x14;
        const F_REC0: u32 = 0x18;
        const F_REC1W: u32 = 0x1c;
        const F_REC2: u32 = 0x20;
        const F_REC3W: u32 = 0x24;
        const F_ENTRY0: u32 = 0x28;
        const F_SAVED_ENTRY: u32 = 0x2c;
        const F_SCRATCH: u32 = 0x30;
        const F_CB_BUF: u32 = 0x3c8;
        const F_ZERO_SLOT: u32 = 0x5c8;
        const F_ENDPOINT: u32 = 0x5d0;
        const F_ZERO_BASE: u32 = 0x871;
        const F_ZERO_ITERS: i32 = 31;
        const F_EVENT: u32 = 0xa68;
        const F_EVT_TAIL: u32 = 0xae8;
        const F_EVT_MARK: u32 = 0xcec;

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
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let mut frame = [0u8; 0xe00];
        let fb = frame.as_mut_ptr() as u32;

        let k0 = rd32(arg1.wrapping_add(KEY0_OFF));
        let k1 = rd32(arg1.wrapping_add(KEY1_OFF));
        let first = lf_checker_rt::callee_thiscall!(CB_KEY, u32, this, k0, k1);
        wr32(fb.wrapping_add(F_SAVED_KEY), first);
        wr32(fb.wrapping_add(F_ZERO_SLOT), 0);
        let mut esi: u32;
        let tgt = rd32(this.wrapping_add(CB_TGT_OFF));
        if tgt == 0 {
            // Fallback: no callback set; build and run an event instead.
            let _: u32 =
                lf_checker_rt::callee_thiscall!(CB_EVENT_CTOR, u32, fb.wrapping_add(F_EVENT), arg1);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CB_EVENT_RUN,
                u32,
                this.wrapping_add(EVENT_RUN_OBJ_OFF),
                this,
                fb.wrapping_add(F_EVENT)
            );
            let mark = rd32(fb.wrapping_add(F_EVT_MARK));
            let _: u32 = lf_checker_rt::callee_cdecl!(
                CB_SCRIPT_DATA,
                u32,
                fb.wrapping_add(F_CB_BUF),
                fb.wrapping_add(F_EVT_TAIL),
                mark
            );
            // Both displacements are push-shifted (the helper's three
            // arguments are still on the stack): the mark lands on the zero
            // slot, and esi reloads the saved first-lookup answer.
            wr32(fb.wrapping_add(F_ZERO_SLOT), mark);
            esi = rd32(fb.wrapping_add(F_SAVED_KEY));
        } else {
            let cb_arg = rd32(this.wrapping_add(CB_ARG_OFF));
            // Both original call sites make the same call; site 2 (taken
            // when cb_arg is 0) additionally shifts the stack pointer, a
            // path the stage-A contract steers away from and proves
            // separately with the stack-pointer check off.
            // Argument order is the callee's (idx0 = last pushed): the
            // original pushes the frame buffer, then arg1, then this.
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CB_NOTIFY,
                u32,
                cb_arg,
                this,
                arg1,
                fb.wrapping_add(F_CB_BUF)
            );
            esi = first;
        }

        let _: u32 =
            lf_checker_rt::callee_thiscall!(CB_ENDPOINT, u32, fb.wrapping_add(F_ENDPOINT));
        let mut e = F_ZERO_BASE;
        let mut cc = F_ZERO_ITERS;
        loop {
            cc -= 1;
            wr64(fb.wrapping_add(e).wrapping_sub(9), 0);
            wr64(fb.wrapping_add(e).wrapping_sub(1), 0);
            wr16(fb.wrapping_add(e).wrapping_sub(1), 0);
            e = e.wrapping_add(0x10);
            if cc < 0 {
                break;
            }
        }

        let r2 = lf_checker_rt::callee_thiscall!(CB_KEY, u32, this, k0, k1);
        let key: u32 = if r2 != 0 { rd32(r2) } else { MISS_HANDLE };
        let s = lf_checker_rt::callee_thiscall!(CB_ADDR, u32, rd32(this.wrapping_add(ADDR_OBJ_OFF)), key);
        wr32(fb.wrapping_add(F_REC0), rd32(s));
        wr16(fb.wrapping_add(F_REC1W), rd16(s.wrapping_add(4)));
        wr32(fb.wrapping_add(F_REC2), rd32(s.wrapping_add(8)));
        wr16(fb.wrapping_add(F_REC3W), rd16(s.wrapping_add(0xc)));
        let flag = rd16(fb.wrapping_add(F_REC3W));
        let handle = rd32(fb.wrapping_add(F_REC2));
        if flag == 0 || handle == MISS_HANDLE || handle == 0 {
            wr32(fb.wrapping_add(F_REC0), rd32(esi.wrapping_add(0x58)));
            wr16(fb.wrapping_add(F_REC1W), rd16(esi.wrapping_add(0x5c)));
            wr32(fb.wrapping_add(F_REC2), rd32(esi.wrapping_add(0x60)));
            wr16(fb.wrapping_add(F_REC3W), rd16(esi.wrapping_add(0x64)));
        }

        let f =
            lf_checker_rt::callee_thiscall!(CB_FIND, u32, this, arg1.wrapping_add(IDPAIR_OFF));
        let ed: u32 = if f != 0 {
            rd32(f.wrapping_add(FIND_HANDLE_OFF))
        } else {
            MISS_HANDLE
        };
        // The lea/load displacements are push-shifted: the word tested and
        // passed here is the zero written to +0x5c8 above, the object is the
        // endpoint at +0x5d0, and the record argument is the record at +0x18.
        let a2v: u32 = rd32(fb.wrapping_add(F_ZERO_SLOT));
        let a3ptr = if a2v == 0 { 0 } else { fb.wrapping_add(F_CB_BUF) };
        // idx0 is the last pushed word: +0x540, +0x544, arg1, record,
        // handle, frame-or-null, tested word, arg2, arg3.
        let ans: u32 = lf_checker_rt::callee_thiscall!(
            CB_CHANNEL,
            u32,
            fb.wrapping_add(F_ENDPOINT),
            rd32(this.wrapping_add(CHAN_A0_OFF)),
            rd32(this.wrapping_add(CHAN_A1_OFF)),
            arg1,
            fb.wrapping_add(F_REC0),
            ed,
            a3ptr,
            a2v,
            arg2,
            arg3
        );

        wr32(fb.wrapping_add(F_LOOP_I), 0);
        // Signed: the entry test is jle and the loop test is jl.
        let total = rd32(this.wrapping_add(TABLE_COUNT_OFF)) as i32;
        if total > 0 {
            let mut i = 0i32;
            while i < total {
                let ent = rd32(
                    this
                        .wrapping_add(TABLE_OFF)
                        .wrapping_add((i as u32).wrapping_mul(4)),
                );
                if ent != esi {
                    let cnt = rd32(ent.wrapping_add(0x6c)) as i32;
                    if cnt > 0 {
                        let mut k = 0i32;
                        let mut hit = 0u32;
                        let mut found = false;
                        while k < cnt {
                            let q = rd32(
                                ent.wrapping_add(0x68).wrapping_add((k as u32).wrapping_mul(4)),
                            );
                            if rd8(q.wrapping_add(0x80)) & 1 != 0 {
                                hit = ent.wrapping_add(0x68).wrapping_add((k as u32).wrapping_mul(4));
                                found = true;
                                break;
                            }
                            k += 1;
                        }
                        if found {
                            wr32(fb.wrapping_add(F_SAVED_ENTRY), rd32(ent));
                            let al: u32 = lf_checker_rt::callee_thiscall!(
                                CB_PROBE,
                                u32,
                                fb.wrapping_add(F_ENDPOINT),
                                fb.wrapping_add(F_EVENT),
                                hit,
                                fb.wrapping_add(F_ENTRY0)
                            );
                            if al & 0xff != 0 && rd32(this.wrapping_add(FORWARD_OBJ_OFF)) != 0 {
                                let r: u32 = lf_checker_rt::callee_thiscall!(
                                    CB_SIZEOF,
                                    u32,
                                    fb.wrapping_add(F_SCRATCH),
                                    0
                                );
                                if r != 0 {
                                    let base = rd32(fb.wrapping_add(F_ENTRY0));
                                    let adj = r.wrapping_add(base);
                                    // Unsigned: the limit test is ja.
                                    if adj <= FORWARD_LIMIT {
                                        let z: u32 = lf_checker_rt::callee_cdecl!(
                                            CB_SCRIPT_DATA,
                                            u32,
                                            fb.wrapping_add(F_SCRATCH).wrapping_add(r),
                                            fb.wrapping_add(F_EVENT),
                                            base
                                        );
                                        if z != 0 {
                                            let _: u32 = lf_checker_rt::callee_thiscall!(
                                                CB_FORWARD,
                                                u32,
                                                this.wrapping_add(FORWARD_OBJ_OFF),
                                                rd32(fb.wrapping_add(F_SAVED_ENTRY)),
                                                fb.wrapping_add(F_SCRATCH),
                                                adj,
                                                0,
                                                0
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                esi = rd32(fb.wrapping_add(F_SAVED_KEY));
                i += 1;
            }
        }

        let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
        ans
    }
});
