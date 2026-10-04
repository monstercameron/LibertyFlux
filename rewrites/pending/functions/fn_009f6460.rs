// original: 0x009f6460 Stat_GetFloat
/// First valid code of the small-code branch.
const F2_FIRST: u32 = 0x289;

/// Span of the small-code branch (codes 0x289..=0x2AC).
const F2_SPAN: u32 = 0x23;

/// Code whose value is the sum of four counters.
const F2_SUM_CODE: u32 = 0x2AA;

/// Codes below this read the float table, at/above it the int table.
const F2_INT_TABLE_FROM: i32 = 0xFD;

/// Tunable float getter (cdecl/1 -> ST0).
///
/// Small codes return 1.0 (or a counter sum for 0x2AA); other codes read one
/// of two runtime tables, as float below 0xFD and as integer at/above it.
export!(cdecl, rw_s18f2(code: u32) -> f64 {
    unsafe {
        if code.wrapping_sub(F2_FIRST) > F2_SPAN {
            // Signed comparison in the original (`jge`).
            if (code as i32) < F2_INT_TABLE_FROM {
                let bits = *global::<u32>(0x12B75B0).add(code as usize);
                return f32::from_bits(bits) as f64;
            }
            let raw = *global::<u32>(0x12B75D4).add(code as usize);
            return (raw as i32) as f64;
        }
        if code == F2_SUM_CODE {
            let a = *global::<u32>(0x12B7A6C);
            let b = *global::<u32>(0x12B7A78);
            let c = *global::<u32>(0x12B7A74);
            let d = *global::<u32>(0x12B7A70);
            let sum = a.wrapping_add(b).wrapping_add(c).wrapping_add(d);
            // cvtdq2ps: signed int to float, then exact widening.
            return ((sum as i32) as f32) as f64;
        }
        1.0
    }
});
