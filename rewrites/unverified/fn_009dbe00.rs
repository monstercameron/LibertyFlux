// original: 0x009DBE00 pool_slot_next_0x20 (proposed)

/// Take the next slot of a 32-byte-stride pool: `count * 32 + base`.
///
/// The element count lives in a global, the element base in the next one.
/// The slot address is computed, the count is bumped past it, and the slot
/// is returned. Unlike the neighbouring pool allocators this one neither
/// initialises the slot nor registers it anywhere.
///
/// Original: 0x009DBE00 (cdecl, no arguments, no outgoing calls).
lf_checker_rt::export!(cdecl, rw_009DBE00() -> u32 {
    unsafe {
        const COUNT_VA: u32 = 0x0103ADBC;
        const BASE_VA: u32 = 0x0103ADC0;
        const STRIDE_BITS: u32 = 5;

        let count = (lf_checker_rt::global::<u32>(COUNT_VA) as *const u32).read_unaligned();
        let base = (lf_checker_rt::global::<u32>(BASE_VA) as *const u32).read_unaligned();
        let slot = count.wrapping_shl(STRIDE_BITS).wrapping_add(base);
        lf_checker_rt::global::<u32>(COUNT_VA).write_unaligned(count.wrapping_add(1));
        slot
    }
});
