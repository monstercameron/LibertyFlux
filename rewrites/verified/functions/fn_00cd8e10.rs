// original: 0x00cd8e10 CTaskComplexFollowPedFootsteps::vf18

/// Event dispatcher for a follow-ped-footsteps task (thiscall, one stack word).
///
/// `this` is the task: `+0x00` its table (slot `+0x54` takes a request code
/// and the event), `+0x08` the gate object, `+0x14` the target record,
/// `+0x20` an optional extra record. `p0` is the event, passed through to
/// whichever request runs. Every call is through a table slot; there are no
/// direct calls. The return value is always the last request's answer.
///
/// With no target record the request is `0x516` at once. Otherwise the gate
/// object's slot `+0x0c` is read: an answer of `0x11d` selects the inner
/// object at gate `+0x14` (a null inner object stands in the value `0xC8`
/// without a second call), any other answer re-reads the same slot. The
/// resulting value dispatches, the first comparison signed: above `0x1F4`
/// only `0x384` (extra-record request) and `0x38B` (range request) are
/// special; equal to `0x1F4` requests `0xCB`; below, only `0xCB` (also the
/// extra-record request) is special; everything else requests `0x516`.
///
/// The extra-record request reads `[this+0x20]`: a null record or a zero
/// first word falls back to request `0xCB`, otherwise the request is
/// `0x384`. The range request measures the squared distance between the
/// target-side block (`[[this+0x14]+0x20]`, x/y/z at `+0x30`/`+0x34`/`+0x38`)
/// and the event-side block (`[p0+0x20]`, laid out alike), formed as
/// `(dx*dx + dy*dy) + dz*dz`, and requests `0x38B` when strictly above the
/// constant 1.0 kept in the image, `0xCB` otherwise (including an unordered
/// NaN compare). A null event faults on both sides, like the original.
///
/// Original: 0x00cd8e10 (thiscall, one stack word, returns the last answer).
lf_checker_rt::export!(thiscall, rw_00cd8e10(this: u32, p0: u32) -> u32 {
    unsafe {
        const GATE_OFF: u32 = 0x08;
        const TARGET_OFF: u32 = 0x14;
        const EXTRA_OFF: u32 = 0x20;
        const INNER_OFF: u32 = 0x14;
        const POS_LINK: u32 = 0x20;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const POS_Z: u32 = 0x38;
        const VSLOT_GATE: u32 = 0x0c;
        const VSLOT_SELF: u32 = 0x54;
        const GATE_OK: u32 = 0x11d;
        const INNER_DEFAULT: u32 = 0xc8;
        const THRESH: u32 = 0x1f4;
        const EXTRA_LO: u32 = 0xcb;
        const EXTRA_HI: u32 = 0x384;
        const FLOAT_TRIG: u32 = 0x38b;
        const CODE_FALLBACK: u32 = 0x516;
        const CODE_IDLE: u32 = 0xcb;
        const CODE_EXTRA: u32 = 0x384;
        const CODE_RANGE: u32 = 0x38b;
        const RANGE_LIMIT: u32 = 0x00fe88e8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// Gate query: virtual slot `VSLOT_GATE` on `obj`.
        #[inline(always)]
        unsafe fn vgate(obj: u32) -> u32 {
            unsafe {
                let slot: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VSLOT_GATE) as usize);
                slot(obj)
            }
        }
        /// Task request: virtual slot `VSLOT_SELF` on the task itself.
        #[inline(always)]
        unsafe fn vself(this: u32, code: u32, p0: u32) -> u32 {
            unsafe {
                let slot: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this) + VSLOT_SELF) as usize);
                slot(this, code, p0)
            }
        }

        let target = rd32(this + TARGET_OFF);
        if target == 0 {
            return vself(this, CODE_FALLBACK, p0);
        }
        let gate = rd32(this + GATE_OFF);
        let a1 = vgate(gate);
        let a2 = if a1 == GATE_OK {
            let inner = rd32(gate + INNER_OFF);
            if inner == 0 {
                INNER_DEFAULT
            } else {
                vgate(inner)
            }
        } else {
            vgate(gate)
        };
        // First comparison is signed (jg); the rest match exact values.
        if (a2 as i32) > THRESH as i32 {
            if a2 == EXTRA_HI {
                let extra = rd32(this + EXTRA_OFF);
                if extra == 0 || rd32(extra) == 0 {
                    return vself(this, CODE_IDLE, p0);
                }
                return vself(this, CODE_EXTRA, p0);
            }
            if a2 != FLOAT_TRIG {
                return vself(this, CODE_FALLBACK, p0);
            }
            let tbl = rd32(target + POS_LINK);
            let evt = rd32(p0 + POS_LINK);
            let dx = sub(
                f32::from_bits(rd32(tbl + POS_X)),
                f32::from_bits(rd32(evt + POS_X)),
            );
            let dy = sub(
                f32::from_bits(rd32(tbl + POS_Y)),
                f32::from_bits(rd32(evt + POS_Y)),
            );
            let dz = sub(
                f32::from_bits(rd32(tbl + POS_Z)),
                f32::from_bits(rd32(evt + POS_Z)),
            );
            let d2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
            let limit = f32::from_bits(lf_checker_rt::global::<u32>(RANGE_LIMIT).read());
            // comiss+jbe: the far code only when strictly greater.
            if d2 > limit {
                return vself(this, CODE_RANGE, p0);
            }
            return vself(this, CODE_IDLE, p0);
        }
        if a2 == THRESH {
            return vself(this, CODE_IDLE, p0);
        }
        if a2 != EXTRA_LO {
            return vself(this, CODE_FALLBACK, p0);
        }
        let extra = rd32(this + EXTRA_OFF);
        if extra == 0 || rd32(extra) == 0 {
            return vself(this, CODE_IDLE, p0);
        }
        vself(this, CODE_EXTRA, p0)
    }
});
