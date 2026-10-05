// original: 0x00cde6a0 CTaskSimpleNMOnFire::vf27 (symbols)

/// NaturalMotion "on fire" task update: stamp the target's state, then either
/// send a burning-parameter message or refresh the task's burn direction.
///
/// `this` is the task object, `target` the ped (or ped-adjacent) object the
/// task acts on. The task reads its flag word at `+0x28`, a blend factor at
/// `+0x2c`, a direction vector at `+0x30` (x, y, z) with a spare word at
/// `+0x3c`, and a parameter-row index at `+0x4c`. The target contributes a
/// vector source at `+0x20` (a pointer to at least five words), a message
/// recipient at `+0x7b4`, and a state word at `+0xba0`.
///
/// Behaviour: write `0x1b9` to the target state word. If flag bit 1 is set,
/// clear the low two flag bits and send two messages: first a one-boolean
/// message, then a fourteen-float message whose values come from row
/// `index * 0x68` of the runtime parameter table (floats at row offsets
/// `0x28..0x5c`), plus a string and a boolean derived from a helper call's
/// low two bits. Otherwise, when the blend factor is ordered and
/// non-negative and flag bit 0 is set: copy word 0 of the parameter row into
/// the blend slot if the blend is exactly zero; if any direction component
/// is non-zero skip the refresh, else copy the source vector (or, when its
/// squared length is not above a small threshold, the un-offset vector),
/// scale it by a helper-supplied factor, and then always send a final
/// boolean/vector/float message. Messages are built in scratch buffers
/// through intercepted callees; their bytes are never read back directly.
///
/// Float order is the original's (squared-length sums add x, then y, then z;
/// scaling multiplies component by factor). NaN inputs follow the original's
/// unordered-compare branches. Original: thiscall, one stack word, no return
/// value (callee pops the argument).
lf_checker_rt::export!(thiscall, rw_00cde6a0(this: u32, target: u32) -> u32 {
    unsafe {
        const TARGET_STATE: u32 = 0xba0;
        const STATE_BURNING: u32 = 0x1b9;
        const TARGET_VEC_SRC: u32 = 0x20;
        const TARGET_MSG_TO: u32 = 0x7b4;
        const TASK_FLAGS: u32 = 0x28;
        const TASK_BLEND: u32 = 0x2c;
        const TASK_DIR_X: u32 = 0x30;
        const TASK_DIR_Y: u32 = 0x34;
        const TASK_DIR_Z: u32 = 0x38;
        const TASK_DIR_W: u32 = 0x3c;
        const TASK_PARAM_ROW: u32 = 0x4c;
        const FLAG_SEND_PARAMS: u32 = 2;
        const FLAG_DIRTY: u32 = 1;
        const TABLE_BASE: u32 = 0x0171_ca40;
        const TABLE_STRIDE: u32 = 0x68;
        const TABLE_ROW_WORD: u32 = 0x00;
        const TABLE_ROW_FLOATS: u32 = 0x28;
        const N_FLOATS: u32 = 14;
        const MSG_STR: u32 = 0x00ed_b3cc;
        const G_MSG_A_ID: u32 = 0x0105_1cc8;
        const G_MSG_A_SEND: u32 = 0x0105_1dfc;
        const G_MSG_B_ID: u32 = 0x0105_1cc8;
        const G_BOOL2_ID: u32 = 0x0105_1da8;
        const G_STR_ID: u32 = 0x0105_1da4;
        const G_MSG_B_SEND: u32 = 0x0105_1d64;
        const G_MSG_C_ID: u32 = 0x0105_1cc8;
        const G_MSG_C_BOOL: u32 = 0x0105_1e98;
        const G_MSG_C_VEC: u32 = 0x0105_1e90;
        const G_MSG_C_FLOAT: u32 = 0x0105_1e94;
        const G_MSG_C_SEND: u32 = 0x0105_1e88;
        const G_FLOAT_IDS: [u32; 14] = [
            0x0105_1d6c, 0x0105_1d88, 0x0105_1d7c, 0x0105_1d94, 0x0105_1da0,
            0x0105_1d74, 0x0105_1d8c, 0x0105_1d80, 0x0105_1d98, 0x0105_1d70,
            0x0105_1d84, 0x0105_1d78, 0x0105_1d90, 0x0105_1d9c,
        ];
        const G_LEN_THRESH: u32 = 0x00fe_870c;
        const C_INIT: u32 = 1;
        const C_SET_BOOL: u32 = 2;
        const C_SEND2: u32 = 3;
        const C_SET_FLOAT: u32 = 4;
        const C_SET_STRING: u32 = 5;
        const C_DECIDE: u32 = 6;
        const C_SEND3: u32 = 7;
        const C_FACTOR: u32 = 8;
        const C_COOKIE: u32 = 10;
        const C_SET_VEC3: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        unsafe fn gid(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
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
        unsafe fn table_word(row: u32, off: u32) -> u32 {
            unsafe {
                let base = lf_checker_rt::relocated(TABLE_BASE);
                rd32(base.wrapping_add(row).wrapping_add(off))
            }
        }

        // The original's stack-allocator prologue call runs natively (not
        // intercepted): Rust owns its own frame instead.
        wr32(target.wrapping_add(TARGET_STATE), STATE_BURNING);
        let flags = rd32(this.wrapping_add(TASK_FLAGS));
        // Scratch message buffers. The intercepted callees never read or
        // write them observably; the original never reads them back either.
        let mut buf_hi = [0u32; 64];
        let mut buf_lo = [0u32; 64];
        let b1 = buf_hi.as_mut_ptr() as u32;
        let b0 = buf_lo.as_mut_ptr() as u32;
        if flags & FLAG_SEND_PARAMS != 0 {
            wr32(this.wrapping_add(TASK_FLAGS), flags & 0xffff_fffc);
            lf_checker_rt::callee_thiscall!(C_INIT, u32, b1);
            lf_checker_rt::callee_thiscall!(C_SET_BOOL, u32, b1, gid(G_MSG_A_ID), 0);
            lf_checker_rt::callee_thiscall!(
                C_SEND2, u32,
                rd32(target.wrapping_add(TARGET_MSG_TO)),
                gid(G_MSG_A_SEND),
                b1
            );
            lf_checker_rt::callee_thiscall!(C_INIT, u32, b0);
            lf_checker_rt::callee_thiscall!(C_SET_BOOL, u32, b0, gid(G_MSG_B_ID), 1);
            let row = (rd32(this.wrapping_add(TASK_PARAM_ROW)) as i32)
                .wrapping_mul(TABLE_STRIDE as i32) as u32;
            let mut k = 0u32;
            while k < N_FLOATS {
                lf_checker_rt::callee_thiscall!(
                    C_SET_FLOAT, u32, b0, gid(G_FLOAT_IDS[k as usize]),
                    table_word(row, TABLE_ROW_FLOATS + k * 4)
                );
                k += 1;
            }
            lf_checker_rt::callee_thiscall!(
                C_SET_STRING, u32, b0, gid(G_STR_ID),
                lf_checker_rt::relocated(MSG_STR)
            );
            let decision: u32 = lf_checker_rt::callee_cdecl!(C_DECIDE, u32,);
            let b = if decision & 0xff & 3 == 0 { 0 } else { 1 };
            lf_checker_rt::callee_thiscall!(C_SET_BOOL, u32, b0, gid(G_BOOL2_ID), b);
            lf_checker_rt::callee_thiscall!(
                C_SEND2, u32,
                rd32(target.wrapping_add(TARGET_MSG_TO)),
                gid(G_MSG_B_SEND),
                b0
            );
            lf_checker_rt::callee_thiscall!(C_SEND3, u32, b0);
            lf_checker_rt::callee_thiscall!(C_SEND3, u32, b1);
        } else {
            let mut send_final = true;
            let blend = rdf(this.wrapping_add(TASK_BLEND));
            // Less-or-unordered branch: exit unless blend is ordered and >= 0.
            if blend >= 0.0 && flags & FLAG_DIRTY != 0 {
                // Equality branch (NaN counts as different): copy the row word
                // only when blend is +0/-0.
                if blend == 0.0 {
                    let row = (rd32(this.wrapping_add(TASK_PARAM_ROW)) as i32)
                        .wrapping_mul(TABLE_STRIDE as i32)
                        as u32;
                    wr32(this.wrapping_add(TASK_BLEND), table_word(row, TABLE_ROW_WORD));
                }
                let dx = rdf(this.wrapping_add(TASK_DIR_X));
                let dy = rdf(this.wrapping_add(TASK_DIR_Y));
                let dz = rdf(this.wrapping_add(TASK_DIR_Z));
                // Each inequality branch skips the refresh when that component
                // is non-zero (NaN counts as non-zero).
                if dx == 0.0 && dy == 0.0 && dz == 0.0 {
                    let src = rd32(target.wrapping_add(TARGET_VEC_SRC)).wrapping_add(0x10);
                    let px = rdf(src);
                    let py = rdf(src.wrapping_add(4));
                    wrf(this.wrapping_add(TASK_DIR_X), f32::from_bits(rd32(src)));
                    wrf(this.wrapping_add(TASK_DIR_Y), py);
                    wrf(this.wrapping_add(TASK_DIR_Z), rdf(src.wrapping_add(8)));
                    wr32(
                        this.wrapping_add(TASK_DIR_W),
                        rd32(src.wrapping_add(0xc)),
                    );
                    // Silence an unused-variable warning without changing
                    // the reads above (px is re-read below like the original).
                    core::hint::black_box(px);
                    wr32(this.wrapping_add(TASK_DIR_Z), 0);
                    let sx = rdf(this.wrapping_add(TASK_DIR_X));
                    let sy = rdf(this.wrapping_add(TASK_DIR_Y));
                    let sz = rdf(this.wrapping_add(TASK_DIR_Z));
                    let len_sq = add(add(mul(sx, sx), mul(sy, sy)), mul(sz, sz));
                    let thresh = f32::from_bits(gid(G_LEN_THRESH));
                    // Threshold branch taken unless len is ordered and <= thresh;
                    // unordered (NaN) skips the re-copy.
                    if len_sq <= thresh {
                        let src2 = rd32(target.wrapping_add(TARGET_VEC_SRC));
                        wr32(this.wrapping_add(TASK_DIR_X), rd32(src2));
                        wrf(this.wrapping_add(TASK_DIR_Y), rdf(src2.wrapping_add(4)));
                        wrf(this.wrapping_add(TASK_DIR_Z), rdf(src2.wrapping_add(8)));
                        wr32(
                            this.wrapping_add(TASK_DIR_W),
                            rd32(src2.wrapping_add(0xc)),
                        );
                        wr32(this.wrapping_add(TASK_DIR_Z), 0);
                    }
                    let fx = rdf(this.wrapping_add(TASK_DIR_X));
                    let fy = rdf(this.wrapping_add(TASK_DIR_Y));
                    let fz = rdf(this.wrapping_add(TASK_DIR_Z));
                    let norm_sq = add(add(mul(fx, fx), mul(fy, fy)), mul(fz, fz));
                    let factor: f32 =
                        lf_checker_rt::callee_cdecl!(C_FACTOR, f32, norm_sq.to_bits());
                    wrf(this.wrapping_add(TASK_DIR_X), mul(fx, factor));
                    wrf(this.wrapping_add(TASK_DIR_Y), mul(fy, factor));
                    wrf(this.wrapping_add(TASK_DIR_Z), mul(fz, factor));
                }
            } else {
                send_final = false;
            }
            if send_final {
                lf_checker_rt::callee_thiscall!(C_INIT, u32, b0);
                lf_checker_rt::callee_thiscall!(C_SET_BOOL, u32, b0, gid(G_MSG_C_ID), 1);
                lf_checker_rt::callee_thiscall!(C_SET_BOOL, u32, b0, gid(G_MSG_C_BOOL), 0);
                lf_checker_rt::callee_thiscall!(
                    C_SET_VEC3, u32, b0, gid(G_MSG_C_VEC),
                    rd32(this.wrapping_add(TASK_DIR_X)),
                    rd32(this.wrapping_add(TASK_DIR_Y)),
                    rd32(this.wrapping_add(TASK_DIR_Z))
                );
                lf_checker_rt::callee_thiscall!(
                    C_SET_FLOAT, u32, b0, gid(G_MSG_C_FLOAT),
                    rd32(this.wrapping_add(TASK_BLEND))
                );
                lf_checker_rt::callee_thiscall!(
                    C_SEND2, u32,
                    rd32(target.wrapping_add(TARGET_MSG_TO)),
                    gid(G_MSG_C_SEND),
                    b0
                );
                lf_checker_rt::callee_thiscall!(C_SEND3, u32, b0);
            }
        }
        lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        0
    }
});
