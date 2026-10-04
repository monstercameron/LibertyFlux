// original: 0x00d8f6f0 audio_vector_rescale_publish
//! Single-function extract of the verified rewrite; needs `lf_k2_rt` and the
//! `cvttss2si` helper below.

use lf_k2_rt::{callee_stdcall, export, global};

/// Truncate an `f32` to `i32` with x86 `CVTTSS2SI` semantics.
///
/// Rust's `as` casts saturate out-of-range values and map NaN to 0; the
/// instruction instead yields `0x80000000` for NaN, infinities and anything
/// outside `[-2^31, 2^31)`. The boundary checks below are exact for `f32`:
/// the representable values adjacent to the range ends are `-2^31`
/// (valid, converts to itself) and `-2^31 - 256` (invalid).
#[inline(always)]
pub fn cvttss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        0x80000000u32 as i32
    } else {
        x as i32
    }
}

/// Rescale a 3-vector around a centre value and forward it.
///
/// Reads three floats through `vec`, forms `(v - f)` and `(v + f)` for each
/// lane, scales the six results by the shared constant 8.0 and truncates them
/// to 16-bit slots in a stack scratch table, publishes the unscaled
/// differences and sums plus the two flag bytes to globals, then forwards a
/// field of the `this` object, the vector pointer and three pointers into
/// the scratch table (an explicit zero word, a marker word, the slot table)
/// to the next stage (callee 1).
///
/// The original also copies one untouched scratch word (which the checker
/// contract zero-fills) into two globals, so those two outputs are 0.0 here,
/// and returns the explicitly zeroed scratch word, so the return is 0.
export!(thiscall, rw_00d8f6f0(this_ptr: u32, vec: u32, f: f32, b0: u32, b1: u32) -> u32 {
    unsafe {
        const SCALE_ADDR: u32 = 0x00FE8AFC; // shared float constant (8.0)
        const GLO_BASE: u32 = 0x0179FAD0; // eight published float slots
        const FLAG_BASE: u32 = 0x0179F940; // two published flag bytes

        let x = *(vec as *const f32);
        let y = *((vec + 4) as *const f32);
        let z = *((vec + 8) as *const f32);
        let c = *(global::<f32>(SCALE_ADDR) as *const f32);

        let xm = x - f;
        let xp = x + f;
        let ym = y - f;
        let yp = y + f;
        let zm = z - f;
        let zp = z + f;

        // Scratch table with the original's exact byte layout so the three
        // table pointers passed to the callee observe identical bytes.
        #[repr(C)]
        struct Scratch {
            fill0: u32, // +0x00: untouched scratch (contract zero-fill)
            fill1: u32, // +0x04: untouched scratch
            zero: u32, // +0x08: explicit zero (the original returns this word)
            head: u32, // +0x0c: marker word 0x7F7FFFFF
            w0: u16, // +0x10: trunc((x - f) * 8)
            w3: u16, // +0x12: trunc((x + f) * 8)
            w1: u16, // +0x14: trunc((y - f) * 8)
            w4: u16, // +0x16: trunc((y + f) * 8)
            w2: u16, // +0x18: trunc((z - f) * 8)
            w5: u16, // +0x1a: trunc((z + f) * 8)
            tail: u32, // +0x1c: untouched scratch (copied to two globals)
            trail: u32, // +0x20: untouched scratch
        }
        let mut s = Scratch {
            fill0: 0,
            fill1: 0,
            zero: 0,
            head: 0x7F7FFFFF,
            w0: cvttss2si(xm * c) as u16,
            w3: cvttss2si(xp * c) as u16,
            w1: cvttss2si(ym * c) as u16,
            w4: cvttss2si(yp * c) as u16,
            w2: cvttss2si(zm * c) as u16,
            w5: cvttss2si(zp * c) as u16,
            tail: 0,
            trail: 0,
        };
        let base = (&mut s as *mut Scratch) as u32;

        *(global::<f32>(GLO_BASE) as *mut f32) = xm;
        *(global::<f32>(GLO_BASE + 4) as *mut f32) = ym;
        *(global::<f32>(GLO_BASE + 8) as *mut f32) = zm;
        *(global::<f32>(GLO_BASE + 12) as *mut f32) = 0.0;
        *(global::<f32>(GLO_BASE + 16) as *mut f32) = xp;
        *(global::<f32>(GLO_BASE + 20) as *mut f32) = yp;
        *(global::<f32>(GLO_BASE + 24) as *mut f32) = zp;
        *(global::<f32>(GLO_BASE + 28) as *mut f32) = 0.0;
        *(global::<u8>(FLAG_BASE) as *mut u8) = b0 as u8;
        *(global::<u8>(FLAG_BASE + 1) as *mut u8) = b1 as u8;

        let field70 = *((this_ptr + 0x70) as *const u32);
        let _: u32 = callee_stdcall!(1, u32, field70, vec, base + 0x10, base + 0x0c, base + 0x08);
        // The original returns its explicitly zeroed scratch word.
        0
    }
});
