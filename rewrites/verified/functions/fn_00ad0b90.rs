// original: 0x00AD0B90 audio_mix_apply
/// Mix source and per-voice values into the output buffer. When a mix source
/// is present, prepare it and combine its header and indexed values with the
/// source using single-precision operations in the original operand order.
/// Copy the header and config folds, clear the per-voice slots, and call the
/// two mixers for each enabled table entry to accumulate their partials. The
/// return is the config word when no voice runs, otherwise the output cursor
/// after the per-voice blocks. The contract defines two otherwise unread
/// frame words as zero.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

#[inline(always)]
unsafe fn rf(p: *const u8, off: usize) -> f32 {
    *(p.add(off) as *const f32)
}

#[inline(always)]
unsafe fn ru(p: *const u8, off: usize) -> u32 {
    *(p.add(off) as *const u32)
}

#[inline(always)]
unsafe fn wf(p: *mut u8, off: usize, v: f32) {
    *(p.add(off) as *mut f32) = v;
}

#[inline(always)]
unsafe fn wu(p: *mut u8, off: usize, v: u32) {
    *(p.add(off) as *mut u32) = v;
}

#[inline(always)]
fn add_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) + core::hint::black_box(right))
}

#[inline(always)]
fn mul_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) * core::hint::black_box(right))
}

