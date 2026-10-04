// original: 0x00ae16a0 MO_CLPNOSAVE (merged)

/// Refresh the saved clip/no-save mouse state for the current frame.
///
/// Takes no arguments (cdecl, nothing read from the incoming stack). It
/// opens the frame pair (thiscall with (1, 4) on a fixed object, then a
/// chained thiscall on its answer), runs the coordinate refresher as a
/// black box, and publishes four scaled floats: each block selects one of
/// a global integer pair through the mode callee's low byte, converts to
/// float, and multiplies by a calibration float (the last two blocks sum
/// two calibration floats first, and the last multiplies by one more
/// constant), storing to four output globals.
///
/// What follows depends on the state byte at the state object's `+0x19`.
/// When clear, each of five component globals with a non-null object gets
/// an indirect thiscall through its table slot at `+4`; a device query
/// then compares two bytes (each xored with a tag byte) against `0x7f`:
/// a high first byte or a low second byte ends the frame, otherwise a
/// command thiscall runs and either
/// the pending path (state `+0x18` set, global flag clear: mark `+0x19`,
/// run the flush callee, sink eight words) or the flag path (set the
/// shared flag byte) is taken. When the state byte is set, a different
/// eight-word sink runs first, the same two-byte query runs again (this
/// time clearing `+0x19` through another command when the first byte is
/// low and the second high), and a second query over another tag either
/// ends the frame or raises the shared flag byte when its second byte is
/// high, keeping the old value otherwise.
///
/// The tail runs the notifier thiscall when its global is non-null, and
/// when the state byte at `+0x1b` is clear and the closer callee answers
/// true, closes with the frame pair again (thiscall with (0, 4), then
/// the chained call). Nothing meaningful is returned.
///
/// Original: 0x00ae16a0 (cdecl, no stack arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00ae16a0() -> u32 {
    unsafe {
        const ST_PEND: u32 = 0x18;
        const ST_STATE: u32 = 0x19;
        const ST_TAIL: u32 = 0x1b;
        const VT_SLOT: u32 = 0x04;
        const DEV_TAG_A: u32 = 0x2fdc;
        const DEV_BASE_A: u32 = 0x2fd8;
        const DEV_TAG_B: u32 = 0x2b6c;
        const DEV_BASE_B: u32 = 0x2b68;
        const G_SELA0: u32 = 0x0105c884;
        const G_SELA1: u32 = 0x0105c888;
        const G_SELB0: u32 = 0x0105c880;
        const G_SELB1: u32 = 0x0105c87c;
        const G_CAL0: u32 = 0x01593b94;
        const G_CAL1: u32 = 0x01593b98;
        const G_CAL2: u32 = 0x01593b9c;
        const G_CAL3: u32 = 0x01593ba0;
        const G_K: u32 = 0x00fe8880;
        const G_OUT0: u32 = 0x0103f69c;
        const G_OUT1: u32 = 0x0103f6a0;
        const G_OUT2: u32 = 0x0103f6a4;
        const G_OUT3: u32 = 0x0103f6a8;
        const G_STATE: u32 = 0x01593b78;
        const G_COMP0: u32 = 0x01593b68;
        const G_COMP1: u32 = 0x01593b70;
        const G_COMP2: u32 = 0x01593b74;
        const G_COMP3: u32 = 0x01593b6c;
        const G_COMP4: u32 = 0x01593b80;
        const G_GATE: u32 = 0x0103775a;
        const G_FLAG: u32 = 0x011f7071;
        const G_NOTIFY: u32 = 0x01593b7c;
        const FRAME_OBJ: u32 = 0x0103e498;
        const CMD_OBJ: u32 = 0x01176888;
        const CALLEE_FRAME: u32 = 1;
        const CALLEE_CHAIN: u32 = 2;
        const CALLEE_COORDS: u32 = 3;
        const CALLEE_MODE: u32 = 4;
        const CALLEE_COMP: u32 = 5;
        const CALLEE_DEV: u32 = 6;
        const CALLEE_CMD: u32 = 7;
        const CALLEE_FLUSH: u32 = 8;
        const CALLEE_SINK: u32 = 9;
        const CALLEE_NOTIFY: u32 = 10;
        const CALLEE_CLOSER: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn comp(g: u32) {
            unsafe {
                let o = rd32(lf_checker_rt::relocated(g));
                if o != 0 {
                    let f: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(rd32(o).wrapping_add(VT_SLOT)) as usize);
                    f(o);
                }
            }
        }

        let fobj = lf_checker_rt::relocated(FRAME_OBJ);
        let opened: u32 = lf_checker_rt::callee_thiscall!(CALLEE_FRAME, u32, fobj, 1u32, 4u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CHAIN, u32, opened);
        let _: u32 = lf_checker_rt::callee_cdecl!(CALLEE_COORDS, u32,);
        let mode: u32 = lf_checker_rt::callee_cdecl!(CALLEE_MODE, u32,);
        let s = if (mode & 0xff) != 0 {
            rd32(lf_checker_rt::relocated(G_SELA1))
        } else {
            rd32(lf_checker_rt::relocated(G_SELA0))
        };
        let f: f32 = core::hint::black_box(s as i32) as f32;
        let r = mul(f, f32::from_bits(rd32(lf_checker_rt::relocated(G_CAL0))));
        wr32(lf_checker_rt::relocated(G_OUT0), r.to_bits());
        let mode: u32 = lf_checker_rt::callee_cdecl!(CALLEE_MODE, u32,);
        let s = if (mode & 0xff) != 0 {
            rd32(lf_checker_rt::relocated(G_SELB1))
        } else {
            rd32(lf_checker_rt::relocated(G_SELB0))
        };
        let f: f32 = core::hint::black_box(s as i32) as f32;
        let r = mul(f, f32::from_bits(rd32(lf_checker_rt::relocated(G_CAL1))));
        wr32(lf_checker_rt::relocated(G_OUT1), r.to_bits());
        let mode: u32 = lf_checker_rt::callee_cdecl!(CALLEE_MODE, u32,);
        let t = add(
            f32::from_bits(rd32(lf_checker_rt::relocated(G_CAL2))),
            f32::from_bits(rd32(lf_checker_rt::relocated(G_CAL0))),
        );
        let s = if (mode & 0xff) != 0 {
            rd32(lf_checker_rt::relocated(G_SELA1))
        } else {
            rd32(lf_checker_rt::relocated(G_SELA0))
        };
        let f: f32 = core::hint::black_box(s as i32) as f32;
        wr32(lf_checker_rt::relocated(G_OUT2), mul(t, f).to_bits());
        let mode: u32 = lf_checker_rt::callee_cdecl!(CALLEE_MODE, u32,);
        let t = add(
            f32::from_bits(rd32(lf_checker_rt::relocated(G_CAL3))),
            f32::from_bits(rd32(lf_checker_rt::relocated(G_CAL1))),
        );
        let s = if (mode & 0xff) != 0 {
            rd32(lf_checker_rt::relocated(G_SELB1))
        } else {
            rd32(lf_checker_rt::relocated(G_SELB0))
        };
        let f: f32 = core::hint::black_box(s as i32) as f32;
        let r = mul(mul(t, f), f32::from_bits(rd32(lf_checker_rt::relocated(G_K))));
        wr32(lf_checker_rt::relocated(G_OUT3), r.to_bits());

        let state = rd32(lf_checker_rt::relocated(G_STATE));
        if rd8(state.wrapping_add(ST_STATE)) == 0 {
            comp(G_COMP0);
            comp(G_COMP1);
            comp(G_COMP2);
            comp(G_COMP3);
            comp(G_COMP4);
            let dev: u32 = lf_checker_rt::callee_cdecl!(CALLEE_DEV, u32,);
            let tag = rd8(dev.wrapping_add(DEV_TAG_A));
            let qb = dev.wrapping_add(DEV_BASE_A);
            let a = rd8(qb.wrapping_add(6)) ^ tag;
            let a2 = rd8(qb.wrapping_add(7)) ^ tag;
            if a > 0x7f || a2 <= 0x7f {
                // Tail below.
            } else {
                let cobj = lf_checker_rt::relocated(CMD_OBJ);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    CALLEE_CMD,
                    u32,
                    cobj,
                    lf_checker_rt::relocated(0xea731cu32)
                );
                let st = rd32(lf_checker_rt::relocated(G_STATE));
                if rd8(st.wrapping_add(ST_PEND)) != 0
                    && rd8(lf_checker_rt::relocated(G_GATE)) == 0
                {
                    wr8(st.wrapping_add(ST_STATE), 1);
                    let st2 = rd32(lf_checker_rt::relocated(G_STATE));
                    let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_FLUSH, u32, st2);
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        CALLEE_SINK, u32, 0u32,
                        lf_checker_rt::relocated(0xea7330u32), 0x48u32, 0u32,
                        0xffffffffu32, 0u32, 0u32, 0u32
                    );
                } else {
                    wr8(lf_checker_rt::relocated(G_FLAG), 1);
                }
            }
        } else {
            let _: u32 = lf_checker_rt::callee_cdecl!(
                CALLEE_SINK, u32, 0u32, lf_checker_rt::relocated(0xea7340u32), 0x48u32,
                0u32, 0xffffffffu32, 0u32, 0u32, 0u32
            );
            let dev: u32 = lf_checker_rt::callee_cdecl!(CALLEE_DEV, u32,);
            let tag = rd8(dev.wrapping_add(DEV_TAG_A));
            let qb = dev.wrapping_add(DEV_BASE_A);
            let a = rd8(qb.wrapping_add(6)) ^ tag;
            let a2 = rd8(qb.wrapping_add(7)) ^ tag;
            if !(a > 0x7f) && !(a2 <= 0x7f) {
                let cobj = lf_checker_rt::relocated(CMD_OBJ);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    CALLEE_CMD,
                    u32,
                    cobj,
                    lf_checker_rt::relocated(0xea7350u32)
                );
                wr8(rd32(lf_checker_rt::relocated(G_STATE)).wrapping_add(ST_STATE), 0);
            }
            let dev: u32 = lf_checker_rt::callee_cdecl!(CALLEE_DEV, u32,);
            let tag = rd8(dev.wrapping_add(DEV_TAG_B));
            let qb = dev.wrapping_add(DEV_BASE_B);
            let b = rd8(qb.wrapping_add(6)) ^ tag;
            if b <= 0x7f {
                let b2 = rd8(qb.wrapping_add(7)) ^ tag;
                let mut flag = rd8(lf_checker_rt::relocated(G_FLAG));
                if b2 > 0x7f {
                    flag = 1;
                }
                wr8(lf_checker_rt::relocated(G_FLAG), flag);
            }
        }
        let n = rd32(lf_checker_rt::relocated(G_NOTIFY));
        if n != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_NOTIFY, u32, n);
        }
        let st = rd32(lf_checker_rt::relocated(G_STATE));
        if rd8(st.wrapping_add(ST_TAIL)) != 0 {
            return 0;
        }
        let close: u32 = lf_checker_rt::callee_cdecl!(CALLEE_CLOSER, u32,);
        if (close & 0xff) == 0 {
            return 0;
        }
        let opened: u32 = lf_checker_rt::callee_thiscall!(CALLEE_FRAME, u32, fobj, 0u32, 4u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CHAIN, u32, opened);
        0
    }
});
