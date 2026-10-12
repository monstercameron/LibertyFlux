// original: 0x005ECA30 CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard>::vf10

/// Update the seek radius and reset the cached radius-derived state if it changed.
///
/// The thiscall receiver is in ECX and the float's exact 32-bit word is on the
/// stack. The original treats unordered float comparisons as changed, treats
/// positive and negative zero as equal, copies the new radius bits to offset
/// 0x34, copies a relocated data word to offset 0x48, clears offset 0x4c, and
/// sets the byte at offset 0x50. Equal ordered values leave all four fields
/// untouched. The method removes its one stack argument and has no semantic
/// return value.
lf_checker_rt::export!(thiscall, rw_005eca30(this: *mut u8, radius_bits: u32) -> () {
    const CURRENT_RADIUS_OFFSET: usize = 0x34;
    const CACHED_SCALE_OFFSET: usize = 0x48;
    const CACHE_FLAGS_OFFSET: usize = 0x4c;
    const CACHE_VALID_OFFSET: usize = 0x50;
    const SEEK_SCALE_VA: u32 = 0x011735b4;

    unsafe {
        let current_bits = this.add(CURRENT_RADIUS_OFFSET).cast::<u32>().read_unaligned();
        let requested_radius = f32::from_bits(radius_bits);
        let current_radius = f32::from_bits(current_bits);
        if requested_radius != current_radius {
            let scale_bits = lf_checker_rt::global::<u32>(SEEK_SCALE_VA).read_unaligned();
            this.add(CURRENT_RADIUS_OFFSET).cast::<u32>().write_unaligned(radius_bits);
            this.add(CACHED_SCALE_OFFSET).cast::<u32>().write_unaligned(scale_bits);
            this.add(CACHE_FLAGS_OFFSET).cast::<u32>().write_unaligned(0);
            this.add(CACHE_VALID_OFFSET).write(1);
        }
    }
});
