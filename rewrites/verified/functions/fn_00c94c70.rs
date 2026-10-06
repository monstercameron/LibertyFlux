// original: 0x00c94c70 task_pose_update (proposed)

use lf_checker_rt::global;

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd16(a: u32) -> u16 {
    unsafe { (a as *const u16).read_unaligned() }
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
unsafe fn rdf(a: u32) -> f32 {
    unsafe { f32::from_bits(rd32(a)) }
}

#[inline(always)]
unsafe fn wrf(a: u32, v: f32) {
    unsafe { wr32(a, v.to_bits()) }
}

#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    let a = core::hint::black_box(a);
    let b = core::hint::black_box(b);
    a + b
}

#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    let a = core::hint::black_box(a);
    let b = core::hint::black_box(b);
    a * b
}

#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    let a = core::hint::black_box(a);
    let b = core::hint::black_box(b);
    a - b
}

// ---------------------------------------------------------------------------
// Layout and calibration constants.
// ---------------------------------------------------------------------------

/// Offset of the work object from the task object.
const TASK_WORK: u32 = 0x40;
/// Ready flag on the work object: the function exits unless nonzero.
const WORK_READY: u32 = 0x219;
/// Cached selector on the work object (-1 keeps the shared one).
const WORK_SELECTOR: u32 = 0xEE0;
/// Pose matrix pointer on the work object.
const WORK_MATRIX: u32 = 0x20;
/// Signed 16-bit row index on a resolved object.
const OBJ_ROW_INDEX: u32 = 0x2E;
/// Output pose quad (x, y, z, w) on resolved objects.
const POSE_X: u32 = 0x30;
const POSE_Y: u32 = 0x34;
const POSE_Z: u32 = 0x38;
const POSE_W: u32 = 0x3C;
/// Virtual slot used for the reselect call.
const VT_RESELECT: u32 = 0x144;
/// Virtual slot used for the pose-commit call.
const VT_COMMIT: u32 = 0x08;

// Shared-state globals (file VAs; resolved through the worker mapping).
const G_GATE_A: u32 = 0x011F7060;
const G_STAMP_A: u32 = 0x012088B4;
const G_STAMP_B: u32 = 0x00F1C040;
const G_GATE_B: u32 = 0x01037720;
const G_GATE_B_QUIT: u32 = 0x12;
const G_SHARED_SEL: u32 = 0x01050B90;
const G_FACTORY: u32 = 0x01632C60;
const G_ROW_COUNT: u32 = 0x012FA4F4;
const G_FLAG: u32 = 0x01050B8D;
const G_IN_X: u32 = 0x01050C10;
const G_IN_Y: u32 = 0x01050C14;
const G_IN_Z: u32 = 0x01050C18;
const G_B1_X: u32 = 0x01050C20;
const G_B1_Y: u32 = 0x01050C24;
const G_B1_Z: u32 = 0x01050C28;
const G_B2_X: u32 = 0x01050C30;
const G_B2_Y: u32 = 0x01050C34;
const G_B2_Z: u32 = 0x01050C38;
const G_B3_X: u32 = 0x01050C40;
const G_B3_Y: u32 = 0x01050C44;
const G_B3_Z: u32 = 0x01050C48;
const G_B4_X: u32 = 0x01050C50;
const G_B4_Y: u32 = 0x01050C54;
const G_B4_Z: u32 = 0x01050C58;
const G_SCALE: u32 = 0x01050C98;

// Lookup keys, one pair per region.
const KEY_R1_A: u32 = 0x4C3;
const KEY_R1_B: u32 = 0x3B12;
const KEY_R1_C: u32 = 0x3B10;
const KEY_R1_D: u32 = 0x3B11;
const KEY_R2_A: u32 = 0x4D0;
const KEY_R2_B: u32 = 0x3AF5;
const KEY_R2_C: u32 = 0x3AF3;
const KEY_R2_D: u32 = 0x3AF4;
const KEY_R3_A: u32 = 0x4C0;
const KEY_R3_B: u32 = 0x3B21;
const KEY_R4_A: u32 = 0x4C7;
const KEY_R4_B: u32 = 0x3B04;

