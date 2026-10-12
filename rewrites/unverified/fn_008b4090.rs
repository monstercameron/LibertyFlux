// original: 0x008b4090 stick_response_apply
/// Applies a two-channel stick response curve and advances the slot cycle.
///
/// The source holds two sampled channel values, a tag word, a spare word and
/// two flag bytes; the destination holds the filtered outputs, a copied tag, a
/// rotating slot index and the flag bytes. Each channel value is first shifted
/// by the gate constant: a shifted value that is below zero (or unordered) maps
/// to a zero output, otherwise the value is scaled and passed with the base
/// double to the curve helper, whose double result narrows to the output float.
/// The spare word and both flag bytes are copied over, and when the tag differs
/// from the stored one the stored tag is replaced and, if the second flag byte
/// is zero, the table refresh routine runs against this object. Finally the
/// slot index increments modulo 3 and the divided-down count is returned.
use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

export!(thiscall, rw_008b4090(this_ptr: u32, src_ptr: u32) -> u32 {
    unsafe {
        /// Gate constant: values at or below it map to a zero output.
        const GATE_LO: u32 = 0x00FE_8DF8;
        /// Per-channel scale applied before the curve helper runs.
        const SCALE: u32 = 0x00FE_876C;
        /// Base double passed as the helper's first argument.
        const BASE_DBL: u32 = 0x00FE_8A78;
        /// Destination fields.
        const OUT_A: u32 = 0x0C;
        const OUT_B: u32 = 0x10;
        const SPARE: u32 = 0x14;
        const TAG: u32 = 0x18;
        const FLAG_A: u32 = 0x1C;
        const FLAG_B: u32 = 0x20;
        const SLOT: u32 = 0x08;
        /// Source fields.
        const SRC_A: u32 = 0x00;
        const SRC_B: u32 = 0x04;
        const SRC_SPARE: u32 = 0x08;
        const SRC_TAG: u32 = 0x0C;
        const SRC_FLAG_B: u32 = 0x10;
        const SRC_FLAG_A: u32 = 0x11;
        /// Curve helper callee id (takes two doubles, answers one).
        const CURVE: u32 = 1;
        /// Table refresh callee id.
        const REFRESH: u32 = 2;
        /// Slot cycle length.
        const SLOTS: u32 = 3;

        let gate = *(relocated(GATE_LO) as *const f32);
        let scale = *(relocated(SCALE) as *const f32);
        let base_bits = *(relocated(BASE_DBL) as *const u64);
        let base_lo = (base_bits & 0xFFFF_FFFF) as u32;
        let base_hi = (base_bits >> 32) as u32;

        let run_curve = |sample: f32| -> f32 {
            let scaled = black_box(sample) * black_box(scale);
            let wide: f64 = black_box(scaled) as f64;
            let bits = wide.to_bits();
            let answer: u64 = callee_cdecl!(
                CURVE,
                u64,
                base_lo,
                base_hi,
                (bits & 0xFFFF_FFFF) as u32,
                (bits >> 32) as u32
            );
            black_box(f64::from_bits(answer)) as f32
        };

        let a = *(src_ptr.wrapping_add(SRC_A) as *const f32);
        let gated_a = black_box(a) - black_box(gate);
        // The original branches below-or-unordered here: NaN takes the zero path.
        let out_a = if !(gated_a >= 0.0) { 0.0 } else { run_curve(a) };
        *((this_ptr.wrapping_add(OUT_A)) as *mut f32) = out_a;

        let b = *(src_ptr.wrapping_add(SRC_B) as *const f32);
        let gated_b = black_box(b) - black_box(gate);
        let out_b = if !(gated_b >= 0.0) { 0.0 } else { run_curve(b) };
        *((this_ptr.wrapping_add(OUT_B)) as *mut f32) = out_b;

        let spare = *((src_ptr.wrapping_add(SRC_SPARE)) as *const u32);
        *((this_ptr.wrapping_add(SPARE)) as *mut u32) = spare;
        let flag_a = *((src_ptr.wrapping_add(SRC_FLAG_A)) as *const u8);
        *((this_ptr.wrapping_add(FLAG_A)) as *mut u8) = flag_a;
        let flag_b = *((src_ptr.wrapping_add(SRC_FLAG_B)) as *const u8);
        *((this_ptr.wrapping_add(FLAG_B)) as *mut u8) = flag_b;

        let tag = *((src_ptr.wrapping_add(SRC_TAG)) as *const u32);
        if tag != *((this_ptr.wrapping_add(TAG)) as *const u32) {
            *((this_ptr.wrapping_add(TAG)) as *mut u32) = tag;
            if *((this_ptr.wrapping_add(FLAG_B)) as *const u8) == 0 {
                callee_thiscall!(REFRESH, u32, this_ptr);
            }
        }

        let slot = (*((this_ptr.wrapping_add(SLOT)) as *const u32)).wrapping_add(1);
        *((this_ptr.wrapping_add(SLOT)) as *mut u32) = slot % SLOTS;
        slot / SLOTS
    }
});
