// original: 0x00D854E0 input_ui_solver_update
/// Steer one input-tracked slot toward its target point and report the turn.
///
/// The heading starts from the slot's stored direction, or from the sine and
/// cosine of its stored angle when no anchor is attached, and is normalised
/// before the first solver probe. When the probe declines, a mode table picks
/// the follow-up call, the two probe angles are wrapped to a half turn, the
/// reach check runs through the slot's own function table, and the blended
/// turn is handed to the writer together with the outputs. The two outputs
/// are negated on the mirrored path.
///
/// Returns the writer's answer, or the first output pointer on the mirrored
/// path.
use lf_checker_rt::{callee_cdecl, export, global, relocated};

/// Order-exact SSE scalar arithmetic. A plain Rust `+`/`-`/`*` lets the
/// compiler pick which operand lands in the destination register, and the
/// destination's NaN wins when both operands are NaN, so two different
/// payloads compare differently. The original always computes first-operand
/// first, which these helpers reproduce bit for bit.
#[inline(always)]
fn fadd1(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        a
    } else if b.is_nan() {
        b
    } else {
        a + b
    }
}

/// Order-exact subtraction; see [`fadd1`].
#[inline(always)]
fn fsub1(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        a
    } else if b.is_nan() {
        b
    } else {
        a - b
    }
}

/// Order-exact multiplication; see [`fadd1`].
#[inline(always)]
fn fmul1(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        a
    } else if b.is_nan() {
        b
    } else {
        a * b
    }
}

