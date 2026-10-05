// original: 0x00e694f0 veh_ratio_init_e694f0 (proposed)
/// Initialise one derived vehicle tuning constant as the quotient of two globals.
///
/// Reads two `f32` globals (numerator at numerator address, denominator 4 bytes past
/// it) and stores their SSE quotient (`divss`) into a third global. Takes no arguments
/// and makes no calls (cdecl, no stack words); the return register is untouched.
///
/// Edge cases: a zero denominator yields signed infinity (or NaN for 0/0), NaN inputs
/// propagate, all exactly as one `divss` instruction; operand order is pinned with
/// `black_box` so the compiler cannot commute or fold the division.
///
/// Original: 0x00e694f0 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00e694f0() -> u32 {
    unsafe {
        const NUMERATOR: u32 = 0x010457B0;
        const DENOMINATOR: u32 = 0x010457B4;
        const DESTINATION: u32 = 0x01657638;
        let a = f32::from_bits(core::ptr::read_unaligned(lf_checker_rt::global::<u32>(NUMERATOR) as *const u32));
        let b = f32::from_bits(core::ptr::read_unaligned(lf_checker_rt::global::<u32>(DENOMINATOR) as *const u32));
        let q = core::hint::black_box(a) / core::hint::black_box(b);
        core::ptr::write_unaligned(lf_checker_rt::global::<u32>(DESTINATION), q.to_bits());
        0
    }
});
