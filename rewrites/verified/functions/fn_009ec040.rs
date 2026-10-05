// original: 0x009EC040 ped_target_picker (proposed)

/// Ped target picker: polls a scanner helper, keeps the highest-scoring
/// candidate, then attaches the winner.
///
/// `thiscall`: `this` in ECX, no stack arguments, no return value.
/// Reads `this+0x38`/`+0x7B4` (an equal nonzero pair returns at once), then
/// runs up to five scanner rounds: each round calls the direct helper on
/// the object at `this+0x78` with the round index twice and four
/// out-pointers. The four out-words are a float pair and a pointer pair;
/// each pointer's flag bit 5 at +0x46 selects whether its float is lowered
/// by 0.4 (marker +0x5C reads 0.0), raised by 0.4 (marker reads 1.0) or
/// kept. The higher float's pointer becomes the candidate; a candidate
/// scoring at least 0.6 ends the scan, otherwise the next round runs. With
/// no winner the function returns.
///
/// The winner's word at +0x44 selects an owner object (+0x40 when 1, else
/// the null pointer, which faults on the flag read like the original).
/// When the owner's flag byte +0x28 has bit 3 set, a direct lookup runs; a
/// hit yields a ratio table that selects one entry by
/// `min(cvttss2si(0.0/scale), n-1)` (the index wraps mod 2^32 for the
/// not-a-number scale), and virtual slot +0x14 on the entry's object runs
/// with the negated blend weight and an out-pointer. A clear out-word and
/// a nonzero link at winner+0x80 run a direct attacher on `this+0x3C0` and
/// clear the link. Owner flag bits 6 and 7 each run a direct setter on the
/// ped with winner+0x78/+0x7C.
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
fn fsub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}
#[inline(always)]
fn fdiv(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

/// Exact `cvttss2si`: truncate toward zero; out-of-range, infinite and
/// not-a-number inputs give `i32::MIN` (Rust's `as` cast saturates
/// instead, so it cannot be used directly).
#[inline(always)]
fn cvttss2si(x: f32) -> i32 {
    if x.is_nan() {
        return i32::MIN;
    }
    let t = x.trunc();
    if t < -2147483648.0 || t >= 2147483648.0 {
        i32::MIN
    } else {
        t as i32
    }
}

const G_ZERO_MARK: u32 = 0x00FE8628; // 0.0
const G_STEP: u32 = 0x00FE881C; // 0.4
const G_ONE_MARK: u32 = 0x00FE88E8; // 1.0
const G_ACCEPT: u32 = 0x00FE8858; // 0.6
const G_SIGN: u32 = 0x00FE8FA0; // 0x80000000

unsafe fn adjust_marked(ptr: u32, base: f32, step: f32) -> f32 {
    unsafe {
        if ptr == 0 {
            return base;
        }
        if (rd8(ptr.wrapping_add(0x46)) >> 5) & 1 == 0 {
            return base;
        }
        let mark = f32::from_bits(rd32(ptr.wrapping_add(0x5C)));
        let zero = f32::from_bits(rd32(lf_checker_rt::relocated(G_ZERO_MARK)));
        if mark != zero {
            let one = f32::from_bits(rd32(lf_checker_rt::relocated(G_ONE_MARK)));
            if mark != one {
                return base;
            }
            return fadd(base, step);
        }
        fsub(base, step)
    }
}

unsafe fn owner_of(winner: u32) -> u32 {
    unsafe {
        if rd16(winner.wrapping_add(0x44)) == 1 {
            rd32(winner.wrapping_add(0x40))
        } else {
            0
        }
    }
}

unsafe fn release_link(winner: u32, this: u32) {
    unsafe {
        let link = rd32(winner.wrapping_add(0x80));
        if link != 0 {
            lf_checker_rt::callee_thiscall!(4, u32, this.wrapping_add(0x3C0), link);
            wr32(winner.wrapping_add(0x80), 0);
        }
    }
}

lf_checker_rt::export!(thiscall, rw_009EC040(this: u32) -> u32 {
    unsafe {
        let gate = rd32(this.wrapping_add(0x38));
        if gate != 0 && gate == rd32(this.wrapping_add(0x7B4)) {
            return 0;
        }
        let scanner = rd32(this.wrapping_add(0x78));
        let step = f32::from_bits(rd32(lf_checker_rt::relocated(G_STEP)));
        let accept = f32::from_bits(rd32(lf_checker_rt::relocated(G_ACCEPT)));
        let mut esi = 4u32;
        let winner: u32;
        loop {
            let mut d0 = 0u32;
            let mut d1 = 0u32;
            let mut d2 = 0u32;
            let mut d3 = 0u32;
            lf_checker_rt::callee_thiscall!(
                1, u32, scanner, esi, esi,
                &mut d3 as *mut u32 as u32,
                &mut d2 as *mut u32 as u32,
                &mut d1 as *mut u32 as u32,
                &mut d0 as *mut u32 as u32
            );
            let mut cand = d3;
            let mut score = adjust_marked(d3, f32::from_bits(d2), step);
            d2 = score.to_bits();
            let other = d1;
            let other_score = adjust_marked(d1, f32::from_bits(d0), step);
            d0 = other_score.to_bits();
            if !(score > other_score) {
                cand = other;
                score = other_score;
            }
            if cand != 0 && score >= accept {
                winner = cand;
                break;
            }
            esi = esi.wrapping_sub(1);
            if (esi as i32) < 0 {
                return 0;
            }
        }
        if rd8(owner_of(winner).wrapping_add(0x28)) & 8 != 0 {
            let hit: u32 =
                lf_checker_rt::callee_thiscall!(2, u32, owner_of(winner), 0x82, 0);
            if hit == 0 {
                release_link(winner, this);
            } else {
                let scale = rd16(hit.wrapping_add(4));
                let ratio = fdiv(0.0, scale as f32);
                let table = rd32(hit.wrapping_add(8));
                let mut idx = rd16(hit.wrapping_add(0x0C)) as u32;
                let pick = cvttss2si(ratio);
                idx = idx.wrapping_sub(1);
                if pick < idx as i32 {
                    idx = pick as u32;
                }
                let prod = (scale as u32).wrapping_mul(idx);
                let weight = (prod as i32) as f32;
                let neg = f32::from_bits(
                    weight.to_bits() ^ rd32(lf_checker_rt::relocated(G_SIGN)),
                );
                let entry = rd32(table.wrapping_add(idx.wrapping_mul(4)));
                let obj = rd32(entry.wrapping_add(4));
                let vt = rd32(obj);
                let run: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(0x14)) as usize);
                let mut out = 0u32;
                run(obj, neg.to_bits(), &mut out as *mut u32 as u32);
                if (out >> 24) & 1 == 0 {
                    release_link(winner, this);
                }
            }
        }
        if rd8(owner_of(winner).wrapping_add(0x28)) & 0x40 != 0 {
            let arg = rd32(winner.wrapping_add(0x78));
            lf_checker_rt::callee_thiscall!(5, u32, this, arg, 0, 0, 1);
        }
        if rd8(owner_of(winner).wrapping_add(0x28)) & 0x80 != 0 {
            let arg = rd32(winner.wrapping_add(0x7C));
            lf_checker_rt::callee_thiscall!(5, u32, this, arg, 0, 0, 1);
        }
        0
    }
});
