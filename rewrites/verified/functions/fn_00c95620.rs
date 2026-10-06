// original: 0x00c95620 task_target_solve (proposed)

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
const POSE_X: u32 = 0x30;
const POSE_Y: u32 = 0x34;
const POSE_Z: u32 = 0x38;
const POSE_W: u32 = 0x3C;

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
// Second function: ped task target solve (0x00C95620).
// ---------------------------------------------------------------------------

// Layout constants for the target solve.
const T2_WORK: u32 = 0x40;
const T2_ROW_INDEX: u32 = 0x2E;
const T2_MATRIX: u32 = 0x20;
const T2_INFO: u32 = 0xAB0;
const T2_INFO_MODE: u32 = 0x28;
const T2_TAB_BASE: u32 = 0x01295CD8;
const T2_TAB_SLOT: u32 = 0x38;
const T2_VT_SLOT_A0: u32 = 0xA0;
const T2_FLAG_DEFAULT: u32 = 0x46;
const T2_FLAG_MODE2: u32 = 0x4E;
const T2_FLAG_MODE4: u32 = 0xC6;
const T2_SEL_DEFAULT: u32 = 6;
const T2_SEL_MODE2: u32 = 0x0E;
const T2_SEL_MODE4: u32 = 0x86;
const T2_LOOP_BIT: u32 = 0x40;
const T2_ABS_MASK: u32 = 0x7FFF_FFFF;
const T2_CLASS_MASK: u32 = 0x3C0;
const T2_CLASS_WANT: u32 = 0xC0;
const T2_ENTRY_STRIDE: u32 = 0x60;
const T2_ENTRY_COUNT: i32 = 15;

// Callee ids in the fn2 contract.
const D_TABLE: u32 = 1;
const D_COMMIT2: u32 = 2;
const D_SOLVE: u32 = 3;
const D_FETCH: u32 = 4;
const D_SCATTER: u32 = 5;
const D_OPEN: u32 = 6;
const D_CLASSIFY: u32 = 7;
const D_COOKIE: u32 = 8;

/// Tabulated row call shared by both table sites: load the entry, its vtable
/// slot, and call it with the entry as this and one argument.
#[inline(always)]
unsafe fn t2_table_call(entry: u32, arg: u32) -> u32 {
    unsafe {
        let vt = rd32(entry);
        let slot = rd32(vt.wrapping_add(T2_TAB_SLOT));
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        f(entry, arg)
    }
}

/// The slot-A0 virtual call on the work object: six stack arguments.
#[inline(always)]
unsafe fn t2_commit2(work: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        let vt = rd32(work);
        let slot = rd32(vt.wrapping_add(T2_VT_SLOT_A0));
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        f(work, a0, a1, a2, a3, a4, a5)
    }
}

/// comiss + jbe, taken exactly when `!(x > y)` (unordered counts as taken).
#[inline(always)]
fn comiss_jbe(x: f32, y: f32) -> bool {
    let x = core::hint::black_box(x);
    let y = core::hint::black_box(y);
    !(x > y)
}