export!(cdecl, rw_ad0b90(out: *mut u8, src_b: *const u8, aval: u32, scale: f32, mix_c: *const u8, cfg: *const u8) -> u32 {
    unsafe {
        let count = ru(cfg, 0x38) as i32;
        let mut head = [0f32; 7];
        let mut idx = [0f32; 9];
        let mut in272 = 0f32;
        let mut buf4 = [0f32; 1];
        let mut buf5 = [0f32; 3];
        let mut buf6 = [0f32; 3];
        let mut buf7 = [0f32; 4];
        let mut buf8 = [0f32; 1];
        if mix_c.is_null() {
            in272 = rf(src_b, 0);
            head[0] = rf(src_b, 0x40);
            head[1] = rf(src_b, 0x44);
            head[2] = rf(src_b, 0x48);
            head[3] = rf(src_b, 0x4c);
            head[4] = rf(src_b, 0x50);
            head[5] = rf(src_b, 0x54);
            head[6] = rf(src_b, 0x58);
            idx[0] = rf(src_b, 0x5c);
            if count > 0 {
                for k in 0..(count as usize) {
                    idx[1 + k] = rf(src_b, 0x60 + 4 * k);
                }
            }
        } else {
            let mut scratch = [0u32; 8];
            callee_cdecl!(1, u32, scratch.as_mut_ptr() as u32, src_b as u32,
                mix_c as u32, (mix_c as u32).wrapping_add(16), scale.to_bits());
            head[0] = add_f32(rf(src_b, 0x40), mul_f32(rf(mix_c, 0x20), scale));
            head[2] = add_f32(rf(src_b, 0x48), mul_f32(rf(mix_c, 0x28), scale));
            head[1] = add_f32(rf(src_b, 0x44), mul_f32(rf(mix_c, 0x24), scale));
            head[5] = add_f32(rf(src_b, 0x54), mul_f32(rf(mix_c, 0x34), scale));
            head[6] = add_f32(rf(src_b, 0x58), mul_f32(rf(mix_c, 0x38), scale));
            head[4] = add_f32(rf(src_b, 0x50), mul_f32(rf(mix_c, 0x30), scale));
            let mut e = 0i32;
            if count >= 4 {
                let iters = (((count - 4) >> 2) + 1) as usize;
                for j in 0..iters {
                    for k in 0..4 {
                        idx[1 + 4 * j + k] = add_f32(
                            mul_f32(rf(mix_c, 0x40 + 16 * j + 4 * k), scale),
                            rf(src_b, 0x60 + 16 * j + 4 * k),
                        );
                    }
                }
                e = (4 * iters) as i32;
            }
            while e < count {
                let v = add_f32(
                    mul_f32(rf(mix_c, (0x40 + 4 * e) as usize), scale),
                    rf(src_b, (0x60 + 4 * e) as usize),
                );
                e += 1;
                idx[e as usize] = v;
            }
        }
        wf(out, 0x00, head[0]);
        wf(out, 0x04, head[1]);
        wf(out, 0x08, head[2]);
        wf(out, 0x0c, head[3]);
        wf(out, 0x10, head[4]);
        wf(out, 0x14, head[5]);
        wf(out, 0x18, head[6]);
        wf(out, 0x1c, idx[0]);
        wf(out, 0x20, add_f32(rf(cfg, 0x10), rf(cfg, 0)));
        wf(out, 0x24, add_f32(rf(cfg, 0x14), rf(cfg, 4)));
        wf(out, 0x28, add_f32(rf(cfg, 0x18), rf(cfg, 8)));
        wf(out, 0x2c, 0.0);
        wu(out, 0x30, ru(cfg, 0x20));
        wf(out, 0x34, rf(cfg, 0x24));
        wf(out, 0x38, rf(cfg, 0x28));
        wu(out, 0x3c, ru(cfg, 0x2c));
        if count > 0 {
            for k in 0..(count as usize) {
                wu(out, 0x40 + 4 * k, 0);
            }
            let table = ru(cfg, 0x34);
            let cfg30 = ru(cfg, 0x30);
            for i in 0..(count as usize) {
                let ent = (table as usize).wrapping_add(0x170 * i);
                let flag = *((ent + 0x164) as *const u8);
                if flag & 1 == 0 {
                    continue;
                }
                let this = (table as u32).wrapping_add((0x170 * i) as u32);
                buf4[0] = 0.0;
                callee_thiscall!(2, u32, this, cfg30,
                    buf7.as_mut_ptr() as u32, buf6.as_mut_ptr() as u32,
                    buf5.as_mut_ptr() as u32, buf4.as_mut_ptr() as u32,
                    core::ptr::addr_of!(in272) as u32,
                    core::ptr::addr_of!(head[0]) as u32,
                    core::ptr::addr_of!(head[4]) as u32, aval);
                wf(out, 0x20, add_f32(rf(out, 0x20), buf6[0]));
                wf(out, 0x24, add_f32(buf6[1], rf(out, 0x24)));
                wf(out, 0x28, add_f32(buf6[2], rf(out, 0x28)));
                wf(out, 0x30, add_f32(buf5[0], rf(out, 0x30)));
                wf(out, 0x34, add_f32(buf5[1], rf(out, 0x34)));
                wf(out, 0x38, add_f32(buf5[2], rf(out, 0x38)));
                let wa = (out as *mut f32).wrapping_add((0x60 + 16 * i) / 4);
                *wa.offset(0) = buf7[0];
                *wa.offset(1) = buf7[1];
                *wa.offset(2) = buf7[2];
                *wa.offset(3) = buf7[3];
                let mix1 = idx[i + 1];
                callee_thiscall!(3, u32, this, cfg30,
                    buf7.as_mut_ptr() as u32, buf6.as_mut_ptr() as u32,
                    buf5.as_mut_ptr() as u32, buf8.as_mut_ptr() as u32,
                    buf4[0].to_bits(), core::ptr::addr_of!(in272) as u32,
                    core::ptr::addr_of!(head[0]) as u32,
                    core::ptr::addr_of!(head[4]) as u32,
                    mix1.to_bits(), aval);
                wf(out, 0x20, add_f32(rf(out, 0x20), buf6[0]));
                wf(out, 0x24, add_f32(buf6[1], rf(out, 0x24)));
                wf(out, 0x28, add_f32(buf6[2], rf(out, 0x28)));
                wf(out, 0x30, add_f32(buf5[0], rf(out, 0x30)));
                wf(out, 0x34, add_f32(buf5[1], rf(out, 0x34)));
                wf(out, 0x38, add_f32(buf5[2], rf(out, 0x38)));
                wf(out, 0x40 + 4 * i, buf8[0]);
                *wa.offset(0) = add_f32(*wa.offset(0), buf7[0]);
                *wa.offset(1) = add_f32(*wa.offset(1), buf7[1]);
                *wa.offset(2) = add_f32(buf7[2], *wa.offset(2));
            }
        }
        if count <= 0 {
            ru(cfg, 0x2c)
        } else {
            (out as u32).wrapping_add(0x64).wrapping_add((16 * count) as u32)
        }
    }
});