// Callee ids in the checker contract.
const C_GATE: u32 = 1;
const C_LOOKUP: u32 = 9;
const C_RESOLVE: u32 = 10;
const C_FETCH: u32 = 11;
const C_MAKE: u32 = 3;
const C_DISCARD: u32 = 4;
const C_ATTACH: u32 = 5;
const C_PREPARE: u32 = 6;
const C_FINISH: u32 = 8;
const C_NORMALISE: u32 = 12;

/// Value the harness gives the one uninitialised stack slot the original
/// reads (contract `stack_fill` 0). The original copies it into every output
/// pose's w lane; modelling it as the harness-defined fill keeps both sides
/// identical. Listed as narrowed in the result.
const FILL_W: f32 = 0.0;

#[inline(always)]
unsafe fn g32(va: u32) -> u32 {
    unsafe { global::<u32>(va).read_unaligned() }
}

#[inline(always)]
unsafe fn set_g32(va: u32, v: u32) {
    unsafe { global::<u32>(va).write_unaligned(v) }
}

#[inline(always)]
unsafe fn gf(va: u32) -> f32 {
    unsafe { f32::from_bits(g32(va)) }
}

#[inline(always)]
unsafe fn flag_on() -> bool {
    unsafe { rd8(global::<u8>(G_FLAG) as u32) != 0 }
}

/// The reselect virtual call: slot VT_RESELECT on the work object with (-1).
#[inline(always)]
unsafe fn v_reselect(work: u32) {
    unsafe {
        let vt = rd32(work);
        let slot = rd32(vt.wrapping_add(VT_RESELECT));
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        f(work, 0xFFFF_FFFF);
    }
}

/// The pose-commit virtual call: slot VT_COMMIT on the row object with the
/// pose vector pointer, 0 and 1.
#[inline(always)]
unsafe fn v_commit(obj: u32, vec: u32) {
    unsafe {
        let vt = rd32(obj);
        let slot = rd32(vt.wrapping_add(VT_COMMIT));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        f(obj, vec, 0, 1);
    }
}

/// Pose block shape A (regions 1 and 3): transform the direction banks by the
/// matrix at `mat`, add the translation row, and store the pose quad at `dst`.
/// Operation order is the original's.
#[inline(always)]
unsafe fn pose_block_a(mat: u32, vx: f32, vy: f32, vz: f32, dst: u32, w_fill: f32) {
    unsafe {
        let m00 = rdf(mat);
        let m10 = rdf(mat + 0x10);
        let m14 = rdf(mat + 0x14);
        let m18 = rdf(mat + 0x18);
        let t0 = fmul(m00, vx);
        let mut acc0 = fmul(m10, vy);
        let mut acc1 = fmul(m14, vy);
        let mut acc2 = fmul(m18, vy);
        acc0 = fadd(acc0, t0);
        let t1 = fmul(rdf(mat + 0x20), vz);
        acc0 = fadd(acc0, t1);
        let t2 = fmul(rdf(mat + 4), vx);
        let mut out1 = rdf(mat + 0x34);
        acc0 = fadd(acc0, rdf(mat + 0x30));
        acc1 = fadd(acc1, t2);
        let t3 = fmul(rdf(mat + 0x24), vz);
        acc1 = fadd(acc1, t3);
        let t4 = fmul(rdf(mat + 8), vx);
        out1 = fadd(out1, acc1);
        acc2 = fadd(acc2, t4);
        let t5 = fmul(rdf(mat + 0x28), vz);
        acc2 = fadd(acc2, t5);
        let mut out2 = rdf(mat + 0x38);
        wrf(dst + POSE_X, acc0);
        wrf(dst + POSE_Y, out1);
        out2 = fadd(out2, acc2);
        wrf(dst + POSE_Z, out2);
        wrf(dst + POSE_W, w_fill);
    }
}