/// Shared body. `invert_thresh` flips every threshold-lane store decision
/// (mutant only): lanes that moved more than 0.01 keep their old value and
/// lanes within tolerance are overwritten.
unsafe fn t2_body(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    invert_thresh: bool,
) -> u32 {
    unsafe {
        let work = rd32(this.wrapping_add(T2_WORK));
        // Table call 1.
        let idx1 = rd16(work.wrapping_add(T2_ROW_INDEX)) as i16 as i32;
        let tab = global::<u32>(T2_TAB_BASE) as u32;
        let ent1 = rd32(tab.wrapping_add((idx1 as u32).wrapping_mul(4)));
        let ans1 = t2_table_call(ent1, 0);
        let o1 = lf_checker_rt::callee_thiscall!(D_FETCH, u32, work, ans1);
        let s30 = rdf(o1 + POSE_X);
        let s48 = rdf(o1 + POSE_Y);
        // Second fetch + matrix copy.
        let o2 = lf_checker_rt::callee_thiscall!(D_FETCH, u32, work, a1);
        let m00 = rdf(o2);
        let m04 = rdf(o2 + 4);
        let m08 = rdf(o2 + 8);
        let m10 = rdf(o2 + 0x10);
        let m14 = rdf(o2 + 0x14);
        let m18 = rdf(o2 + 0x18);
        let m20 = rdf(o2 + 0x20);
        let m24 = rdf(o2 + 0x24);
        let m28 = rdf(o2 + 0x28);
        let s20 = rdf(o2 + POSE_X);
        let s1c = rdf(o2 + POSE_Y);
        let s50 = rdf(o2 + POSE_Z);
        // Third fetch.
        let o3 = lf_checker_rt::callee_thiscall!(D_FETCH, u32, work, a0);
        let s90 = rdf(o3 + POSE_X);
        let s88 = rdf(o3 + POSE_Y);
        let s68 = rdf(o3 + POSE_Z);
        // Float output through the a4 pointer.
        let mat2 = rd32(work.wrapping_add(T2_MATRIX));
        let t = fsub(rdf(mat2 + POSE_Z), s68);
        let r = fsub(1.0, t);
        wrf(a4, r);
        // Selector from the info block.
        let info = rd32(work.wrapping_add(T2_INFO));
        let mut sel = T2_SEL_DEFAULT;
        if info != 0 {
            let f = (rd32(info + T2_INFO_MODE) >> 6) & 0xF;
            if f == 2 {
                sel = T2_SEL_MODE2;
            } else if f == 4 {
                sel = T2_SEL_MODE4;
            }
        }
        // Solve call: two frame vectors plus the selector.
        let mut v0 = [s30, s48, s50, 0.0];
        let mut v1 = [s20, s1c, s50, 0.0];
        let ok = lf_checker_rt::callee_thiscall!(
            D_SOLVE, u32, this, v0.as_mut_ptr() as u32, v1.as_mut_ptr() as u32, sel
        );
        // The callee's status byte is modelled as untouched (its initial 0:
        // see the doc comment); only the answer bit matters here.
        let mut c: u32 = 0;
        if (ok & 0xFF) == 0 {
            c = 1;
        }
        let mut s30b: f32 = 1.0;
        if c != 0 {
            s30b = 0.0;
        }
        // Flag dword from the same info block.
        let mut fd = T2_FLAG_DEFAULT;
        if info != 0 {
            let f = (rd32(info + T2_INFO_MODE) >> 6) & 0xF;
            if f == 2 {
                fd = T2_FLAG_MODE2;
            } else if f == 4 {
                fd = T2_FLAG_MODE4;
            }
        }
        // Blend.
        let mut b20 = fadd(s20, s90);
        let mut b1c = fadd(s1c, s88);
        let mut b48 = fadd(s50, s68);
        b20 = fmul(b20, 0.5);
        b1c = fmul(b1c, 0.5);
        b48 = fmul(b48, 0.5);
        // Entry array + scatter inputs that outlive one loop trip.
        let mut mat_copy = [
            m00, m04, m08, 0.0, m10, m14, m18, 0.0, m20, m24, m28, 0.0, s20, s1c, s50, 0.0,
        ];
        let mut w88 = [
            s88.to_bits(),
            this,
            s90.to_bits(),
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        // Outer loop: at most two trips; the first trip clears the loop bit.
        let mut flag = fd;
        loop {
            // Fill fifteen entries from the shared vector.
            let gx = gf(0x01B4B320);
            let gy = gf(0x01B4B324);
            let gz = gf(0x01B4B328);
            let mut ent = [0u32; 360];
            for j in 0..T2_ENTRY_COUNT {
                let b = (j as usize) * 24;
                ent[b] = 0;
                ent[b + 4] = gx.to_bits();
                ent[b + 5] = gy.to_bits();
                ent[b + 6] = gz.to_bits();
                ent[b + 8] = gx.to_bits();
                ent[b + 9] = gy.to_bits();
                ent[b + 10] = gz.to_bits();
                ent[b + 12] = gx.to_bits();
                ent[b + 13] = gy.to_bits();
                ent[b + 14] = gz.to_bits();
                ent[b + 16] = 0;
                ent[b + 17] = 0;
                ent[b + 18] = 0;
                ent[b + 19] = 0xFFFF;
                // Byte 0 at +0x50 and word 0 at +0x52 sit in a zero word.
                ent[b + 20] = 0;
            }
            // Table call 2.
            let idx2 = rd16(work.wrapping_add(T2_ROW_INDEX)) as i16 as i32;
            let ent2 = rd32(tab.wrapping_add((idx2 as u32).wrapping_mul(4)));
            let ans2 = t2_table_call(ent2, 0x12);
            // Threshold group: 0x150 on equality, 0x160 otherwise.
            let (x2, x3);
            if ans2 == a1 {
                let d0 = fsub(rdf(this + 0x150), b20);
                let a0f = f32::from_bits(d0.to_bits() & T2_ABS_MASK);
                if (!comiss_jbe(a0f, 0.01)) != invert_thresh {
                    wrf(this + 0x150, b20);
                }
                let d1 = fsub(rdf(this + 0x154), b1c);
                let a1f = f32::from_bits(d1.to_bits() & T2_ABS_MASK);
                if (!comiss_jbe(a1f, 0.01)) != invert_thresh {
                    wrf(this + 0x154, b1c);
                }
                wrf(this + 0x158, b48);
                x2 = rdf(this + 0x150);
                x3 = rdf(this + 0x154);
            } else {
                let d0 = fsub(rdf(this + 0x160), b20);
                let a0f = f32::from_bits(d0.to_bits() & T2_ABS_MASK);
                if (!comiss_jbe(a0f, 0.01)) != invert_thresh {
                    wrf(this + 0x160, b20);
                }
                let d1 = fsub(rdf(this + 0x164), b1c);
                let a1f = f32::from_bits(d1.to_bits() & T2_ABS_MASK);
                if (!comiss_jbe(a1f, 0.01)) != invert_thresh {
                    wrf(this + 0x164, b1c);
                }
                wrf(this + 0x168, b48);
                x2 = rdf(this + 0x160);
                x3 = rdf(this + 0x164);
            }
            let x1 = b48;
            let mut x0 = x1;
            x0 = fadd(x0, 0.4);
            let x1b = fsub(x1, s30b);
            w88[6] = x2.to_bits();
            w88[7] = x3.to_bits();
            w88[8] = x0.to_bits();
            let ans_c = t2_commit2(work, flag, 0xFFFF_FFFF, 7, 1, 0x10, 0);
            let factory = g32(0x012B9C78);
            let count = lf_checker_rt::callee_thiscall!(
                D_SCATTER, u32, factory, mat_copy.as_mut_ptr().add(2) as u32,
                w88.as_mut_ptr() as u32, 0x3DCCCCCDu32, ent.as_mut_ptr().add(6) as u32, ans_c
            ) as i32;
            if count <= 0 {
                return t2_fail(a2, a3, a4, s50, w88[9]);
            }
            // Scan for the smallest entry key below the running best.
            // Raw reads like the original's cursor; the entry base of slot i
            // sits 96 bytes apart.
            let ent_base = ent.as_ptr() as u32;
            let mut best: f32 = 1.0;
            let mut sel_idx: i32 = -1;
            let mut i = 0i32;
            while i < count {
                let b = ent_base.wrapping_add((i as u32).wrapping_mul(96));
                if rd32(b) != 0 {
                    let k1 = rdf(b + 40);
                    if !comiss_jbe(k1, 0.1) {
                        let k0 = rdf(b + 64);
                        if !comiss_jbe(best, k0) {
                            best = k0;
                            sel_idx = i;
                        }
                    }
                }
                i += 1;
            }
            if sel_idx == -1 {
                return t2_fail(a2, a3, a4, s50, w88[9]);
            }
            let eb = (sel_idx as usize) * 24;
            let h = lf_checker_rt::callee_cdecl!(D_OPEN, u32, ent[eb]);
            if h == 0 {
                return t2_success(a2, a3, sel_idx, &ent);
            }
            if (rd32(h + 0x28) & T2_CLASS_MASK) != T2_CLASS_WANT {
                return t2_success(a2, a3, sel_idx, &ent);
            }
            let r = lf_checker_rt::callee_thiscall!(D_CLASSIFY, u32, h);
            if r == 0 || r == 1 || r == 2 || r == 3 {
                return t2_success(a2, a3, sel_idx, &ent);
            }
            if (flag & T2_LOOP_BIT) == 0 {
                return t2_success(a2, a3, sel_idx, &ent);
            }
            flag &= !T2_LOOP_BIT;
        }
    }
}

/// Failure epilogue: zero pose with the scatter word, residual through a3.
unsafe fn t2_fail(a2: u32, a3: u32, a4: u32, s50: f32, ac: u32) -> u32 {
    unsafe {
        wr32(a2 + 0xC, ac);
        wr32(a2, 0);
        wr32(a2 + 4, 0);
        wr32(a2 + 8, 0x3F80_0000);
        let d = fsub(s50, rdf(a4));
        wrf(a3, d);
        lf_checker_rt::callee_cdecl!(D_COOKIE, u32,);
        0
    }
}

/// Success epilogue: copy the winning entry's pose quad plus key.
unsafe fn t2_success(a2: u32, a3: u32, sel: i32, ent: &[u32; 360]) -> u32 {
    unsafe {
        let eb = (sel as usize) * 24;
        wr32(a2, ent[eb + 8]);
        wr32(a2 + 4, ent[eb + 9]);
        wr32(a2 + 8, ent[eb + 10]);
        wr32(a2 + 0xC, ent[eb + 11]);
        wr32(a3, ent[eb + 6]);
        lf_checker_rt::callee_cdecl!(D_COOKIE, u32,);
        1
    }
}

/// Ped task target solve: fetch the row's matrices, solve the blend target,
/// scatter fifteen candidate entries, and keep the best-scoring one.
///
/// `this` is the task object with its work object at `+0x40`; `a0` is an
/// opaque row handle passed to the third fetch, `a1` the id the second
/// table answer is compared against, `a2`/`a3` out-pointers for the pose
/// quad and the residual, and `a4` a pointer to one input float. The work
/// object carries the row index word at `+0x2E`, a matrix pointer at `+0x20`
/// and an info block at `+0xAB0` whose mode field at `+0x28` (bits 6..9)
/// picks the selector (6, 14 or 134) and the flag dword (0x46, 0x4E or 0xC6).
/// The task object itself holds two threshold triples at `+0x150` and
/// `+0x160`.
///
/// Behaviour: two tabulated calls (index sign-extended, entry loaded from a
/// shared pointer table, slot 0x38) bracket three fetches; the fetched
/// matrices are blended and scaled, one residual is stored through `a4`, and
/// a solve call takes two frame vectors plus the selector. Fifteen 0x60-byte
/// entries are filled from the shared vector, the second table answer picks
/// the threshold triple (each lane stored only when it moves more than 0.01,
/// unordered comparisons counting as within tolerance), a six-argument
/// commit runs, and a scatter call produces a count. A non-positive count or
/// an empty scan takes the failure epilogue (zero pose, residual through
/// `a3`, false); otherwise the winning entry opens, classifies (class bits
/// 6..9 must read 3) and returns a status, and any status but 0..3 with the
/// loop bit set re-runs the fill once with the bit cleared. The success
/// epilogue copies the winning pose quad plus key and returns true.
///
/// Edge cases: a null info block (default selector/flags); a table answer
/// equal to `a1` or not; threshold lanes within tolerance (unordered too);
/// counts of 0, 1, 2 or more with none, one or several selectable entries; a
/// null open result; class mismatch; statuses 0..3; the loop bit clear on
/// the second trip. Signed comparisons: the row index (sign-extended), the
/// count (less-or-equal against 0, loop bound), the loop counters. The
/// original's stack-cookie check calls are reproduced (their cookie argument
/// is frame-derived and not compared). One callee status byte the original
/// initialises and re-reads sits below every pointer the callee receives, so
/// no declared out-param can reach it; it is modelled as untouched (0), and
/// both answer branches are still driven through the callee's return value.
///
/// Original: thiscall, this plus five stack words, boolean in al (upper
/// bytes are leftovers and only al is compared).
lf_checker_rt::export!(thiscall, rw_00c95620(
    this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32
) -> u32 {
    unsafe {
        t2_body(this, a0, a1, a2, a3, a4, false)
    }
});

