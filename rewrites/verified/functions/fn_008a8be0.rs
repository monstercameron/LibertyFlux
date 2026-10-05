// original: 0x008a8be0 audio_voice_spatialize_a
//
// Spatialize one voice: filter four direction rows through two intercepted
// stage calls, transpose the result, emit it, then store the input rows
// and the output level into per-index slots.
//
// Arguments: `obj` (ECX) is the voice object, `vec` points at four
// 3-float rows at strides of 0x10, `index` selects the slots, `level` is
// the output level as bits. Each row's squared length is checked for
// infinity/NaN; if any row fails, the fallback path runs instead (the
// level becomes the global float and the row copies are skipped). The
// emitter slot uses bias 0xF and the row-store slot bias 5. Returns
// `tls_base + index * 4`. Thiscall with three stack args.
//
// Frame model: the original's aligned frame holds three 16-word buffers
// (rows, direction, one scratch shared by both stage calls); they are
// modelled as zeroed arrays and their addresses are passed to the
// intercepted callees exactly like the original passes its frame
// pointers, so call-time snapshots and scripted callee writes land in
// the same places on both sides.

use lf_checker_rt::{callee_thiscall, export, global, tls_slot};

/// File VA of the global holding the TLS slot number of the voice table.
const TLS_SLOT_INDEX: u32 = 0x17aba14;
/// Field in the thread's TLS struct holding the voice-table base.
const TLS_VOICE_BASE_OFF: u32 = 0x70;
/// Bytes per coefficient row selected by the biased index.
const ROW_STRIDE_BITS: u32 = 6;
/// Global float xor-mask applied to the direction row in the normal path.
const SP_XOR_MASK: u32 = 0xfe8fa0;
/// Global float used as the output level in the fallback path.
const SP_FALLBACK_LEVEL: u32 = 0xfe870c;
/// Object field read by the second stage call (callee id 2, call B).
const SP_STAGE_INPUT_OFF: u32 = 0x1400;
/// Stride of the final per-index level slot, in 4-byte units past 0x1580.
const SP_LEVEL_BASE_OFF: u32 = 0x1580;
/// Callee ids (see contract): 1 = row initializer, 2 = stage combiner,
/// 3 = indexed emitter.
const SP_INIT: u32 = 1;
const SP_STAGE: u32 = 2;
const SP_EMIT: u32 = 3;
/// Slot biases selecting this instance's emitter and row-store slots.
const EMIT_BIAS: u32 = 0xF;
const STORE_BIAS: u32 = 5;

#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}
#[inline(always)]
unsafe fn rf32(addr: u32) -> f32 {
    unsafe { (addr as *const f32).read_unaligned() }
}
#[inline(always)]
unsafe fn ru32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn wu32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write_unaligned(v) }
}

