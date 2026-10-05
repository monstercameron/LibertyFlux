// original: 0x00947330 stream_driver
/// Streaming driver: entry gates plus a four-way float state machine.
///
/// Takes one opaque word (forwarded to two callees on some paths) and
/// returns nothing. Behaviour in order:
/// - Entry gates: resolve the driver object through a scripted
///   allocator fed from a data word; a null object exits through shared
///   cleanup. A scripted readiness check on the object must report
///   non-zero in its low byte. Two data mode words must both differ
///   from 4, an inhibit byte must be clear, and the owner slot must be
///   clear or already hold this object. Any failure runs cleanup:
///   release each of the two handle slots when non-null through one
///   scripted release helper, then clear the state byte and the owner.
/// - The state byte then selects one of four paths. A slot index byte
///   read from the object picks one record out of a 0xBD0-stride table
///   inside the object, and a cursor word from that record drives the
///   range checks below (all range arithmetic is signed 32-bit).
/// - Path D (any other state): scan the selected record. A negative
///   cursor or a cleared live flag fails. A two-rung ladder compares
///   span starts against the cursor with threshold 0xBB8 and derives a
///   positive extent plus a floor value, else fails. A neighbour-slot
///   marker of 3 or 4, or a set answer from a scripted probe of the
///   constant 0.8, routes to a scripted combiner fed with the flag,
///   a rung marker (0 when the first rung took, 1 for the second) and
///   the extent; a zero combiner answer
///   or a null handle slot fails, else the owner is set to the object
///   and the limit word becomes extent minus answer plus floor with
///   state 1. Otherwise two scripted setup calls run (each taking the
///   setup argument, one slot address, and fixed flags); a null first
///   slot fails, a null second slot releases the first and fails, else
///   the limit becomes floor plus extent with the owner set and
///   state 1.
/// - Path B (state 2): compare the cursor against the limit word plus
///   or minus 0x1F4. Below the window continues as path C with state
///   still 2; above it releases both slots unconditionally and clears
///   state and owner; inside it runs a scripted touch helper on each
///   slot, a scripted notify on the object with the flag, clears the
///   owner, sets state 3, then continues as path C.
/// - Path C (state 3, shared with path B): float ladder over the two
///   handle slots. When the first slot is set, a base data float goes
///   through a scripted filter, is kept only when strictly above a
///   small floor constant (NaN takes the floor), goes through a
///   scripted adjuster, adds a data addend, and is emitted through the
///   first slot. When the second slot is null the path ends: through
///   the failure exit if the first slot ran, else by clearing state
///   and owner. Otherwise one minus the base goes through the second
///   filter/floor step; a data pointer either selects a fixed -100.0
///   output (when null) or supplies a three-float vector to a scripted
///   vector helper plus a scripted query, and the output is the
///   query answer plus the second adjuster answer plus the addend;
///   the output is emitted through the second slot and the path ends
///   through the failure exit. (The vector words only feed the vector
///   helper call; the second filter answer only feeds the adjuster.)
/// - Path A (state 1): when the cursor exceeds the limit plus 0x1F4,
///   release both slots and clear state and owner. Else run a scripted
///   check helper on the first slot with a data word and flag 1: an
///   answer of 1 continues to the same check on the second slot (1
///   sets state 2 and returns), an answer of 2 releases both slots and
///   clears state and owner, any other answer fails; the second check
///   resolves the same way.
/// - The failure exit keeps the current state when non-zero (plain
///   return) and clears the owner when the state is zero.
export!(cdecl, rw_00947330(flag: u32) -> u32 {
    unsafe {
        const ALLOC_ID: u32 = 1;
        const READY_ID: u32 = 2;
        const RELEASE_ID: u32 = 3;
        const PROBE_ID: u32 = 4;
        const SETUP_A_ID: u32 = 5;
        const SETUP_B_ID: u32 = 6;
        const COMBINE_ID: u32 = 7;
        const TOUCH_A_ID: u32 = 8;
        const TOUCH_B_ID: u32 = 9;
        const NOTIFY_ID: u32 = 10;
        const FILT_A_ID: u32 = 11;
        const FILT_B_ID: u32 = 12;
        const ADJ_A_ID: u32 = 13;
        const ADJ_B_ID: u32 = 14;
        const EMIT_A_ID: u32 = 15;
        const EMIT_B_ID: u32 = 16;
        const VECUSE_ID: u32 = 17;
        const QUERY_ID: u32 = 18;
        const CHK_A_ID: u32 = 19;
        const CHK_B_ID: u32 = 20;

        const ARG_GLOB: u32 = 0x1284644;
        const MODE_A_GLOB: u32 = 0x128463C;
        const MODE_B_GLOB: u32 = 0x1284640;
        const INHIBIT_GLOB: u32 = 0x11D7629;
        const OWNER_GLOB: u32 = 0x11D7624;
        const SLOT_A_GLOB: u32 = 0x11D74FC;
        const SLOT_B_GLOB: u32 = 0x11D7500;
        const STATE_GLOB: u32 = 0x11D74F3;
        const LIMIT_GLOB: u32 = 0x11D751C;
        const AUX_GLOB: u32 = 0x11D7630;
        const SETUP_ARG_GLOB: u32 = 0x1037638;
        const ESI_SRC_GLOB: u32 = 0x12845FC;
        const F_BASE_GLOB: u32 = 0x12845F0;
        const F_ADD_GLOB: u32 = 0x12845F4;
        const FLOOR_GLOB: u32 = 0xFE8670;
        const ONE_GLOB: u32 = 0xFE88E8;
        const NEG100_GLOB: u32 = 0xFE8DF8;
        const FILT_THIS: u32 = 0x11D7650;
        const SETUP_THIS: u32 = 0x12845D0;

        const INDEX_OFF: u32 = 0x1917;
        const RECORD_STRIDE: u32 = 0xBD0;
        const CURSOR_OFF: u32 = 0x990;
        const LIVE_OFF: u32 = 0xBCB;
        const NEIGHBOUR_OFF: u32 = 0xBC0;
        const SPAN0_OFF: u32 = 0xAA4;
        const SPAN1_OFF: u32 = 0xAA8;
        const SPAN2_OFF: u32 = 0xAAC;
        const SPAN3_OFF: u32 = 0xAB0;
        const LADDER_LIMIT: i32 = 0xBB8;
        const WINDOW: i32 = 0x1F4;
        const PROBE_BITS: u32 = 0x3F4CCCCD;
        const QUERY_THIS_OFF: u32 = 0x210;
        const VEC_LINK_OFF: u32 = 0x20;
        const VEC_X_OFF: u32 = 0x30;
        const VEC_Y_OFF: u32 = 0x34;
        const VEC_Z_OFF: u32 = 0x38;

        let arg = *(global::<u32>(ARG_GLOB));
        let edi: u32 = callee_cdecl!(ALLOC_ID, u32, arg,);
        if edi == 0 {
            cleanup();
            return 0;
        }
        let ready: u32 = callee_thiscall!(READY_ID, u32, edi,);
        if (ready as u8) == 0 {
            cleanup();
            return 0;
        }
        if *(global::<u32>(MODE_A_GLOB)) == 4 {
            cleanup();
            return 0;
        }
        if *(global::<u32>(MODE_B_GLOB)) == 4 {
            cleanup();
            return 0;
        }
        if *(global::<u8>(INHIBIT_GLOB)) != 0 {
            cleanup();
            return 0;
        }
        let owner = *(global::<u32>(OWNER_GLOB));
        if owner != 0 && owner != edi {
            cleanup();
            return 0;
        }
        let idx = *((edi.wrapping_add(INDEX_OFF)) as *const u8) as u32;
        let record = edi.wrapping_add(idx.wrapping_mul(RECORD_STRIDE));
        let edx = *((record.wrapping_add(CURSOR_OFF)) as *const u32);
        match *(global::<u8>(STATE_GLOB)) {
            1 => path_a(edx),
            2 => path_b(edi, edx, flag),
            3 => path_c(),
            _ => path_d(edi, edx, idx, flag),
        }
        return 0;

        unsafe fn cleanup() {
            let a = *(global::<u32>(SLOT_A_GLOB));
            if a != 0 {
                let _: u32 = callee_thiscall!(RELEASE_ID, u32, a, 0);
            }
            let b = *(global::<u32>(SLOT_B_GLOB));
            if b != 0 {
                let _: u32 = callee_thiscall!(RELEASE_ID, u32, b, 0);
            }
            *(global::<u8>(STATE_GLOB)) = 0;
            *(global::<u32>(OWNER_GLOB)) = 0;
        }

        unsafe fn fail() {
            if *(global::<u8>(STATE_GLOB)) != 0 {
                return;
            }
            *(global::<u32>(OWNER_GLOB)) = 0;
        }

        unsafe fn release_both_raw() {
            let a = *(global::<u32>(SLOT_A_GLOB));
            let _: u32 = callee_thiscall!(RELEASE_ID, u32, a, 0);
            let b = *(global::<u32>(SLOT_B_GLOB));
            let _: u32 = callee_thiscall!(RELEASE_ID, u32, b, 0);
            *(global::<u8>(STATE_GLOB)) = 0;
            *(global::<u32>(OWNER_GLOB)) = 0;
        }

        unsafe fn path_d(edi: u32, edx: u32, idx: u32, flag: u32) {
            if (edx as i32) < 0 {
                fail();
                return;
            }
            let record = edi.wrapping_add(idx.wrapping_mul(RECORD_STRIDE));
            if *((record.wrapping_add(LIVE_OFF)) as *const u8) == 0 {
                fail();
                return;
            }
            let w0 = *((record.wrapping_add(SPAN0_OFF)) as *const u32);
            let w1 = *((record.wrapping_add(SPAN1_OFF)) as *const u32);
            let w2 = *((record.wrapping_add(SPAN2_OFF)) as *const u32);
            let w3 = *((record.wrapping_add(SPAN3_OFF)) as *const u32);
            // Two-rung ladder: the first rung whose start clears the
            // cursor by more than the limit supplies extent, floor and a
            // rung marker (0 for the first rung, 1 for the second);
            // anything else fails. Matches the original rung by rung.
            // The original keeps the marker as one byte of a scratch
            // word and passes the whole word on; the harness defines
            // unwritten scratch to zero, so the word equals the marker.
            let span1 = w1.wrapping_sub(w0);
            let take_first =
                (w0.wrapping_sub(edx) as i32) > LADDER_LIMIT && (span1 as i32) > 0;
            let (esi, floor, mode) = if take_first {
                (span1, w0, 0)
            } else {
                if (w2.wrapping_sub(edx) as i32) > LADDER_LIMIT {
                    let span2 = w3.wrapping_sub(w2);
                    if (span2 as i32) <= 0 {
                        fail();
                        return;
                    }
                    (span2, w2, 1)
                } else {
                    fail();
                    return;
                }
            };
            // Neighbour-slot marker: the slot after the index, modulo 2.
            let nidx = (idx.wrapping_add(1)) & 1;
            let marker = *((edi
                .wrapping_add(nidx.wrapping_mul(RECORD_STRIDE))
                .wrapping_add(NEIGHBOUR_OFF)) as *const u8);
            if marker == 4 || marker == 3 {
                branch_combine(edi, esi, floor, mode, flag);
                return;
            }
            let probe: u32 = callee_cdecl!(PROBE_ID, u32, PROBE_BITS,);
            if (probe as u8) != 0 {
                branch_combine(edi, esi, floor, mode, flag);
                return;
            }
            let setup_this = relocated(SETUP_THIS);
            let sarg = *(global::<u32>(SETUP_ARG_GLOB));
            let _: u32 = callee_thiscall!(
                SETUP_A_ID, u32, setup_this, sarg, relocated(SLOT_A_GLOB), 1, 0, 0,
                0xFFFF_FFFF, 0, 0,
            );
            if *(global::<u32>(SLOT_A_GLOB)) == 0 {
                fail();
                return;
            }
            let _: u32 = callee_thiscall!(
                SETUP_B_ID, u32, setup_this, sarg, relocated(SLOT_B_GLOB), 1, 0, 0,
                0xFFFF_FFFF, 0, 0,
            );
            if *(global::<u32>(SLOT_B_GLOB)) == 0 {
                let a = *(global::<u32>(SLOT_A_GLOB));
                let _: u32 = callee_thiscall!(RELEASE_ID, u32, a, 0);
                fail();
                return;
            }
            *(global::<u32>(LIMIT_GLOB)) = floor.wrapping_add(esi);
            *(global::<u32>(OWNER_GLOB)) = edi;
            *(global::<u8>(STATE_GLOB)) = 1;
        }

        unsafe fn branch_combine(edi: u32, esi: u32, floor: u32, mode: u32, flag: u32) {
            let answer: u32 = callee_thiscall!(COMBINE_ID, u32, edi, flag, mode, esi,);
            if answer == 0 {
                fail();
                return;
            }
            if *(global::<u32>(SLOT_A_GLOB)) == 0 {
                fail();
                return;
            }
            if *(global::<u32>(SLOT_B_GLOB)) == 0 {
                fail();
                return;
            }
            *(global::<u32>(OWNER_GLOB)) = edi;
            *(global::<u32>(LIMIT_GLOB)) = esi.wrapping_sub(answer).wrapping_add(floor);
            *(global::<u8>(STATE_GLOB)) = 1;
        }

        unsafe fn path_b(edi: u32, edx: u32, flag: u32) {
            let limit = *(global::<u32>(LIMIT_GLOB)) as i32;
            if (edx as i32) < limit.wrapping_sub(WINDOW) {
                path_c();
                return;
            }
            if (edx as i32) > limit.wrapping_add(WINDOW) {
                release_both_raw();
                return;
            }
            let a = *(global::<u32>(SLOT_A_GLOB));
            let _: u32 = callee_thiscall!(TOUCH_A_ID, u32, a,);
            let b = *(global::<u32>(SLOT_B_GLOB));
            let _: u32 = callee_thiscall!(TOUCH_B_ID, u32, b,);
            let _: u32 = callee_thiscall!(NOTIFY_ID, u32, edi, flag,);
            *(global::<u32>(OWNER_GLOB)) = 0;
            *(global::<u8>(STATE_GLOB)) = 3;
            path_c();
        }

        unsafe fn path_c() {
            let fbase = *(global::<f32>(F_BASE_GLOB));
            let mut first_ran = false;
            if *(global::<u32>(SLOT_A_GLOB)) != 0 {
                first_ran = true;
                let filtered: f32 =
                    callee_thiscall!(FILT_A_ID, f32, relocated(FILT_THIS), fbase.to_bits(),);
                let floor = *(global::<f32>(FLOOR_GLOB));
                // Scalar compare with an above-only taken edge: only a
                // strictly greater ordered value is kept, NaN takes
                // the floor.
                let kept = if filtered > floor { filtered } else { floor };
                let adjusted: f32 = callee_cdecl!(ADJ_A_ID, f32, kept.to_bits(),);
                let out = core::hint::black_box(adjusted + *(global::<f32>(F_ADD_GLOB)));
                let a = *(global::<u32>(SLOT_A_GLOB));
                let _: u32 = callee_thiscall!(EMIT_A_ID, u32, a, out.to_bits(),);
            }
            if *(global::<u32>(SLOT_B_GLOB)) == 0 {
                if first_ran {
                    fail();
                } else {
                    *(global::<u8>(STATE_GLOB)) = 0;
                    *(global::<u32>(OWNER_GLOB)) = 0;
                }
                return;
            }
            let one = *(global::<f32>(ONE_GLOB));
            let diff = core::hint::black_box(one - fbase);
            let filtered: f32 =
                callee_thiscall!(FILT_B_ID, f32, relocated(FILT_THIS), diff.to_bits(),);
            let floor = *(global::<f32>(FLOOR_GLOB));
            let kept = if filtered > floor { filtered } else { floor };
            let addend = *(global::<f32>(F_ADD_GLOB));
            let out = {
                let esi_src = *(global::<u32>(ESI_SRC_GLOB));
                if esi_src == 0 {
                    *(global::<f32>(NEG100_GLOB))
                } else {
                    let inner = *((esi_src.wrapping_add(VEC_LINK_OFF)) as *const u32);
                    let vx = *((inner.wrapping_add(VEC_X_OFF)) as *const f32);
                    let vy = *((inner.wrapping_add(VEC_Y_OFF)) as *const f32);
                    let vz = *((inner.wrapping_add(VEC_Z_OFF)) as *const f32);
                    let vec = [vx, vy, vz];
                    let b = *(global::<u32>(SLOT_B_GLOB));
                    let _: u32 =
                        callee_thiscall!(VECUSE_ID, u32, b, vec.as_ptr() as u32,);
                    let query: f32 =
                        callee_thiscall!(QUERY_ID, f32, esi_src.wrapping_add(QUERY_THIS_OFF),);
                    let adjusted: f32 = callee_cdecl!(ADJ_B_ID, f32, kept.to_bits(),);
                    // Pinned in emission order: the adjuster answer
                    // plus the addend first, the query answer last.
                    let mixed = core::hint::black_box(adjusted + addend);
                    core::hint::black_box(query + mixed)
                }
            };
            let b = *(global::<u32>(SLOT_B_GLOB));
            let _: u32 = callee_thiscall!(EMIT_B_ID, u32, b, out.to_bits(),);
            fail();
        }

        unsafe fn path_a(edx: u32) {
            let limit = *(global::<u32>(LIMIT_GLOB)) as i32;
            if (edx as i32) > limit.wrapping_add(WINDOW) {
                release_both_raw();
                return;
            }
            let aux = *(global::<u32>(AUX_GLOB));
            let first: u32 =
                callee_thiscall!(CHK_A_ID, u32, *(global::<u32>(SLOT_A_GLOB)), aux, 1,);
            if first == 1 {
                let second: u32 = callee_thiscall!(
                    CHK_B_ID,
                    u32,
                    *(global::<u32>(SLOT_B_GLOB)),
                    aux,
                    1,
                );
                if second == 1 {
                    *(global::<u8>(STATE_GLOB)) = 2;
                    return;
                }
                if second == 2 {
                    release_both_raw();
                } else {
                    fail();
                }
                return;
            }
            if first == 2 {
                release_both_raw();
            } else {
                fail();
            }
        }
    }
});
