// original: 0x00908ce0 radar_range_div_store
/// Store a divided radar-range value set.
///
/// Stores `a` then the signed quotient `a / b` (the caller guarantees a
/// nonzero divisor and a fitting quotient) plus the other three arguments
/// into five scalar globals. Returns the quotient.
export!(cdecl, rw_00908CE0(a: u32, b: u32, c: u32, d: u32) -> u32 {
    unsafe {
        *global::<u32>(0x10344E0) = a;
        let q = (a as i32) / (b as i32);
        *global::<u32>(0x10344E8) = c;
        *global::<u32>(0x10344E4) = b;
        *global::<u32>(0x10344EC) = d;
        *global::<u32>(0x10344DC) = q as u32;
        q as u32
    }
});
