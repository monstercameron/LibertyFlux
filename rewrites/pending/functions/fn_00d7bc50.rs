// original: 0x00d7bc50 ui_joint_solver
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const UI_SHARED_OBJECT: u32 = 0x01177A80;
const JOINT_TABLE: u32 = 0x01178284;
const PARAM_TABLE: u32 = 0x01178384;
const JOINT_SCALE: u32 = 0x00FE87A4;
const C_8B08: u32 = 0x00FE8B08;
const C_ONE: u32 = 0x00FE88E8;
const C_AB: u32 = 0x00FE8AB8;
const C_E8: u32 = 0x00E833D4;
const C_8830: u32 = 0x00FE8830;
const C_XOR: u32 = 0x00FE8FA0;
const C_87E4: u32 = 0x00FE87E4;
const C_863C: u32 = 0x00FE863C;

#[inline(always)]
fn rd_joint(base: u32, off: u32, scale: f32) -> f32 {
    unsafe { (base.wrapping_add(off) as *const i16).read_unaligned() as f32 * scale }
}

/// Two-stage joint solver with a shared reporter (original 0x00D7BC50).
///
/// Resolves a base joint from the packed `a3` selector, then optionally
/// refines it in two stages (selectors `a4`/`a6` and `a2`/`a5`; a stage is
/// skipped when its index is 0xFFFF or its table entry is null). Each stage
/// normalizes its joint delta, queries a shared helper (callee 1, answered
/// in x87 ST0) through a parameter block, and folds the answer into two
/// accumulators. The tail normalizes the accumulators, mixes in a gain from
/// the `a1` object, writes two outputs through `a9`, lets a reporter
/// (callee 2) adjust the frame copy, and scales the outputs once more.
///
/// Returns the reporter's answer. Also records `a1` in a global. Note the
/// original pushes one more register after its first block, so every frame
/// slot in the stages sits four bytes below its setup-time address; the
/// names below track values, not slot numbers. The stage-1 scale slot is
/// never written when both stages are skipped and reads as zero under the
/// contract's defined fill, matched here with a zero initializer.
export!(cdecl, rw_d7bc50(
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
    a9: u32,
) -> u32 {
    unsafe {
        let table1 = global::<u32>(JOINT_TABLE);
        let table2 = global::<u32>(PARAM_TABLE);
        let scale = global::<f32>(JOINT_SCALE).read();
        let c_8b08 = global::<f32>(C_8B08).read();
        let c_one = global::<f32>(C_ONE).read();
        let c_ab = global::<f32>(C_AB).read();
        let c_e8 = global::<f32>(C_E8).read();
        let c_8830 = global::<f32>(C_8830).read();
        let c_xor = global::<u32>(C_XOR).read();
        let c_87e4 = global::<f32>(C_87E4).read();
        let c_863c = global::<f32>(C_863C).read();
        global::<u32>(0x01797738).write(a1);
        // Base joint from a3 (no validity check in the original).
        let lo3 = a3 & 0xFFFF;
        let r3 = table1.add(lo3 as usize).read().wrapping_add((a3 >> 16) << 5);
        let bit0 = (r3.wrapping_add(0x1F) as *const u8).read() & 2;
        let x0 = rd_joint(r3, 0x14, scale);
        let y0 = rd_joint(r3, 0x16, scale);
        let mut f24 = if bit0 != 0 { c_8b08 } else { c_one };
        let mut f28 = 0.0f32;
        let mut f2c = 0.0f32;
        let mut f30 = 0.0f32;
        let mut x2v = 0.0f32;
        let mut x4v = 0.0f32;
        let mut x7 = 0.0f32;
        // Stage 1 (a4 joint, a3 params, a6 index).
        let lo4 = a4 & 0xFFFF;
        if lo4 != 0xFFFF {
            let b4 = table1.add(lo4 as usize).read();
            if b4 != 0 {
                let p1 = table2.add(lo3 as usize).read().wrapping_add(a6.wrapping_mul(8));
                let r4 = b4.wrapping_add((a4 >> 16) << 5);
                let x4 = rd_joint(r4, 0x14, scale);
                let y4 = rd_joint(r4, 0x16, scale);
                let dx = x4 - x0;
                let dy = y4 - y0;
                let sq = dy * dy + dx * dx;
                let inv = if sq == 0.0 { 0.0 } else { 1.0 / sq.sqrt() };
                let bit7 = (p1.wrapping_add(6) as *const u8).read() & 0x80;
                let nx = dx * inv;
                let ny = dy * inv;
                f28 = ny;
                x7 = nx;
                f2c = nx;
                let m38 = if bit7 != 0 { c_ab } else { c_e8 };
                f30 = m38;
                let ans: f64 = callee_thiscall!(1, f64, p1);
                let fa = ans as f32;
                let mut v1 = fa + (a8 as i32) as f32 * m38;
                let cl = (p1.wrapping_add(7) as *const u8).read();
                let al5 = ((p1.wrapping_add(5) as *const u8).read() & 7) as f32;
                if cl & 1 != 0 {
                    let c = if bit7 != 0 { c_ab } else { c_e8 };
                    v1 = al5 * (c * c_8830) + v1;
                }
                if cl & 2 != 0 {
                    let c = if bit7 != 0 { c_ab } else { c_e8 };
                    v1 = v1 - al5 * (c * c_8830);
                }
                x4v = nx * v1;
                x2v = ny * v1;
                x4v = f32::from_bits(x4v.to_bits() ^ c_xor);
                if bit7 != 0 {
                    f24 = c_87e4;
                }
            }
        }
        // Stage 2 (a2 joint, a2 params, a5 index).
        let lo2 = a2 & 0xFFFF;
        if lo2 != 0xFFFF {
            let b2 = table1.add(lo2 as usize).read();
            if b2 != 0 {
                let p2 = table2.add(lo2 as usize).read().wrapping_add(a5.wrapping_mul(8));
                let r2 = b2.wrapping_add((a2 >> 16) << 5);
                let x2 = rd_joint(r2, 0x14, scale);
                let y2 = rd_joint(r2, 0x16, scale);
                let dx = x0 - x2;
                let dy = y0 - y2;
                let sq = dy * dy + dx * dx;
                let inv = if sq == 0.0 { 0.0 } else { 1.0 / sq.sqrt() };
                let bit7 = (p2.wrapping_add(6) as *const u8).read() & 0x80;
                let nx = dx * inv;
                let ny = dy * inv;
                x7 += nx;
                f2c = x7;
                f28 += ny;
                x4v = x4v * ny + nx * x2v;
                let m0c = x4v * nx;
                let m28b = x4v * ny;
                let m1c = if bit7 != 0 { c_ab } else { c_e8 };
                f30 = nx;
                let ans2: f64 = callee_thiscall!(1, f64, p2);
                let fa2 = ans2 as f32;
                let mut v1b = fa2 + (a7 as i32) as f32 * m1c;
                let cl = (p2.wrapping_add(7) as *const u8).read();
                let al5 = ((p2.wrapping_add(5) as *const u8).read() & 7) as f32;
                if cl & 4 != 0 {
                    let c = if bit7 != 0 { c_ab } else { c_e8 };
                    v1b = al5 * (c * c_8830) + v1b;
                }
                if cl & 8 != 0 {
                    let c = if bit7 != 0 { c_ab } else { c_e8 };
                    v1b = v1b - al5 * (c * c_8830);
                }
                x2v = v1b * ny + m0c;
                x4v = m28b - v1b * nx;
                if bit7 != 0 {
                    f24 = c_87e4;
                }
            }
        }
        // Tail: normalize, mix, report, scale.
        // NOTE: the tail loads its accumulators before popping its saved
        // register, so these are the stage-frame slots (the sum and the two
        // running totals), not the setup-time numbering.
        let n2 = f2c * f2c + f28 * f28;
        let (s5, s3) = if c_863c > n2 {
            (1.0, 0.0)
        } else if n2 == 0.0 {
            (0.0, 0.0)
        } else {
            let i = 1.0 / n2.sqrt();
            (i * f2c, i * f28)
        };
        let k = (a1.wrapping_add(0xF08) as *const f32).read();
        x4v -= k * s5;
        let outx = x0 + (k * s3 + x2v);
        let outy = y0 + x4v;
        (a9 as *mut f32).write(outx);
        (a9.wrapping_add(4) as *mut f32).write(outy);
        let mut pair = [outx.to_bits(), outy.to_bits()];
        let r3: u32 = callee_cdecl!(2, u32, a1, pair.as_mut_ptr() as u32);
        let fx = f32::from_bits(pair[0]);
        let fy = f32::from_bits(pair[1]);
        (a9 as *mut f32).write(fx * f24 + outx);
        (a9.wrapping_add(4) as *mut f32).write(fy * f24 + outy);
        r3
    }
});
