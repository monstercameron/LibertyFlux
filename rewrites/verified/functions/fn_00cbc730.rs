// original: 0x00CBC730 ped_task_update_route_state (proposed)

/// Advance a route-following task one step, collecting route points.
///
/// `this` is the task (state word at `+0x20`, context at `+0x44`, flag
/// word at `+0xB0`, stage at `+0x3C`, blend at `+0x40`, point list at
/// `+0x64`, retry counter at `+0x24`); the stack argument is passed
/// through to one helper. State 0 means idle, 1 collected, 2 placed or
/// failed, 3 finished. Returns a leftover in `eax` that callers ignore
/// but the proof compares: 0 on the idle exit, the helper answer on the
/// two middle exits, `stage + 1` on the finished exits.
///
/// After a context validity check and clearing flag bit 3, a collector
/// helper fills four out-slots: a point count, two table pointers and a
/// flag word (its pre-call slot contents are zeroes except one masked
/// garbage word the helper overwrites before any read). A nonzero answer
/// or a count of at most 1 (signed) takes the fail path: state 2, then a
/// signed stage of at least 4 bumps the retry counter with wraparound
/// from 3 to 0 and clears blend and stage, while a lower stage arms the
/// finish flag. Otherwise, when
/// the flag word's bit 0 is set, a placement helper runs with the stack
/// argument, the count and the first table; a nonzero answer with a stage
/// below 4 (signed) stores state 2 and arms the flag, jumping over the
/// collection loop. The loop appends points 1 through count - 1 (signed):
/// each table word into the task's small array and each 16-byte record
/// into the list, stopping early once eight are stored. Afterwards the
/// last record is copied to the result pose, flag bit 8 is set to the
/// flag word's bit 3, two words are cleared, and blend and stage are
/// cleared too.
///
/// A teardown helper then runs and the context is cleared. A set finish
/// flag exits with the teardown answer; otherwise state becomes 3 and the
/// stage decides: stage 0 stores a constant blend and stage 1; a positive
/// blend is negated and the stage incremented; otherwise a float helper
/// maps the blend and the result is scaled by `(|blend| + C7) * -1.0`
/// with the stage incremented. Float gates are `comiss` + `jbe`
/// (strictly greater continues, NaN takes the exit). All operation orders
/// are the original's. The one dead branch (a second capacity check the
/// first already guarantees) is mirrored.
///
/// Original: 0x00CBC730 (thiscall, one stack word; `eax` holds the leftover
/// described above).
lf_checker_rt::export!(thiscall, rw_00CBC730(this: u32, arg: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x44;
        const STATE: u32 = 0x20;
        const FLAGS: u32 = 0xb0;
        const PLACED_BIT: u32 = 0x08;
        const STAGE: u32 = 0x3c;
        const BLEND: u32 = 0x40;
        const LIST: u32 = 0x64;
        const RETRY: u32 = 0x24;
        const SMALL: u32 = 0x68;
        const RES_A: u32 = 0x60;
        const RES_B: u32 = 0x50;
        const RES_X: u32 = 0x90;
        const RES_Y: u32 = 0x94;
        const RES_Z: u32 = 0x98;
        const RES_W: u32 = 0x9c;
        const STAGE_DONE: u32 = 4;
        const CAP: i32 = 8;
        const BLEND_CONST: u32 = 0x3ec90fdb;
        const MGR_VA: u32 = 0x0179d114;
        const BIAS_VA: u32 = 0x00e9b9cc;
        const SCALE_VA: u32 = 0x00fe8d94;
        const ABS_MASK: u32 = 0x7fffffff;
        const SIGN_BIT: u32 = 0x80000000;
        const VALID_CALLEE: u32 = 1;
        const COLLECT_CALLEE: u32 = 2;
        const PLACE_CALLEE: u32 = 3;
        const ALLOC_CALLEE: u32 = 4;
        const TEARDOWN_CALLEE: u32 = 5;
        const MAP_CALLEE: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn live_f32(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }

        let ctx = rd32(this.wrapping_add(CTX));
        wr32(this.wrapping_add(STATE), 0);
        if ctx == 0 {
            return 0;
        }
        let ok: u32 = lf_checker_rt::callee_cdecl!(VALID_CALLEE, u32, ctx);
        if ok & 0xff == 0 {
            return ok;
        }
        wr32(
            this.wrapping_add(FLAGS),
            rd32(this.wrapping_add(FLAGS)) & !PLACED_BIT,
        );
        let mut slot_c = 0u32;
        let mut slot_10 = 0u32;
        let mut slot_14 = 0u32;
        let mut slot_20 = 0u32;
        let mut flag = 1u8;
        let code: u32 = lf_checker_rt::callee_cdecl!(
            COLLECT_CALLEE,
            u32,
            ctx,
            &mut slot_c as *mut u32 as u32,
            &mut slot_10 as *mut u32 as u32,
            &mut slot_14 as *mut u32 as u32,
            &mut slot_20 as *mut u32 as u32
        );
        if code != 0 || (slot_c as i32) <= 1 {
            wr32(this.wrapping_add(STATE), 2);
            if (rd32(this.wrapping_add(STAGE)) as i32) >= STAGE_DONE as i32 {
                let n = rd32(this.wrapping_add(RETRY)).wrapping_add(1);
                wr32(this.wrapping_add(RETRY), if n == 3 { 0 } else { n });
                // Falls through to the clearing below.
                wr32(this.wrapping_add(BLEND), 0);
                wr32(this.wrapping_add(STAGE), 0);
            } else {
                flag = 0;
            }
        } else {
            let mut jumped = false;
            if slot_20 & 1 != 0 {
                let placed: u32 = lf_checker_rt::callee_thiscall!(
                    PLACE_CALLEE,
                    u32,
                    this,
                    arg,
                    slot_c,
                    slot_10
                );
                if placed & 0xff != 0 && (rd32(this.wrapping_add(STAGE)) as i32) < STAGE_DONE as i32
                {
                    wr32(this.wrapping_add(STATE), 2);
                    flag = 0;
                    jumped = true;
                }
            }
            if !jumped {
                let g = rd32(this.wrapping_add(LIST));
                let list = if g == 0 {
                    let mgr = rd32(lf_checker_rt::relocated(MGR_VA));
                    let p: u32 =
                        lf_checker_rt::callee_thiscall!(ALLOC_CALLEE, u32, mgr);
                    // A null answer faults in the original (immediate
                    // null write); the contract never scripts one.
                    wr32(p, 0);
                    wr32(this.wrapping_add(LIST), p);
                    p
                } else {
                    g
                };
                wr32(list, 0);
                let mut edx = 1u32;
                while (edx as i32) < (slot_c as i32) {
                    let count = rd32(list);
                    if (count as i32) >= CAP {
                        break;
                    }
                    wr32(
                        this.wrapping_add(SMALL).wrapping_add(count.wrapping_mul(4)),
                        rd32(slot_14.wrapping_add(edx.wrapping_mul(4))),
                    );
                    // Dead when reached (the break above already left on a
                    // full list), mirrored anyway.
                    if (count as i32) < CAP {
                        let dst = list.wrapping_add(count.wrapping_add(1).wrapping_mul(16));
                        // The index starts at 16 and steps by 16 per point,
                        // not by one: `(an instruction of the original)` runs once, then
                        // `(an instruction of the original)` each round.
                        let base = slot_10.wrapping_add(edx.wrapping_mul(16));
                        wr32(dst, rd32(base));
                        wrf(dst.wrapping_add(4), rdf(base.wrapping_add(4)));
                        wrf(dst.wrapping_add(8), rdf(base.wrapping_add(8)));
                        wr32(dst.wrapping_add(12), rd32(base.wrapping_add(12)));
                    }
                    wr32(list, count.wrapping_add(1));
                    edx = edx.wrapping_add(1);
                }
                let count = rd32(list);
                let src = list.wrapping_add(count.wrapping_mul(16));
                wr32(this.wrapping_add(RES_X), rd32(src));
                wrf(this.wrapping_add(RES_Y), rdf(src.wrapping_add(4)));
                wrf(this.wrapping_add(RES_Z), rdf(src.wrapping_add(8)));
                wr32(this.wrapping_add(RES_W), rd32(src.wrapping_add(12)));
                let b0 = rd32(this.wrapping_add(FLAGS));
                wr32(this.wrapping_add(STATE), 1);
                wr32(
                    this.wrapping_add(FLAGS),
                    b0 ^ ((slot_20.wrapping_shl(5) ^ b0) & 0x100),
                );
                wr32(this.wrapping_add(RES_A), 0);
                wr8(this.wrapping_add(RES_B), 0);
                // Falls through to the clearing below, like the counter path.
                wr32(this.wrapping_add(BLEND), 0);
                wr32(this.wrapping_add(STAGE), 0);
            }
        }
        let done: u32 = lf_checker_rt::callee_cdecl!(TEARDOWN_CALLEE, u32, ctx);
        wr32(this.wrapping_add(CTX), 0);
        if flag != 0 {
            return done;
        }
        let stage = rd32(this.wrapping_add(STAGE));
        wr32(this.wrapping_add(STATE), 3);
        if stage == 0 {
            wr32(this.wrapping_add(BLEND), BLEND_CONST);
            wr32(this.wrapping_add(STAGE), 1);
            return 1;
        }
        let blend = rdf(this.wrapping_add(BLEND));
        if blend > 0.0 {
            wrf(
                this.wrapping_add(BLEND),
                f32::from_bits(blend.to_bits() ^ SIGN_BIT),
            );
            wr32(this.wrapping_add(STAGE), stage.wrapping_add(1));
            return stage.wrapping_add(1);
        }
        let mapped: f32 = lf_checker_rt::callee_cdecl!(MAP_CALLEE, f32, blend.to_bits());
        // The scale term reads the spilled blend, not the flag word: the
        // blend is stored to [esp+0x18] before the call and reloaded after.
        let scaled = mul(
            mul(
                mapped,
                add(
                    f32::from_bits(blend.to_bits() & ABS_MASK),
                    live_f32(BIAS_VA),
                ),
            ),
            live_f32(SCALE_VA),
        );
        wr32(this.wrapping_add(STAGE), stage.wrapping_add(1));
        wrf(this.wrapping_add(BLEND), scaled);
        stage.wrapping_add(1)
    }
});