export!(thiscall, rw_008a8be0(obj: u32, vec: u32, index: u32, level: u32) -> u32 {
    unsafe {
        // Finiteness gate: squared length of each row, (x0^2 + x1^2) + x2^2.
        let mut fallback = false;
        for r in [0u32, 0x10, 0x20, 0x30] {
            let x0 = rf32(vec.wrapping_add(r));
            let x1 = rf32(vec.wrapping_add(r).wrapping_add(4));
            let x2 = rf32(vec.wrapping_add(r).wrapping_add(8));
            let s = fadd(fadd(fmul(x0, x0), fmul(x1, x1)), fmul(x2, x2));
            if s.to_bits() & 0x7f80_0000 == 0x7f80_0000 {
                fallback = true;
                break;
            }
        }
        let mut rows = [0u32; 16];
        let mut dir = [0u32; 16];
        let mut scratch = [0u32; 16];
        let slot = global::<u32>(TLS_SLOT_INDEX).read();
        let tls = tls_slot(slot as usize);
        let tls_base = ru32(tls.wrapping_add(TLS_VOICE_BASE_OFF));
        if !fallback {
            let _ = callee_thiscall!(SP_INIT, u32, dir.as_mut_ptr() as u32);
            let mask = ru32(global::<u32>(SP_XOR_MASK) as u32);
            dir[3] = ru32(vec.wrapping_add(0x30)) ^ mask;
            dir[7] = ru32(vec.wrapping_add(0x34)) ^ mask;
            dir[11] = ru32(vec.wrapping_add(0x38)) ^ mask;
            dir[15] = 0x3f80_0000;
            let _ = callee_thiscall!(SP_INIT, u32, rows.as_mut_ptr() as u32);
            rows[0] = ru32(vec);
            rows[1] = ru32(vec.wrapping_add(4));
            rows[2] = ru32(vec.wrapping_add(8));
            rows[3] = 0;
            rows[4] = ru32(vec.wrapping_add(0x10));
            rows[5] = ru32(vec.wrapping_add(0x14));
            rows[6] = ru32(vec.wrapping_add(0x18));
            rows[7] = 0;
            rows[8] = ru32(vec.wrapping_add(0x20));
            rows[9] = ru32(vec.wrapping_add(0x24));
            rows[10] = ru32(vec.wrapping_add(0x28));
            rows[11] = 0;
        } else {
            let _ = callee_thiscall!(SP_INIT, u32, dir.as_mut_ptr() as u32);
            let _ = callee_thiscall!(SP_INIT, u32, rows.as_mut_ptr() as u32);
        }
        // Argument order mirrors the original's pushes: the last push lands
        // at [esp+4] (arg0), so arg0 is the second buffer pushed.
        let _ = callee_thiscall!(
            SP_STAGE,
            u32,
            scratch.as_mut_ptr() as u32,
            dir.as_mut_ptr() as u32,
            rows.as_mut_ptr() as u32
        );
        let _ = callee_thiscall!(
            SP_STAGE,
            u32,
            rows.as_mut_ptr() as u32,
            scratch.as_mut_ptr() as u32,
            obj.wrapping_add(SP_STAGE_INPUT_OFF)
        );
        // In-place 4x4 transpose of the off-diagonal words, transcribed
        // move by move: swaps (1,4), (2,8), (6,9), (3,12), (7,13), (11,14).
        let mut t1 = rows[4];
        let mut t0 = rows[1];
        rows[4] = t0;
        t0 = rows[2];
        rows[1] = t1;
        t1 = rows[8];
        rows[8] = t0;
        t0 = rows[6];
        rows[2] = t1;
        t1 = rows[9];
        rows[9] = t0;
        t0 = rows[3];
        rows[6] = t1;
        t1 = rows[12];
        rows[12] = t0;
        t0 = rows[7];
        rows[3] = t1;
        t1 = rows[13];
        rows[13] = t0;
        t0 = rows[11];
        rows[7] = t1;
        t1 = rows[14];
        rows[14] = t0;
        rows[11] = t1;
        // Emit the transposed block into the indexed slot, then store the
        // input rows and the level into their slots.
        let emit = obj.wrapping_add(
            tls_base
                .wrapping_add(index.wrapping_add(EMIT_BIAS).wrapping_mul(4))
                .wrapping_shl(ROW_STRIDE_BITS),
        );
        let _ = callee_thiscall!(SP_EMIT, u32, emit, rows.as_mut_ptr() as u32);
        let dst = obj.wrapping_add(
            tls_base
                .wrapping_add(index.wrapping_add(STORE_BIAS).wrapping_mul(4))
                .wrapping_shl(ROW_STRIDE_BITS),
        );
        for off in [0u32, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38] {
            wu32(dst.wrapping_add(off), ru32(vec.wrapping_add(off)));
        }
        let out_level = if fallback {
            ru32(global::<u32>(SP_FALLBACK_LEVEL) as u32)
        } else {
            level
        };
        let ii = tls_base.wrapping_add(index.wrapping_mul(4));
        wu32(
            obj.wrapping_add(ii.wrapping_mul(4)).wrapping_add(SP_LEVEL_BASE_OFF),
            out_level,
        );
        ii
    }
});
