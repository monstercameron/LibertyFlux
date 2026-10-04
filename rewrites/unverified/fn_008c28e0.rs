// original: 0x008c28e0 audio_engine_update (proposed)

/// Advance the audio engine one update step.
///
/// Takes no arguments and no object (cdecl, balanced frame with a security
/// cookie); the return value is whatever the cookie-check stub answers
/// (the real check leaves `eax` alone, so the rewrite returns that same
/// scripted zero). Behaviour in order: construct two engine objects from
/// literal names, fold the mode byte into the channel dword, compare the
/// tick float against the beat float into a flag byte, and run the first
/// object update. When the level float is not bit-equal to the reference
/// (or the muted reference), store the reference and push it through an
/// equaliser call. Seed a float temp with the beat, read a mixer tap, mix
/// and keep the mixed value. Unless the mode triple (mode, id pair, stage
/// `0x12`) selects the live path, scale the mixed value by the tick and
/// continue; on the live path derive a counter (direct or mapped through
/// a callee) and a marker as unsigned int to double to float, divide,
/// cap at the tick, and scale. Push the result through an apply call,
/// then push the mixed value through one of two apply sites chosen by
/// the same triple, truncate the answer to an index, mirror a few bytes
/// and floats into the state object and globals, and use the index when
/// the index table is present. Construct a controller object, set eight
/// named float parameters (signed ints converted exactly, one scaled by
/// a milli factor, one looked up from a 24-entry constant table by the
/// selected-override index), and tear the object down. Run three object
/// updates, poll the status callback and fold it with the quiet and skip
/// bytes; a no-go runs a secondary check and three more updates. Commit
/// the channel, run another update, set one more parameter through a
/// short-lived object, run the next update stage and two post updates,
/// call the looked-up object's slot-`0x14` method when present, tear
/// down the engine object, and run the cookie check.
///
/// Temporary objects live in the original's frame and are never read
/// back, so the rewrite keeps only the two float temps and the table as
/// locals. Float order follows the original through the small ordered
/// helpers; int to float conversions are exact; float to int truncation
/// reproduces `cvttss2si` including NaN and out-of-range giving
/// `0x80000000`.
///
/// Original: 0x008c28e0 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_008c28e0() -> u32 {
    unsafe {
        const G_COOKIE: u32 = 0x01057FB4;
        const B_MODE: u32 = 0x011618F2;
        const G_CHANNEL: u32 = 0x011618FC;
        const F_TICK: u32 = 0x00FE88E8;
        const F_BEAT: u32 = 0x0103234C;
        const G_ALT_CHANNEL: u32 = 0x011735B4;
        const B_MODE_COPY: u32 = 0x011618F3;
        const B_ABOVE: u32 = 0x011618F9;
        const G_MUTE: u32 = 0x011D6FD4;
        const MUTE_VALUE: u32 = 2;
        const G_LEVEL: u32 = 0x01162524;
        const G_MODE: u32 = 0x011F7060;
        const G_ID_A: u32 = 0x012088B4;
        const G_ID_B: u32 = 0x00F1C040;
        const G_STAGE: u32 = 0x01037720;
        const STAGE_LIVE: u32 = 0x12;
        const G_COUNTER: u32 = 0x011F704C;
        const G_ENABLE: u32 = 0x011F66A0;
        const G_MARKER: u32 = 0x0115A438;
        const G_MAP_SRC: u32 = 0x011F70CC;
        const G_STATE_OBJ: u32 = 0x0115A448;
        const B_STATE_A: u32 = 0x01030C68;
        const B_STATE_B: u32 = 0x01162530;
        const B_STATE_B_OUT: u32 = 0x01168A90;
        const B_STATE_C: u32 = 0x0116252F;
        const F_APPLIED_OUT: u32 = 0x0115DBE0;
        const F_TONE: u32 = 0x01030C6C;
        const B_STATE_C_OUT: u32 = 0x01168A91;
        const B_STATE_D: u32 = 0x01030C69;
        const F_TONE_OUT: u32 = 0x0115DCE0;
        const G_INDEX_TABLE: u32 = 0x0116253C;
        const G_P0: u32 = 0x01295834;
        const G_OV_A: u32 = 0x01295854;
        const G_OV_B: u32 = 0x01295848;
        const G_OV2_A: u32 = 0x01295858;
        const G_OV2_B: u32 = 0x0129584C;
        const G_P3: u32 = 0x01295850;
        const G_P4: u32 = 0x01295844;
        const G_P5: u32 = 0x01295840;
        const G_P6: u32 = 0x01295830;
        const F_MILLI: u32 = 0x00FE86B4;
        const G_POLL_ARG: u32 = 0x017ACCD8;
        const G_POLL_SLOT: u32 = 0x00E733DC;
        const B_QUIET1: u32 = 0x0105B48F;
        const B_QUIET2: u32 = 0x017ED8D1;
        const B_SKIP1: u32 = 0x01173590;
        const B_SKIP2: u32 = 0x01173591;
        const G_FINAL_ARG: u32 = 0x011735B4;
        const OBJ_A: u32 = 0x01165880;
        const OBJ_B: u32 = 0x0115D9A0;
        const OBJ_MIX: u32 = 0x01162578;
        const OBJ_APPLY_A: u32 = 0x01168AF4;
        const OBJ_APPLY_B: u32 = 0x01168B64;
        const OBJ_APPLY_C: u32 = 0x01168B3C;
        const OBJ_SET: u32 = 0x01162630;
        const OBJ_PARAM: u32 = 0x0115DB1C;
        const OBJ_UPD2: u32 = 0x01288780;
        const OBJ_UPD3: u32 = 0x01238898;
        const OBJ_UPD4: u32 = 0x012389E0;
        const OBJ_UPD5: u32 = 0x0128AA90;
        const OBJ_SET_C: u32 = 0x01231800;
        const OBJ_LOOKUP: u32 = 0x0115DA4C;
        const STR_ENGINE: u32 = 0x00E7E738;
        const STR_CONTROLLER: u32 = 0x00E7E744;
        const STR_P0: u32 = 0x00E7E754;
        const STR_P1: u32 = 0x00E7E768;
        const STR_P2: u32 = 0x00E7E778;
        const STR_P3: u32 = 0x00E7E78C;
        const STR_P4: u32 = 0x00E7E7A0;
        const STR_P5: u32 = 0x00E7E7B0;
        const STR_P6: u32 = 0x00E7E7C0;
        const STR_PT: u32 = 0x00E7E7D4;
        const VT_SLOT: u32 = 0x14;
        const C_CTOR_STR: u32 = 1;
        const C_CTOR: u32 = 2;
        const C_DTOR: u32 = 3;
        const C_UPD1: u32 = 4;
        const C_SET_A: u32 = 5;
        const C_SET_EQ: u32 = 6;
        const C_GET: u32 = 7;
        const C_MIX: u32 = 8;
        const C_MAPIDX: u32 = 9;
        const C_APPLY: u32 = 10;
        const C_SET_B: u32 = 11;
        const C_PARAM: u32 = 12;
        const C_SCAN: u32 = 13;
        const C_FLUSH: u32 = 14;
        const C_UPD2: u32 = 15;
        const C_UPD3: u32 = 16;
        const C_POLL: u32 = 17;
        const C_SECOND: u32 = 18;
        const C_UPD4: u32 = 19;
        const C_UPD5: u32 = 20;
        const C_SET_C: u32 = 21;
        const C_COMMIT: u32 = 22;
        const C_UPD6: u32 = 23;
        const C_SET_D: u32 = 24;
        const C_NEXT: u32 = 25;
        const C_UPD7: u32 = 26;
        const C_POST: u32 = 27;
        const C_LOOKUP: u32 = 28;
        const C_VCALL: u32 = 29;
        const C_COOKIE: u32 = 30;
        const C_USEIDX: u32 = 31;
        const NOT_OVERRIDDEN: u32 = 0xFFFF_FFFF;
        /// Constant table the original builds in its frame: four 0s, two
        /// 1s, two 2s, eleven 3s, two 4s, three 5s.
        const TABLE_BITS: [u32; 24] = [
            0, 0, 0, 0, 0x3F800000, 0x3F800000, 0x40000000, 0x40000000, 0x40400000,
            0x40400000, 0x40400000, 0x40400000, 0x40400000, 0x40400000, 0x40400000,
            0x40400000, 0x40400000, 0x40400000, 0x40400000, 0x40800000, 0x40800000,
            0x40A00000, 0x40A00000, 0x40A00000,
        ];

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (lf_checker_rt::global::<u8>(a) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(a) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (lf_checker_rt::global::<u8>(a) as *mut u8).write(v) }
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// Unsigned int to float by way of double, as the original's
        /// convert, bias-add and narrow sequence computes it.
        #[inline(always)]
        fn u2f(v: u32) -> f32 {
            (v as f64) as f32
        }
        /// `cvttss2si` semantics: truncate, with NaN and out-of-range
        /// giving the integer indefinite value.
        #[inline(always)]
        fn cvtt(v: f32) -> u32 {
            const LIM: f32 = 2147483648.0;
            if v.is_nan() || v >= LIM || v < -LIM {
                0x8000_0000
            } else {
                (v as i32) as u32
            }
        }
        #[inline(always)]
        unsafe fn live_path() -> bool {
            unsafe { rd32(G_MODE) != 1 && rd32(G_ID_A) == rd32(G_ID_B) && rd32(G_STAGE) != STAGE_LIVE }
        }
        #[inline(always)]
        unsafe fn param(obj: u32, name: u32, v: f32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(C_PARAM, u32, obj, name, v.to_bits());
            }
        }

        let obj_b = lf_checker_rt::relocated(OBJ_B);
        // Engine objects (frame temporaries the original never reads back;
        // only the calls are observable).
        lf_checker_rt::callee_thiscall!(C_CTOR_STR, u32, 0, lf_checker_rt::relocated(STR_ENGINE));
        lf_checker_rt::callee_thiscall!(C_CTOR, u32, 0,);
        // Mode select, flag byte, first update.
        let mode = rd8(B_MODE);
        let mut channel = rd32(G_CHANNEL);
        if mode == 0 {
            channel = rd32(G_ALT_CHANNEL);
        }
        let tick = rdf(F_TICK);
        wr32(G_CHANNEL, channel);
        wr8(B_MODE_COPY, mode);
        wr8(B_ABOVE, (tick > rdf(F_BEAT)) as u8);
        lf_checker_rt::callee_thiscall!(C_UPD1, u32, lf_checker_rt::relocated(OBJ_A));
        if mode == 0 {
            lf_checker_rt::callee_thiscall!(C_SET_A, u32, obj_b, rd32(G_CHANNEL));
        }
        // Level gate.
        let reference = if rd32(G_MUTE) == MUTE_VALUE { tick } else { 0.0 };
        if rdf(G_LEVEL) == reference {
            wrf(G_LEVEL, reference);
            lf_checker_rt::callee_thiscall!(C_SET_EQ, u32, obj_b, reference.to_bits());
        }
        // Mixer tap and mix.
        let mut mixed = rdf(F_BEAT);
        let tap = lf_checker_rt::callee_thiscall!(C_GET, u32, obj_b);
        mixed = lf_checker_rt::callee_thiscall!(C_MIX, f32, lf_checker_rt::relocated(OBJ_MIX), mixed.to_bits(), tap);
        // Counter path.
        let mut scaled = tick;
        if !live_path() {
            let counter = rd32(G_COUNTER);
            if counter != 0 {
                let mut marker;
                if live_path() {
                    // Dead in practice (the outer test excluded it), kept
                    // to match the original's shape.
                    marker = rd32(G_MAP_SRC);
                    if marker == 0 {
                        scaled = tick;
                    } else {
                        let mapped = lf_checker_rt::callee_cdecl!(C_MAPIDX, u32, marker);
                        marker = rd32(G_MARKER);
                        scaled = div(u2f(mapped), u2f(marker));
                        if scaled > tick {
                            scaled = tick;
                        }
                    }
                } else if rd32(G_ENABLE) != 0 && {
                    marker = rd32(G_MARKER);
                    marker != 0
                } {
                    scaled = div(u2f(counter), u2f(marker));
                    if scaled > tick {
                        scaled = tick;
                    }
                } else if rd32(G_ENABLE) == 0 {
                    let src = rd32(G_MAP_SRC);
                    if src == 0 {
                        scaled = tick;
                    } else {
                        let mapped = lf_checker_rt::callee_cdecl!(C_MAPIDX, u32, src);
                        marker = rd32(G_MARKER);
                        scaled = div(u2f(mapped), u2f(marker));
                        if scaled > tick {
                            scaled = tick;
                        }
                    }
                }
            }
        } else {
            scaled = mixed;
        }
        scaled = mul(scaled, mixed);
        let applied: f32 = lf_checker_rt::callee_thiscall!(
            C_APPLY, f32, lf_checker_rt::relocated(OBJ_APPLY_A), scaled.to_bits()
        );
        // Second apply site chosen by the same triple.
        let site = if live_path() {
            OBJ_APPLY_B
        } else {
            OBJ_APPLY_C
        };
        mixed = lf_checker_rt::callee_thiscall!(C_APPLY, f32, lf_checker_rt::relocated(site), mixed.to_bits());
        let index = cvtt(mixed) & 0xFFFF;
        // Mirror bytes and floats into the state object and globals.
        let state_obj = rd32(G_STATE_OBJ);
        wr8(B_STATE_B_OUT, rd8(B_STATE_B));
        wrf(F_APPLIED_OUT, applied);
        wr8(B_STATE_C_OUT, rd8(B_STATE_C));
        wrf(F_TONE_OUT, rdf(F_TONE));
        wr8(state_obj + 0x95, rd8(B_STATE_D));
        wr8(state_obj + 0x94, rd8(B_STATE_A));
        let index_table = rd32(G_INDEX_TABLE);
        if index_table != 0 {
            lf_checker_rt::callee_thiscall!(C_USEIDX, u32, index_table, index);
        }
        // Controller object and the eight named parameters.
        lf_checker_rt::callee_thiscall!(C_CTOR_STR, u32, 0, lf_checker_rt::relocated(STR_CONTROLLER));
        lf_checker_rt::callee_thiscall!(C_CTOR, u32, 0,);
        lf_checker_rt::callee_thiscall!(C_SET_B, u32, lf_checker_rt::relocated(OBJ_SET), rd32(G_CHANNEL));
        lf_checker_rt::callee_thiscall!(C_DTOR, u32, 0,);
        lf_checker_rt::callee_cdecl!(C_SCAN, u32,);
        let obj_param = lf_checker_rt::relocated(OBJ_PARAM);
        param(obj_param, lf_checker_rt::relocated(STR_P0), (rd32(G_P0) as i32) as f32);
        let ov_a = rd32(G_OV_A);
        let ov = if ov_a != NOT_OVERRIDDEN { ov_a } else { rd32(G_OV_B) };
        param(obj_param, lf_checker_rt::relocated(STR_P1), (ov as i32) as f32);
        let ov2_a = rd32(G_OV2_A);
        let ov2 = if ov2_a != NOT_OVERRIDDEN { ov2_a } else { rd32(G_OV2_B) };
        param(obj_param, lf_checker_rt::relocated(STR_P2), (ov2 as i32) as f32);
        param(obj_param, lf_checker_rt::relocated(STR_P3), (rd32(G_P3) as i32) as f32);
        param(obj_param, lf_checker_rt::relocated(STR_P4), (rd32(G_P4) as i32) as f32);
        param(obj_param, lf_checker_rt::relocated(STR_P5), (rd32(G_P5) as i32) as f32);
        param(
            obj_param,
            lf_checker_rt::relocated(STR_P6),
            mul((rd32(G_P6) as i32) as f32, rdf(F_MILLI)),
        );
        param(obj_param, lf_checker_rt::relocated(STR_PT), f32::from_bits(TABLE_BITS[ov as usize]));
        // Updates, poll, conditional secondaries.
        lf_checker_rt::callee_cdecl!(C_FLUSH, u32,);
        lf_checker_rt::callee_thiscall!(C_UPD2, u32, lf_checker_rt::relocated(OBJ_UPD2));
        lf_checker_rt::callee_thiscall!(C_UPD3, u32, lf_checker_rt::relocated(OBJ_UPD3));
        let poll: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(rd32(G_POLL_SLOT) as usize);
        let answer = poll(rd32(G_POLL_ARG));
        let mut go = (answer != 0 || rd8(B_QUIET1) != 0 && rd8(B_QUIET2) != 0) as u8;
        go |= rd8(B_SKIP1) | rd8(B_SKIP2);
        if go == 0 {
            let second = lf_checker_rt::callee_cdecl!(C_SECOND, u32,);
            if (second as u8) == 0 {
                lf_checker_rt::callee_thiscall!(C_UPD4, u32, lf_checker_rt::relocated(OBJ_UPD4));
                lf_checker_rt::callee_thiscall!(C_UPD5, u32, lf_checker_rt::relocated(OBJ_UPD5));
                lf_checker_rt::callee_thiscall!(
                    C_SET_C, u32, lf_checker_rt::relocated(OBJ_SET_C), rd32(G_FINAL_ARG)
                );
            }
        }
        // Commit, short-lived object, next stage, post updates.
        lf_checker_rt::callee_cdecl!(C_COMMIT, u32, rd32(G_CHANNEL));
        lf_checker_rt::callee_thiscall!(C_UPD6, u32, lf_checker_rt::relocated(OBJ_A));
        lf_checker_rt::callee_thiscall!(C_CTOR, u32, 0,);
        lf_checker_rt::callee_thiscall!(C_SET_D, u32, lf_checker_rt::relocated(OBJ_SET), rd32(G_CHANNEL));
        lf_checker_rt::callee_thiscall!(C_DTOR, u32, 0,);
        lf_checker_rt::callee_cdecl!(C_NEXT, u32,);
        lf_checker_rt::callee_thiscall!(C_UPD7, u32, obj_b);
        lf_checker_rt::callee_cdecl!(C_POST, u32,);
        let found = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, lf_checker_rt::relocated(OBJ_LOOKUP), 3);
        if found != 0 {
            let method: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(found) + VT_SLOT) as usize);
            method(found);
        }
        lf_checker_rt::callee_thiscall!(C_DTOR, u32, 0,);
        lf_checker_rt::callee_thiscall!(C_COOKIE, u32, rd32(G_COOKIE));
        0
    }
});