export!(cdecl, rw_d854e0(
    obj: u32,
    a1: u32,
    fx: u32,
    fy: u32,
    out_a: u32,
    out_b: u32,
    a6: u32,
    out_flag: u32,
    a8: u32,
    scale: u32,
) -> u32 {
    /// Negative half turn: lower edge of the wrapped angle range.
    const NEG_PI: f32 = f32::from_bits(0xC049_0FDB);
    /// Full turn: wrap step of the angle loops.
    const TAU: f32 = f32::from_bits(0x40C9_0FDB);
    /// Positive half turn: upper edge of the wrapped angle range.
    const PI: f32 = f32::from_bits(0x4049_0FDB);
    /// Reach limit that arms the flag write.
    const REACH: f32 = f32::from_bits(0x41A0_0000);
    /// Turn size that sets the flag.
    const TURN_FLAG: f32 = f32::from_bits(0x3F33_3333);
    /// Mirror factor applied to both outputs on the mirrored path, read
    /// from the image like the original: written as `* -1.0` the compiler
    /// folds it into a sign flip, which disagrees with `mulss` on NaN signs.
    const MIRROR_CELL: u32 = 0x00FE_8D94;
    /// Blend weights forwarded to the blend call.
    const BL_W0: u32 = 0x3ECC_CCCD;
    const BL_W1: u32 = 0x3F99_999A;
    const BL_ONE: u32 = 0x3F80_0000;
    /// Mode map for the follow-up call, indexed by mode byte minus one.
    const MODE_MAP: [u8; 7] = [0, 1, 2, 2, 1, 1, 1];

    #[inline(always)]
    fn read_f32(addr: u32) -> f32 {
        unsafe { (addr as *const f32).read() }
    }
    /// Call through the slot's own function table (slot 0xEC), like the
    /// original: object in ECX, output struct pointer on the stack.
    #[inline(always)]
    fn table_call(obj: u32, out: *mut u32) -> u32 {
        let vt = unsafe { (obj as *const u32).read() };
        let tgt = unsafe { ((vt + 0xEC) as *const u32).read() };
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(tgt as usize) };
        f(obj, out as u32)
    }

    // Scratch slots standing in for the original's frame words.
    let mut slot_a1 = 0u32;
    let mut slot_a2 = 0u32;
    let mut slot_t10 = 0u32;
    let mut slot_norm = 0u32;
    let mut slot_vt = [0u32; 3];

    unsafe { (out_flag as *mut u8).write(0) };
    let mirrored = unsafe { ((obj + 0xE73) as *const u8).read() } & 1 != 0;
    let anchor = unsafe { ((obj + 0x20) as *const u32).read() };
    let (dx, dy) = if mirrored {
        if anchor != 0 {
            (-read_f32(anchor + 0x10), -read_f32(anchor + 0x14))
        } else {
            let v = unsafe { ((obj + 0x1C) as *const u32).read() };
            let s = f32::from_bits(callee_cdecl!(1, u32, v));
            let c = f32::from_bits(callee_cdecl!(2, u32, v));
            (s, -c)
        }
    } else if anchor != 0 {
        (read_f32(anchor + 0x10), read_f32(anchor + 0x14))
    } else {
        let v = unsafe { ((obj + 0x1C) as *const u32).read() };
        let s = f32::from_bits(callee_cdecl!(1, u32, v));
        let c = f32::from_bits(callee_cdecl!(2, u32, v));
        (-s, c)
    };
    slot_norm = dy.to_bits();
    let len = fadd1(dy * dy, dx * dx).sqrt();
    let s8: f32;
    if !(len <= 0.0) {
        let rcp = 1.0 / len;
        s8 = fmul1(rcp, dx);
        slot_norm = fmul1(dy, rcp).to_bits();
    } else {
        s8 = 1.0;
    }
    // The far read faults first when no anchor is attached, as observed.
    // Volatile: a plain load here may be sunk past the NaN early-return in
    // fsub1 below, which would skip the fault the original takes.
    let p34 = unsafe { ((anchor + 0x34) as *const f32).read_volatile() };
    let p30 = unsafe { ((anchor + 0x30) as *const f32).read_volatile() };
    let dy1 = fsub1(f32::from_bits(fy), p34);
    let dx1 = fsub1(f32::from_bits(fx), p30);
    let a1v = callee_cdecl!(3, f32, dx1.to_bits(), dy1.to_bits());
    slot_a1 = a1v.to_bits();
    slot_t10 = a1v.to_bits();
    let a2v = callee_cdecl!(3, f32, s8.to_bits(), slot_norm);
    slot_a2 = a2v.to_bits();
    let probe = callee_cdecl!(
        4,
        u32,
        obj,
        slot_a1,
        slot_a2,
        &mut slot_norm as *mut u32 as u32,
        &mut slot_a1 as *mut u32 as u32
    );
    if probe == 0 {
        let mode = unsafe { ((obj + 0xE70) as *const i8).read() } as i32 - 1;
        let (mx, my, mz): (u32, u32, u32) = if (mode as u32) > 6 {
            (0, 0, 1)
        } else {
            match MODE_MAP[mode as usize] {
                0 => (0, 0, 0),
                1 => (1, 1, 2),
                _ => (0, 0, 1),
            }
        };
        let r = callee_cdecl!(
            5, f32, obj, a1, slot_a1, slot_a2, BL_ONE, mz, my, mx, a8
        );
        let mut d = fsub1(f32::from_bits(slot_a1), f32::from_bits(slot_a2));
        while NEG_PI > d {
            d += TAU;
        }
        while d > PI {
            d -= TAU;
        }
        let flag01 = if 0.0 > d { 1u32 } else { 0u32 };
        let r2 = callee_cdecl!(6, f32, obj, flag01, slot_a2, r.to_bits());
        let mut d2 = fsub1(r2, f32::from_bits(slot_a2));
        while NEG_PI > d2 {
            d2 += TAU;
        }
        while d2 > PI {
            d2 -= TAU;
        }
        slot_t10 = d2.to_bits();
        slot_norm = d2.to_bits();
        let v1 = table_call(obj, slot_vt.as_mut_ptr());
        let v10 = read_f32(v1);
        let v14 = read_f32(v1 + 4);
        let v18 = read_f32(v1 + 8);
        let n3 = fadd1(fadd1(v10 * v10, v14 * v14), v18 * v18).sqrt();
        if n3 > REACH {
            let mut m = f32::from_bits(slot_norm);
            if 0.0 > m {
                m = -m;
            }
            if m > TURN_FLAG {
                unsafe { (out_flag as *mut u8).write(1) };
            }
        }
        let a3 = callee_cdecl!(3, f32, dx1.to_bits(), dy1.to_bits());
        let diff = fsub1(a3, f32::from_bits(slot_a2));
        let b1 = callee_cdecl!(
            8,
            f32,
            diff.to_bits(),
            BL_W0,
            BL_W1,
            BL_W0,
            BL_ONE
        );
        // The scaled turn is stored back into the wrapped-angle slot.
        slot_t10 = fmul1(b1, f32::from_bits(scale)).to_bits();
    }
    // On the probe-taken path the slot keeps the first probe angle.
    callee_cdecl!(
        9,
        u32,
        obj,
        &mut slot_norm as *mut u32 as u32,
        &mut slot_a2 as *mut u32 as u32,
        &mut slot_a1 as *mut u32 as u32
    );
    let v2 = table_call(obj, slot_vt.as_mut_ptr());
    let w40 = read_f32(v2 + 4);
    let w00 = read_f32(v2);
    let w80 = read_f32(v2 + 8);
    let n4 = fadd1(fadd1(w40 * w40, w00 * w00), w80 * w80).sqrt();
    callee_cdecl!(
        10,
        u32,
        slot_t10,
        n4.to_bits(),
        a6,
        out_b,
        obj,
        &mut slot_a2 as *mut u32 as u32
    );
    let norm = f32::from_bits(slot_norm);
    unsafe { (out_a as *mut u32).write(norm.to_bits()) };
    // First float is the reach length stashed beside the slots, not the turn.
    let answer = callee_cdecl!(
        11,
        u32,
        obj,
        n4.to_bits(),
        norm.to_bits(),
        out_flag
    );
    if mirrored {
        let mirror = read_f32(relocated(MIRROR_CELL));
        let oa = unsafe { (out_a as *const f32).read() } * mirror;
        unsafe { (out_a as *mut f32).write(oa) };
        let ob = unsafe { (out_b as *const f32).read() } * mirror;
        unsafe { (out_b as *mut f32).write(ob) };
        out_a
    } else {
        answer
    }
});

