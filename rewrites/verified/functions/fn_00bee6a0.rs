// original: 0x00bee6a0 ped_task_commit_blend (proposed)

/// Commit a blended ped-task state block into the live task object.
///
/// `this` is the live object, `state` the blended block. After notifying two
/// helpers (a flag callee with constant `0xd`, then a state callee taking the
/// block), the position is copied (`state+0x1ed4/+0x1ec0` to `this+0x70/+0x74`),
/// the height comes from the virtual slot at `+0xfc` of the state object
/// (returned on the x87 stack) into `this+0x78`, and the heading goes from
/// `state+0x1ed8` to `this+0x7c`.
///
/// Three packed mode bits are then refreshed: bit 0 of `this+0x81` is set
/// from bit 0 of `state+0xd4f`, and `this+0x80` is rebuilt as three 2-bit
/// fields decoded from `state+0x1ef4/+0x1ef8/+0x1efc` by the float-to-code
/// helper (low bits first), with the top two bits from `state+0xd14`.
/// Finally `state+0x1f68`, clamped to [0, 1] (NaN passes through), is scaled
/// by 255, truncated toward zero with x87 convert semantics (out of range or
/// NaN yields the indefinite value), and stored as the byte at `this+0x82`.
///
/// Original: thiscall, one stack word (`state`), no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00bee6a0(this: u32, state: u32) -> u32 {
    unsafe {
        const POS_X: u32 = 0x70;
        const POS_Y: u32 = 0x74;
        const POS_Z: u32 = 0x78;
        const HEADING: u32 = 0x7c;
        const MODE: u32 = 0x80;
        const FLAG: u32 = 0x81;
        const LEVEL: u32 = 0x82;
        const ST_X: u32 = 0x1ed4;
        const ST_Y: u32 = 0x1ec0;
        const ST_H: u32 = 0x1ed8;
        const ST_C0: u32 = 0x1ef4;
        const ST_C1: u32 = 0x1ef8;
        const ST_C2: u32 = 0x1efc;
        const ST_TOP: u32 = 0xd14;
        const ST_FLAG: u32 = 0xd4f;
        const ST_LEVEL: u32 = 0x1f68;
        const VTABLE_SLOT_Z: u32 = 0xfc;
        const FLAG_ARG: u32 = 0xd;
        const ONE: f32 = 1.0;
        const FULL: f32 = 255.0;
        const CALLEE_FLAG: u32 = 1;
        const CALLEE_STATE: u32 = 2;
        const CALLEE_CODE: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        /// Truncate toward zero with `cvttss2si` overflow semantics: NaN or
        /// out of i32 range yields `i32::MIN` instead of saturating.
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }

        lf_checker_rt::callee_thiscall!(CALLEE_FLAG, u32, this, FLAG_ARG);
        lf_checker_rt::callee_thiscall!(CALLEE_STATE, u32, this, state);
        wr32(this + POS_X, rd32(state + ST_X));
        wr32(this + POS_Y, rd32(state + ST_Y));
        // Indirect height call through the state object's table, exactly like
        // the original; the checker plants its recorder stub at the slot.
        let slot: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(rd32(rd32(state) + VTABLE_SLOT_Z) as usize);
        wrf(this + POS_Z, slot(state));
        wr32(this + HEADING, rd32(state + ST_H));

        wr8(this + FLAG, rd8(this + FLAG) & 0xfe | (rd8(state + ST_FLAG) & 1));
        let c0 = lf_checker_rt::callee_thiscall!(CALLEE_CODE, u32, this, rd32(state + ST_C0));
        let c1 = lf_checker_rt::callee_thiscall!(CALLEE_CODE, u32, this, rd32(state + ST_C1));
        let c2 = lf_checker_rt::callee_thiscall!(CALLEE_CODE, u32, this, rd32(state + ST_C2));
        let mode = (c0 & 3) | ((c1 & 3) << 2) | ((c2 & 3) << 4) | ((rd8(state + ST_TOP) as u32 & 3) << 6);
        wr8(this + MODE, mode as u8);

        let v = rdf(state + ST_LEVEL);
        let clamped = if 0.0 > v {
            0.0
        } else if v > ONE {
            ONE
        } else {
            v
        };
        wr8(this + LEVEL, cvtt(mul(clamped, FULL)) as u8);
        0
    }
});
