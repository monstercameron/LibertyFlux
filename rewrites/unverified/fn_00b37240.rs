// original: 0x00b37240 ped_task_range_gate (proposed)

/// Decide whether a ped may take a task, from its range bands to a target.
///
/// `obj` is the ped, `rate` a scaling factor, `target` three floats and
/// `flag` a bypass byte; returns 1 for yes, 0 for no. A position `p` is the
/// object's linked position plus `0x30` when the link at `+0x20` is set,
/// otherwise the object itself plus `0x10`.
///
/// First a direct cdecl gate over `p` vetoes with 1. Unless the flag byte is
/// set, a device check runs: with the device at `+0xb0` set and the count at
/// `+0xb8` positive, a direct thiscall poll must answer nonzero and a direct
/// thiscall fetch of the count must then answer zero for an immediate 1.
///
/// Otherwise two range bands are formed from `rate`: the outer is
/// `115.0 * rate`, the inner `70.0 * rate`. When bits 6..9 of the word at
/// `+0x28` equal `0xc0`, a direct thiscall mode query answers: nonzero
/// scales both bands by 1.5 and clears the latch; zero reads bit 30 of the
/// word at `+0x29c`, and when set scales both bands by 1.6 if the word at
/// `+0x12c` of the object at `+0x21c` equals 2, else by 1.0. The outer band
/// multiplies as `band * scale`, the inner as `scale * band`, matching the
/// original's operand order.
///
/// A direct thiscall speed query (single-precision x87 return) then gates
/// two divisions: only when its answer is ordered-greater than +0.0 are the
/// outer and inner bands each divided by a fresh answer in turn. The squared
/// distance from `p` to the target, `(dy*dy + dx*dx) + dz*dz` with
/// `d = p - target`, above the squared outer band returns 1; at or below
/// the squared inner band (or unordered) returns 0; between the bands the
/// latch decides, set meaning a final direct thiscall idle query with
/// argument 1 must answer zero for a 1.
///
/// Original: 0x00b37240 (cdecl, four stack words, byte return in al).
lf_checker_rt::export!(cdecl, rw_00b37240(obj: u32, rate: u32, target: u32, flag: u32) -> u32 {
    unsafe {
        const OUTER_RATE: u32 = 0x01045994; // 115.0
        const INNER_RATE: u32 = 0x01045990; // 70.0
        const SCALE_DIRECT: u32 = 0x010459a0; // 1.5
        const SCALE_NEAR: u32 = 0x0104599c; // 1.6
        const SCALE_FAR: u32 = 0x01045998; // 1.0
        const SPEED_LIMIT: u32 = 0x00fe8628; // +0.0
        const OBJ_POS: u32 = 0x20;
        const OBJ_INLINE_POS: u32 = 0x10;
        const POS_X: u32 = 0x30;
        const OBJ_MODE: u32 = 0x28;
        const MODE_MASK: u32 = 0x3c0;
        const MODE_DIRECT: u32 = 0xc0;
        const OBJ_SUBFLAG: u32 = 0x29c;
        const OBJ_SUB: u32 = 0x21c;
        const SUB_NEAR: u32 = 0x12c;
        const NEAR_STATE: u32 = 2;
        const OBJ_DEV: u32 = 0xb0;
        const OBJ_COUNT: u32 = 0xb8;
        const CALLEE_GATE: u32 = 1;
        const CALLEE_POLL: u32 = 2;
        const CALLEE_FETCH: u32 = 3;
        const CALLEE_MODE: u32 = 4;
        const CALLEE_SPEED: u32 = 5;
        const CALLEE_IDLE: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn pos_of(obj: u32) -> u32 {
            unsafe {
                let link = rd32(obj.wrapping_add(OBJ_POS));
                if link != 0 {
                    link.wrapping_add(POS_X)
                } else {
                    obj.wrapping_add(OBJ_INLINE_POS)
                }
            }
        }

        let rate = f32::from_bits(rate);
        if (lf_checker_rt::callee_cdecl!(CALLEE_GATE, u32, pos_of(obj)) & 0xff) != 0 {
            return 0;
        }
        if (flag & 0xff) == 0 {
            let dev = rd32(obj.wrapping_add(OBJ_DEV));
            let count = rd32(obj.wrapping_add(OBJ_COUNT));
            if dev != 0 && (count as i32) > 0 {
                let live = lf_checker_rt::callee_thiscall!(CALLEE_POLL, u32, dev);
                if (live & 0xff) != 0 {
                    let got = lf_checker_rt::callee_thiscall!(CALLEE_FETCH, u32, dev, count);
                    if got == 0 {
                        return 1;
                    }
                }
            }
        }
        let outer_k = f32::from_bits(rd32(lf_checker_rt::relocated(OUTER_RATE)));
        let inner_k = f32::from_bits(rd32(lf_checker_rt::relocated(INNER_RATE)));
        let mut outer = mul(outer_k, rate);
        let mut inner = mul(inner_k, rate);
        let mut latched = true;
        if rd32(obj.wrapping_add(OBJ_MODE)) & MODE_MASK == MODE_DIRECT {
            let direct = lf_checker_rt::callee_thiscall!(CALLEE_MODE, u32, obj);
            if (direct & 0xff) != 0 {
                let scale =
                    f32::from_bits(rd32(lf_checker_rt::relocated(SCALE_DIRECT)));
                outer = mul(outer, scale);
                inner = mul(scale, inner);
                latched = false;
            } else {
                let bit = (rd32(obj.wrapping_add(OBJ_SUBFLAG)) >> 30) & 1;
                if bit != 0 {
                    let sub = rd32(obj.wrapping_add(OBJ_SUB));
                    let va = if rd32(sub.wrapping_add(SUB_NEAR)) == NEAR_STATE {
                        SCALE_NEAR
                    } else {
                        SCALE_FAR
                    };
                    let scale = f32::from_bits(rd32(lf_checker_rt::relocated(va)));
                    outer = mul(outer, scale);
                    inner = mul(scale, inner);
                }
            }
        }
        let limit = f32::from_bits(rd32(lf_checker_rt::relocated(SPEED_LIMIT)));
        let speed0: f32 = lf_checker_rt::callee_thiscall!(CALLEE_SPEED, f32, obj);
        if speed0 > limit {
            let speed1: f32 = lf_checker_rt::callee_thiscall!(CALLEE_SPEED, f32, obj);
            outer = div(outer, speed1);
            let speed2: f32 = lf_checker_rt::callee_thiscall!(CALLEE_SPEED, f32, obj);
            inner = div(inner, speed2);
        }
        let p = pos_of(obj);
        let dx = sub(
            f32::from_bits(rd32(p)),
            f32::from_bits(rd32(target)),
        );
        let dy = sub(
            f32::from_bits(rd32(p.wrapping_add(4))),
            f32::from_bits(rd32(target.wrapping_add(4))),
        );
        let dz = sub(
            f32::from_bits(rd32(p.wrapping_add(8))),
            f32::from_bits(rd32(target.wrapping_add(8))),
        );
        let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        if dist2 > mul(outer, outer) {
            return 1;
        }
        // comiss + jbe: at-or-below, or unordered, returns 0.
        if !(dist2 > mul(inner, inner)) {
            return 0;
        }
        if !latched {
            return 0;
        }
        let idle = lf_checker_rt::callee_thiscall!(CALLEE_IDLE, u32, obj, 1);
        if (idle & 0xff) != 0 {
            return 0;
        }
        1
    }
});