/// Pose block shape B (regions 2 and 4): same transform with the translation
/// row added to the first lane from the other side. Order is the original's.
#[inline(always)]
unsafe fn pose_block_b(mat: u32, vx: f32, vy: f32, vz: f32, dst: u32, w_fill: f32) {
    unsafe {
        let m00 = rdf(mat);
        let m10 = rdf(mat + 0x10);
        let m14 = rdf(mat + 0x14);
        let m18 = rdf(mat + 0x18);
        let t0 = fmul(m00, vx);
        let mut acc0 = fmul(m10, vy);
        let mut acc1 = fmul(m14, vy);
        let mut acc2 = fmul(m18, vy);
        acc0 = fadd(acc0, t0);
        let t1 = fmul(rdf(mat + 0x20), vz);
        acc0 = fadd(acc0, t1);
        let t2 = fmul(rdf(mat + 4), vx);
        let mut out1 = rdf(mat + 0x34);
        acc1 = fadd(acc1, t2);
        let t3 = fmul(rdf(mat + 0x24), vz);
        acc1 = fadd(acc1, t3);
        let t4 = fmul(rdf(mat + 8), vx);
        out1 = fadd(out1, acc1);
        acc2 = fadd(acc2, t4);
        let t5 = fmul(rdf(mat + 0x28), vz);
        let mut out0 = rdf(mat + 0x30);
        out0 = fadd(out0, acc0);
        acc2 = fadd(acc2, t5);
        let mut out2 = rdf(mat + 0x38);
        wrf(dst + POSE_Y, out1);
        wrf(dst + POSE_X, out0);
        out2 = fadd(out2, acc2);
        wrf(dst + POSE_Z, out2);
        wrf(dst + POSE_W, w_fill);
    }
}

/// One lookup round: key to id on `owner`, id resolved, object fetched.
/// Returns `None` when the resolver reports -1.
#[inline(always)]
unsafe fn lookup(owner: u32, key: u32) -> Option<u32> {
    unsafe {
        let id = lf_checker_rt::callee_cdecl!(C_LOOKUP, u32, key);
        let h = lf_checker_rt::callee_cdecl!(C_RESOLVE, u32, id);
        if h == 0xFFFF_FFFF {
            return None;
        }
        let obj = lf_checker_rt::callee_thiscall!(C_FETCH, u32, owner, h);
        Some(obj)
    }
}

