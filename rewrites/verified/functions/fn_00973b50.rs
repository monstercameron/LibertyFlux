// original: 0x00973B50 audio_listener_update (proposed)

/// Refresh an audio listener object from the current voice list or search.
///
/// `this` points to a large audio object holding the listener position
/// (`POS`, three floats), the last search origin (`REF`), and slots for up
/// to sixteen voices (`KIND_ARR`, `GAIN_ARR`, `SRC_ARR`, `FILT_ARR`).
/// The function first publishes three mixer globals from their sources,
/// then dispatches on the voice-list object (`LIST_GV`):
///
/// * List path: when the list object exists, its flag byte is set and the
///   voice count (`COUNT_GV`) is positive and not `COUNT_SENTINEL`, the
///   thread-local voice row is copied into `SRC_ARR`, the gain array head
///   is set to exactly one, and the voice lookup (callee 3) resolves the
///   list entry. A null answer clears the voice slots; otherwise the
///   entry's filter triple is shaped through the filter callee (callee 4)
///   into `FILT_ARR`, the entry kind selects the stored kind
///   (`1/2/3 -> KIND_GROUP`, `4 -> KIND_SOLO`, `5` takes the override
///   global, anything else keeps the entry value) and the return value.
/// * Search path: otherwise the positional search (callee 5) runs with a
///   null list. A pending request object is polled (callee 6); while it
///   is busy the fetch (callee 7) fills the voice arrays through two
///   converter callees (8 and 9) and the filter callee, or, when there is
///   nothing to fetch, the voice arrays are cleared and the request is
///   dropped. With no request, a search is started (callee 10) only when
///   the listener moved more than `MOVE_EPS` squared from `REF`.
///
/// The tail resolves a pending effect request (callee 11): an answer of
/// exactly FLT_MAX stores `EFFECT_LEVEL`, a positive one stores its square
/// root, anything else stores zero. With no effect
/// pending, a new one starts (callee 12 falling back to callee 13 with the
/// thread-local effect row) unless freshly started. The function finally
/// pokes the mixer (callee 14) unless muted, shapes the mute flag through
/// the shaping filter (callee 15, whose second argument is the saved
/// count), publishes two status words and returns the entry-kind byte
/// (zero unless the list path resolved an entry).
///
/// Two frame words are indeterminate on some paths (the fetch count when
/// the fetch never runs is always scripted; the merge slot and the
/// fetch arrays when the fetch is skipped read as zero under the
/// worker's stack fill); the rewrite models them as zero.
///
/// Original: 0x00973B50 (thiscall, no stack arguments, eax is the
/// entry-kind byte; the trailing security-cookie call is intercepted as
/// callee 16 and preserves every register).
unsafe fn audio_listener_body(this: u32, full_fill: bool) -> u32 {
    unsafe {
        const POS: u32 = 0x580;
        const VOICE_LO: u32 = 0x5A0;
        const VOICE_HI: u32 = 0x5A4;
        const VOICE_FLAG: u32 = 0x5A8;
        const REQ: u32 = 0x1E70;
        const EFFECT_REQ: u32 = 0x1E74;
        const EFFECT_LEVEL_SLOT: u32 = 0x1E78;
        const EFFECT_TIME: u32 = 0x1E7C;
        const COUNT: u32 = 0x1E80;
        const KIND_ONE: u32 = 0x1E84;
        const KIND_ARR: u32 = 0x1E84;
        const GAIN_ARR: u32 = 0x1EC4;
        const SRC_ARR: u32 = 0x1F10;
        const FILT_ARR: u32 = 0x2010;
        const FILT_THIS: u32 = 0x2114;
        const FILT_STRIDE: u32 = 0x28;
        const REF: u32 = 0x20D0;
        const REF_STAMP: u32 = 0x20E0;
        const MUTE_FLAG: u32 = 0x3025;
        const MUTE_STATE: u32 = 0x3026;
        const MUTE_SHAPED_FLAG: u32 = 0x3027;
        const MUTE_SHAPED: u32 = 0x3028;
        const SHAPE_THIS: u32 = 0x302C;
        const STATUS_A: u32 = 0x3048;
        const STATUS_B: u32 = 0x304C;
        const KIND_GROUP: u32 = 0xF;
        const KIND_SOLO: u32 = 0xA;
        const COUNT_SENTINEL: i32 = 0x3F;
        const KIND_SCALE: u32 = 0x33;
        const MAX_VOICES: usize = 16;
        const TLS_INDEX_GV: u32 = 0x017A_BA14;
        const TLS_AUDIO_OFF: u32 = 0x70;
        const VOICE_TABLE: u32 = 0x0115_E420;
        const EFFECT_TABLE: u32 = 0x0115_DF20;
        const LIST_GV: u32 = 0x012B_41A4;
        const COUNT_GV: u32 = 0x0103_B110;
        const MIXER_A_GV: u32 = 0x0121_84A8;
        const MIXER_B_GV: u32 = 0x0121_84AC;
        const MIXER_C_GV: u32 = 0x0121_84A4;
        const MIXER_A: u32 = 0x0103_7A70;
        const MIXER_B: u32 = 0x0103_7A74;
        const MIXER_C: u32 = 0x0103_7A78;
        const SAVED_EDI_GV: u32 = 0x0116_18FC;
        const STATUS_A_GV: u32 = 0x017A_CC5C;
        const KIND_OVERRIDE_GV: u32 = 0x0117_6D44;
        const SEARCH_K_GV: u32 = 0x0103_7A08;
        const EFFECT_GATE_GV: u32 = 0x012D_DEAC;
        const EFFECT_TIME_GV: u32 = 0x0117_35B4;
        const MIXER_ARG_GV: u32 = 0x0128_4A5C;
        const MIXER_THIS: u32 = 0x0128_4A60;
        const MODE_GV: u32 = 0x011F_7060;
        const STATE_GV: u32 = 0x0120_88B4;
        const STATE_REF: u32 = 0x00F1_C040;
        const PHASE_GV: u32 = 0x0103_7720;
        const PHASE_CLEAR: u32 = 0x12;
        const EFFECT_TIMEOUT: u32 = 0x7D0;
        const STAMP_GRACE: u32 = 0xC8;
        const MOVE_EPS: f32 = f32::from_bits(0x3E80_0000); // 0.25
        const EFFECT_LEVEL: f32 = f32::from_bits(0x41C8_0000); // 25.0
        const MIXER_B_ARG: u32 = 0x3F19_999A; // 0.1
        const C_POLL: u32 = 1;
        const C_FREE: u32 = 2;
        const C_LOOKUP: u32 = 3;
        const C_FILTER: u32 = 4;
        const C_SEARCH: u32 = 5;
        const C_BUSY: u32 = 6;
        const C_FETCH: u32 = 7;
        const C_CONV_A: u32 = 8;
        const C_CONV_B: u32 = 9;
        const C_START: u32 = 10;
        const C_EFFECT: u32 = 11;
        const C_TRY_EFFECT: u32 = 12;
        const C_MAKE_EFFECT: u32 = 13;
        const C_MIXER: u32 = 14;
        const C_SHAPE: u32 = 15;
        const C_COOKIE: u32 = 16;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn gv_32(addr: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(addr)) }
        }
        #[inline(always)]
        unsafe fn gv_f(addr: u32) -> f32 {
            unsafe { f32::from_bits(gv_32(addr)) }
        }
        #[inline(always)]
        unsafe fn gv_wr(addr: u32, v: u32) {
            unsafe { wr32(lf_checker_rt::relocated(addr), v) }
        }

        // Mixer globals, in the original's write order.
        gv_wr(MIXER_A, gv_32(MIXER_A_GV));
        gv_wr(MIXER_C, gv_32(MIXER_C_GV));
        gv_wr(MIXER_B, gv_32(MIXER_B_GV));

        let edi_saved = gv_32(SAVED_EDI_GV);
        let pos0 = rdf(this.wrapping_add(POS));
        let pos1 = rdf(this.wrapping_add(POS).wrapping_add(4));
        let pos2 = rdf(this.wrapping_add(POS).wrapping_add(8));
        let mut f13: u8 = 0;
        let mut f12: u8 = 0;
        let mut e20: u32 = 0;
        let obj_a = gv_32(LIST_GV);
        let edi = gv_32(COUNT_GV) as i32;
        let mut status_a = gv_32(STATUS_A_GV);
        let mut status_b: u32 = 0;
        let mut merge_slot = 0.0f32;

        // Fresh-start gate shared by the clear, search and effect paths.
        let fresh = || {
            gv_32(MODE_GV) == 1
                || gv_32(STATE_GV) != gv_32(STATE_REF)
                || gv_32(PHASE_GV) == PHASE_CLEAR
        };

        let list_path = if obj_a == 0 {
            false
        } else {
            if edi > 0 && edi != COUNT_SENTINEL {
                f13 = 1;
                let active: u32 = lf_checker_rt::callee_thiscall!(C_POLL, u32, obj_a);
                if active as u8 != 0 {
                    e20 = 1;
                }
            }
            if rd8(obj_a.wrapping_add(0x73)) == 0 {
                false
            } else {
                edi > 0 && edi != COUNT_SENTINEL
            }
        };

        if list_path {
            if rd32(this.wrapping_add(REQ)) != 0 {
                let _: u32 =
                    lf_checker_rt::callee_cdecl!(C_FREE, u32, rd32(this.wrapping_add(REQ)));
                wr32(this.wrapping_add(REQ), 0);
            }
            let tls_index = gv_32(TLS_INDEX_GV);
            let tls = lf_checker_rt::tls_slot(tls_index as usize);
            let sel = rd32(tls.wrapping_add(TLS_AUDIO_OFF));
            let row = lf_checker_rt::relocated(VOICE_TABLE).wrapping_add(sel.wrapping_shl(6));
            wr32(this.wrapping_add(COUNT), 1);
            // Flag word shared with the lookup callee: low byte is the
            // out-flag, then f12, f13, then the list object's low byte,
            // which the callee's word-sized store preserves.
            let mut flagword: u32 =
                0x0001_0000 | ((obj_a & 0xFF).wrapping_shl(24));
            wr32(this.wrapping_add(SRC_ARR), rd32(row));
            wrf(this.wrapping_add(SRC_ARR).wrapping_add(4), rdf(row.wrapping_add(4)));
            wrf(this.wrapping_add(SRC_ARR).wrapping_add(8), rdf(row.wrapping_add(8)));
            wr32(this.wrapping_add(SRC_ARR).wrapping_add(12), rd32(row.wrapping_add(12)));
            wrf(this.wrapping_add(GAIN_ARR), 1.0);
            let entry: u32 = lf_checker_rt::callee_thiscall!(
                C_LOOKUP,
                u32,
                this,
                obj_a,
                edi as u32,
                core::ptr::addr_of_mut!(flagword) as u32
            );
            let f11 = (flagword & 0xFF) as u8;
            f12 = ((flagword >> 8) & 0xFF) as u8;
            f13 = ((flagword >> 16) & 0xFF) as u8;
            ((this.wrapping_add(VOICE_FLAG)) as *mut u16).write_unaligned(0);
            if entry != 0 {
                let off = (f11 as u32).wrapping_mul(KIND_SCALE);
                let base = entry.wrapping_add(off);
                wr32(this.wrapping_add(KIND_ONE), rd32(base.wrapping_add(0x18)));
                let mix_w = rdf(base.wrapping_add(0x14));
                let filt_in = rdf(base.wrapping_add(0x10));
                let mut filt_this = this.wrapping_add(FILT_THIS);
                let mut filt_out = this.wrapping_add(FILT_ARR);
                for _ in 0..3 {
                    let shaped: f32 = lf_checker_rt::callee_thiscall!(
                        C_FILTER,
                        f32,
                        filt_this,
                        filt_in.to_bits()
                    );
                    wrf(filt_out, mul(shaped, mix_w));
                    filt_this = filt_this.wrapping_add(FILT_STRIDE);
                    filt_out = filt_out.wrapping_add(4);
                }
                let kind = rd8(base.wrapping_add(0xF));
                if kind == 5 {
                    status_a = gv_32(KIND_OVERRIDE_GV);
                } else {
                    status_a = rd32(base.wrapping_add(0x1C));
                }
                status_b = rd8(base.wrapping_add(0x20)) as u32;
                ((this.wrapping_add(VOICE_FLAG)) as *mut u16).write_unaligned(0);
                if kind == 1 || kind == 2 || kind == 3 {
                    wr32(this.wrapping_add(KIND_ONE), KIND_GROUP);
                } else if kind == 4 {
                    wr32(this.wrapping_add(KIND_ONE), KIND_SOLO);
                }
                if kind == 2 || kind == 3 {
                    f12 = 1;
                }
            }
            wr32(this.wrapping_add(VOICE_LO), entry);
            wr32(this.wrapping_add(VOICE_HI), f11 as u32);
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_SEARCH, u32, this, obj_a, edi_saved);
        } else {
            // B-entry reloads edi from the saved count; every later use on
            // this path sees the saved value, not the voice count.
            let edi = edi_saved;
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_SEARCH, u32, this, 0, edi_saved);
            wr32(this.wrapping_add(VOICE_LO), 0);
            wr32(this.wrapping_add(VOICE_HI), 0);
            let req = rd32(this.wrapping_add(REQ));
            if req == 0 {
                let dx = sub(pos0, rdf(this.wrapping_add(REF)));
                let dy = sub(pos1, rdf(this.wrapping_add(REF).wrapping_add(4)));
                let dz = sub(pos2, rdf(this.wrapping_add(REF).wrapping_add(8)));
                let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                let mut run_search = fresh();
                if !run_search {
                    let stamp =
                        rd32(this.wrapping_add(REF_STAMP)).wrapping_add(STAMP_GRACE);
                    run_search = (edi as u32) > stamp;
                }
                if run_search && dist2 > MOVE_EPS {
                    let mut req_struct = [pos0.to_bits(), pos1.to_bits(), pos2.to_bits(), 0u32];
                    let mut aux = [0u32; 3];
                    let started: u32 = lf_checker_rt::callee_cdecl!(
                        C_START,
                        u32,
                        core::ptr::addr_of_mut!(req_struct) as u32,
                        gv_32(SEARCH_K_GV),
                        core::ptr::addr_of_mut!(aux) as u32,
                        1,
                        0
                    );
                    wr32(this.wrapping_add(REQ), started);
                    merge_slot = f32::from_bits(req_struct[3]);
                    if started != 0 {
                        wr32(this.wrapping_add(REF_STAMP), edi_saved);
                        wrf(this.wrapping_add(REF), pos0);
                        wrf(this.wrapping_add(REF).wrapping_add(4), pos1);
                        wrf(this.wrapping_add(REF).wrapping_add(8), pos2);
                        wrf(this.wrapping_add(REF).wrapping_add(12), merge_slot);
                    }
                }
            } else {
                let busy: u32 = lf_checker_rt::callee_cdecl!(C_BUSY, u32, req);
                if (busy as u8) != 0 {
                    let mut fetch_count: u32 = 0;
                    let mut kinds = [0u32; MAX_VOICES];
                    let mut gains = [0u32; MAX_VOICES];
                    let mut srcs = [0u32; MAX_VOICES * 4];
                    let fetched: u32 = lf_checker_rt::callee_cdecl!(
                        C_FETCH,
                        u32,
                        req,
                        core::ptr::addr_of_mut!(fetch_count) as u32,
                        core::ptr::addr_of_mut!(kinds) as u32,
                        core::ptr::addr_of_mut!(gains) as u32,
                        core::ptr::addr_of_mut!(srcs) as u32
                    );
                    let n = fetch_count as usize;
                    if fetched == 0 && n != 0 {
                        wr32(this.wrapping_add(COUNT), fetch_count);
                        let mut filt_out = this.wrapping_add(FILT_ARR);
                        for i in 0..n {
                            let k = kinds[i];
                            wr32(this.wrapping_add(KIND_ARR).wrapping_add((i as u32) * 4), k);
                            let c1: f32 =
                                lf_checker_rt::callee_cdecl!(C_CONV_A, f32, k);
                            let c2: f32 = lf_checker_rt::callee_cdecl!(
                                C_CONV_B,
                                f32,
                                rd32(
                                    this.wrapping_add(KIND_ARR).wrapping_add((i as u32) * 4)
                                )
                            );
                            let mut filt_this = this.wrapping_add(FILT_THIS);
                            for _ in 0..3 {
                                let shaped: f32 = lf_checker_rt::callee_thiscall!(
                                    C_FILTER,
                                    f32,
                                    filt_this,
                                    c1.to_bits()
                                );
                                wrf(filt_out, mul(shaped, c2));
                                filt_out = filt_out.wrapping_add(4);
                                filt_this = filt_this.wrapping_add(FILT_STRIDE);
                            }
                            wr32(
                                this.wrapping_add(GAIN_ARR).wrapping_add((i as u32) * 4),
                                gains[i],
                            );
                            let dst = this
                                .wrapping_add(SRC_ARR)
                                .wrapping_add((i as u32) * 16);
                            wr32(dst, srcs[4 * i]);
                            wr32(dst.wrapping_add(4), srcs[4 * i + 1]);
                            wr32(dst.wrapping_add(8), srcs[4 * i + 2]);
                            wr32(dst.wrapping_add(12), srcs[4 * i + 3]);
                        }
                    } else if fresh() {
                        let iters = if full_fill { 16u32 } else { 15u32 };
                        for t in 0..iters {
                            wr32(
                                this.wrapping_add(KIND_ARR).wrapping_add(t * 4),
                                0,
                            );
                            wr32(
                                this.wrapping_add(GAIN_ARR).wrapping_add(t * 4),
                                0,
                            );
                            wr32(
                                this.wrapping_add(SRC_ARR).wrapping_add(t * 16),
                                0,
                            );
                            wr32(
                                this.wrapping_add(SRC_ARR).wrapping_add(t * 16).wrapping_add(4),
                                0,
                            );
                            wr32(
                                this.wrapping_add(SRC_ARR).wrapping_add(t * 16).wrapping_add(8),
                                0,
                            );
                            wr32(
                                this.wrapping_add(FILT_ARR).wrapping_add(t * 12),
                                0,
                            );
                            wr32(
                                this.wrapping_add(FILT_ARR)
                                    .wrapping_add(t * 12)
                                    .wrapping_add(4),
                                0,
                            );
                            wr32(
                                this.wrapping_add(FILT_ARR)
                                    .wrapping_add(t * 12)
                                    .wrapping_add(8),
                                0,
                            );
                        }
                    }
                    wr32(this.wrapping_add(REQ), 0);
                    wr32(this.wrapping_add(REF_STAMP), edi_saved);
                    wrf(this.wrapping_add(REF), pos0);
                    wrf(this.wrapping_add(REF).wrapping_add(4), pos1);
                    wrf(this.wrapping_add(REF).wrapping_add(8), pos2);
                    wrf(this.wrapping_add(REF).wrapping_add(12), merge_slot);
                }
            }
        }

        let effect = rd32(this.wrapping_add(EFFECT_REQ));
        if effect != 0 {
            let busy: u32 = lf_checker_rt::callee_cdecl!(C_BUSY, u32, effect);
            if (busy as u8) != 0 {
                // Mirrors the original's frame: the callee fills the outer
                // words, the middle one keeps pos0. The level read below is
                // the first word: the read runs before the argument cleanup,
                // so it addresses one slot lower than it appears.
                let mut out = [0u32, pos0.to_bits(), pos1.to_bits()];
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    C_EFFECT,
                    u32,
                    effect,
                    core::ptr::addr_of_mut!(out) as u32
                );
                let level = f32::from_bits(out[0]);
                // `ucomiss` against FLT_MAX, then `lahf; (an instruction of the original); jp`:
                // the test overwrites the flags, so the jump tests the
                // parity of the masked byte, which is odd only for an equal
                // comparison. Anything but exactly FLT_MAX takes the
                // square-root-or-zero side.
                let shaped_level = if level == f32::from_bits(0x7F7F_FFFF) {
                    EFFECT_LEVEL
                } else if level > 0.0 {
                    core::hint::black_box(level).sqrt()
                } else {
                    0.0
                };
                wrf(this.wrapping_add(EFFECT_LEVEL_SLOT), shaped_level);
                wr32(this.wrapping_add(EFFECT_REQ), 0);
            }
        } else if gv_f(EFFECT_GATE_GV) > 0.0 {
            let mut start = fresh();
            if !start {
                let deadline =
                    rd32(this.wrapping_add(EFFECT_TIME)).wrapping_add(EFFECT_TIMEOUT);
                start = deadline < gv_32(EFFECT_TIME_GV);
            }
            if start {
                let ok: u32 =
                    lf_checker_rt::callee_thiscall!(C_TRY_EFFECT, u32, this, 0, 0);
                if (ok as u8) == 0 {
                    let tls_index = gv_32(TLS_INDEX_GV);
                    let tls = lf_checker_rt::tls_slot(tls_index as usize);
                    let sel = rd32(tls.wrapping_add(TLS_AUDIO_OFF));
                    let erow =
                        lf_checker_rt::relocated(EFFECT_TABLE).wrapping_add(sel.wrapping_shl(6));
                    let made: u32 = lf_checker_rt::callee_cdecl!(
                        C_MAKE_EFFECT,
                        u32,
                        erow,
                        EFFECT_LEVEL.to_bits(),
                        0
                    );
                    wr32(this.wrapping_add(EFFECT_REQ), made);
                    wr32(this.wrapping_add(EFFECT_TIME), gv_32(EFFECT_TIME_GV));
                }
            }
        }

        if rd8(this.wrapping_add(MUTE_STATE)) != 0 && (e20 as u8) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                C_MIXER,
                u32,
                lf_checker_rt::relocated(MIXER_THIS),
                gv_32(MIXER_ARG_GV),
                MIXER_B_ARG
            );
        }
        ((this.wrapping_add(MUTE_STATE)) as *mut u8).write(e20 as u8);
        ((this.wrapping_add(MUTE_FLAG)) as *mut u8).write(f12);
        ((this.wrapping_add(MUTE_SHAPED_FLAG)) as *mut u8).write(f13);
        // The `(an instruction of the original)` argument is the saved count: every path reloads
        // edi from it after using edi as scratch (the callee pops both
        // words, so the push is purely an argument).
        let shaped: f32 = lf_checker_rt::callee_thiscall!(
            C_SHAPE,
            f32,
            this.wrapping_add(SHAPE_THIS),
            (if f13 != 0 { 1.0f32 } else { 0.0f32 }).to_bits(),
            edi_saved
        );
        wrf(this.wrapping_add(MUTE_SHAPED), shaped);
        wr32(this.wrapping_add(STATUS_A), status_a);
        wr32(this.wrapping_add(STATUS_B), status_b);
        let _: u32 = lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        status_b
    }
}

lf_checker_rt::export!(thiscall, rw_00973B50(this: u32) -> u32 {
    unsafe { audio_listener_body(this, true) }
});