/// Shared body. `key_first` is the region-1 first lookup key; the mutant
/// passes it off by one to prove the contract observes call arguments.
unsafe fn body(this: u32, key_first: u32) {
    unsafe {
        let work = rd32(this.wrapping_add(TASK_WORK));
        if work == 0 {
            return;
        }
        if rd8(work.wrapping_add(WORK_READY)) == 0 {
            return;
        }
        if g32(G_GATE_A) == 1 {
            return;
        }
        if g32(G_STAMP_A) != g32(G_STAMP_B) {
            return;
        }
        if g32(G_GATE_B) == G_GATE_B_QUIT {
            return;
        }
        let cached = rd32(work.wrapping_add(WORK_SELECTOR));
        let mut sel = g32(G_SHARED_SEL);
        // The cached selector is signed: -1 keeps the shared one (cmovne).
        if cached != 0xFFFF_FFFF {
            sel = cached;
        }
        set_g32(G_SHARED_SEL, sel);
        let gate_ok = lf_checker_rt::callee_cdecl!(C_GATE, u32,) & 0xFF;
        let sel_v: u32;
        if gate_ok != 0 {
            sel_v = g32(G_SHARED_SEL);
        } else {
            v_reselect(work);
            sel_v = cached;
        }
        if sel_v == 0xFFFF_FFFF {
            set_g32(G_SHARED_SEL, 0xFFFF_FFFF);
            return;
        }
        let factory = g32(G_FACTORY);
        let obj = lf_checker_rt::callee_thiscall!(C_MAKE, u32, factory, sel_v);
        if obj == 0 {
            v_reselect(work);
            set_g32(G_SHARED_SEL, 0xFFFF_FFFF);
            return;
        }
        // Signed row index (movsx) compared against the row count for equality.
        let row_index = rd16(obj.wrapping_add(OBJ_ROW_INDEX)) as i16 as i32;
        if (row_index as u32) != g32(G_ROW_COUNT) {
            v_reselect(work);
            lf_checker_rt::callee_thiscall!(C_DISCARD, u32, obj);
            set_g32(G_SHARED_SEL, 0xFFFF_FFFF);
            return;
        }
        lf_checker_rt::callee_thiscall!(C_ATTACH, u32, obj, work);
        lf_checker_rt::callee_thiscall!(C_PREPARE, u32, work);

        // Entry transform: input banks by the work matrix, plus translation.
        let mat = rd32(work.wrapping_add(WORK_MATRIX));
        let ix = gf(G_IN_X);
        let iy = gf(G_IN_Y);
        let iz = gf(G_IN_Z);
        let t0 = fmul(ix, rdf(mat));
        let mut r0 = fmul(rdf(mat + 0x10), iy);
        let mut r1 = fmul(rdf(mat + 4), ix);
        let mut r2 = fmul(rdf(mat + 8), ix);
        r0 = fadd(r0, t0);
        let t1 = fmul(rdf(mat + 0x20), iz);
        r0 = fadd(r0, t1);
        let t2 = fmul(rdf(mat + 0x14), iy);
        r1 = fadd(r1, t2);
        let t3 = fmul(rdf(mat + 0x24), iz);
        r1 = fadd(r1, t3);
        let t4 = fmul(rdf(mat + 0x18), iy);
        r2 = fadd(r2, t4);
        let t5 = fmul(rdf(mat + 0x28), iz);
        r2 = fadd(r2, t5);
        let mut pose = [r0, r1, r2, FILL_W];
        r0 = fadd(r0, rdf(mat + 0x30));
        pose[0] = r0;
        let mut q1 = rdf(mat + 0x34);
        q1 = fadd(q1, r1);
        pose[1] = q1;
        let mut q2 = rdf(mat + 0x38);
        q2 = fadd(q2, r2);
        pose[2] = q2;
        v_commit(obj, pose.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(C_FINISH, u32, obj);



        // Region 1.
        {
            let first = lookup(work, key_first);
            if first.is_none() {
                return region234(obj, work);
            }
            let base = first.unwrap_or(0);
            let second = lookup(obj, KEY_R1_B);
            if second.is_none() {
                return region234(obj, work);
            }
            let row = second.unwrap_or(0);
            if flag_on() {
                pose_block_a(base, gf(G_B1_X), gf(G_B1_Y), gf(G_B1_Z), row, FILL_W);
            }
            let third = lookup(obj, KEY_R1_C);
            if third.is_none() {
                return region234(obj, work);
            }
            let other = third.unwrap_or(0);
            let mut delta = [
                fsub(rdf(other + POSE_X), rdf(row + POSE_X)),
                fsub(rdf(other + POSE_Y), rdf(row + POSE_Y)),
                fsub(rdf(other + POSE_Z), rdf(row + POSE_Z)),
            ];
            lf_checker_rt::callee_thiscall!(C_NORMALISE, u32, delta.as_mut_ptr() as u32);
            let k = gf(G_SCALE);
            let sx = fmul(delta[0], k);
            let sy = fmul(delta[1], k);
            let sz = fmul(delta[2], k);
            let fourth = lookup(obj, KEY_R1_D);
            if fourth.is_none() {
                return region234(obj, work);
            }
            let out = fourth.unwrap_or(0);
            let mut oz = rdf(row + POSE_Z);
            oz = fadd(oz, sz);
            let mut ox = sx;
            ox = fadd(ox, rdf(row + POSE_X));
            let mut oy = rdf(row + POSE_Y);
            oy = fadd(oy, sy);
            wrf(out + POSE_Z, oz);
            wrf(out + POSE_X, ox);
            wrf(out + POSE_Y, oy);
            wrf(out + POSE_W, FILL_W);
        }
        region234(obj, work);
    }
}

/// Regions 2 to 4 (also the landing path when region 1 bails early).
unsafe fn region234(obj: u32, work: u32) {
    unsafe {

        // Region 2.
        {
            let first = lookup(work, KEY_R2_A);
            if first.is_none() {
                return region34(obj, work);
            }
            let base = first.unwrap_or(0);
            let second = lookup(obj, KEY_R2_B);
            if second.is_none() {
                return region34(obj, work);
            }
            let row = second.unwrap_or(0);
            if flag_on() {
                pose_block_b(base, gf(G_B2_X), gf(G_B2_Y), gf(G_B2_Z), row, FILL_W);
            }
            let third = lookup(obj, KEY_R2_C);
            if third.is_none() {
                return region34(obj, work);
            }
            let other = third.unwrap_or(0);
            let mut delta = [
                fsub(rdf(other + POSE_X), rdf(row + POSE_X)),
                fsub(rdf(other + POSE_Y), rdf(row + POSE_Y)),
                fsub(rdf(other + POSE_Z), rdf(row + POSE_Z)),
            ];
            lf_checker_rt::callee_thiscall!(C_NORMALISE, u32, delta.as_mut_ptr() as u32);
            let k = gf(G_SCALE);
            let sx = fmul(delta[0], k);
            let sy = fmul(delta[1], k);
            let sz = fmul(delta[2], k);
            let fourth = lookup(obj, KEY_R2_D);
            if fourth.is_none() {
                return region34(obj, work);
            }
            let out = fourth.unwrap_or(0);
            let mut ox = rdf(row + POSE_X);
            ox = fadd(ox, sx);
            let mut oy = sy;
            oy = fadd(oy, rdf(row + POSE_Y));
            let mut oz = sz;
            oz = fadd(oz, rdf(row + POSE_Z));
            wrf(out + POSE_X, ox);
            wrf(out + POSE_Y, oy);
            wrf(out + POSE_W, FILL_W);
            wrf(out + POSE_Z, oz);
        }
        region34(obj, work);
    }
}

/// Regions 3 and 4.
unsafe fn region34(obj: u32, work: u32) {
    unsafe {

        // Region 3.
        {
            let first = lookup(work, KEY_R3_A);
            if first.is_none() {
                return region4(obj, work);
            }
            let base = first.unwrap_or(0);
            let second = lookup(obj, KEY_R3_B);
            if second.is_none() {
                return region4(obj, work);
            }
            let row = second.unwrap_or(0);
            if !flag_on() {
                return region4(obj, work);
            }
            pose_block_a(base, gf(G_B3_X), gf(G_B3_Y), gf(G_B3_Z), row, FILL_W);
        }
        region4(obj, work);
    }
}

/// Region 4.
unsafe fn region4(obj: u32, work: u32) {
    unsafe {

        let first = lookup(work, KEY_R4_A);
        if first.is_none() {
            return;
        }
        let base = first.unwrap_or(0);
        let second = lookup(obj, KEY_R4_B);
        if second.is_none() {
            return;
        }
        let row = second.unwrap_or(0);
        if !flag_on() {
            return;
        }
        pose_block_b(base, gf(G_B4_X), gf(G_B4_Y), gf(G_B4_Z), row, FILL_W);
    }
}

/// Ped task pose-update step: resolve the task's row object, transform the
/// shared direction banks by its matrices, and propagate pose quads through
/// four lookup regions.
///
/// `this` is the task object; its work object sits at `+0x40`. The work
/// object carries a ready flag byte at `+0x219`, a cached selector dword at
/// `+0xEE0` (-1 keeps the shared selector) and a pose matrix pointer at
/// `+0x20`. The row object made by the factory has a vtable at `+0` and a
/// signed 16-bit row index at `+0x2E`. Every object returned by the fetcher
/// is read as a matrix (`+0x00..+0x38`) and written as a pose quad
/// (`+0x30..+0x3C`, x/y/z/w).
///
/// Behaviour: five shared-state gates (ready flag, two stamps that must
/// agree, two quit sentinels) exit early, as does a null work object. The
/// cached selector, when not -1, replaces the shared one; a gate call then
/// decides between a reselect virtual call (which leaves the cached value in
/// place) and re-reading the shared selector. A -1 selector, a null row
/// object, or a row index that differs from the row count exits after
/// storing -1 to the shared selector. Otherwise the row is attached and
/// prepared, the input banks are transformed by the work matrix plus its
/// translation row, the result is committed through virtual slot 2 with
/// arguments (vector, 0, 1), and four regions run: each looks a key up to an
/// id, resolves the id (a -1 answer skips to the next region), fetches the
/// object, optionally writes a transformed pose when the shared flag byte is
/// set (regions 3 and 4 end the region instead when it is clear), and, in
/// regions 1 and 2, normalises a pose delta and blends a scaled copy onto a
/// fourth object.
///
/// Edge cases: every early exit above; a -1 resolver answer at any of the
/// twelve lookups; the flag byte clear; a null row object. The original
/// copies one uninitialised stack dword (its frame slot for the w lane) into
/// every output pose's w lane; the contract defines that slot as 0
/// (`stack_fill`) and this rewrite uses 0 there. All branches are equality
/// tests; the compared values that are signed are the cached selector (the
/// -1 sentinel) and the row index (sign-extended before the equality test
/// against the row count). Float operation order is the original's, pinned
/// through `black_box` helpers.
///
/// Original: thiscall, one object pointer, no stack arguments, void return.
/// The return register on the early paths is the untouched incoming eax, so
/// the contract compares no return channel.
lf_checker_rt::export!(thiscall, rw_00c94c70(this: u32) -> u32 {
    unsafe {
        body(this, KEY_R1_A);
    }
    0
});
