// original: 0x005ECA80 CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard>::vf11

/// Store the supplied radius bits in the task's radius field.
///
/// `this` is the task in ECX and the 32-bit stack argument is the exact
/// representation of the float value. The original copies that word to
/// offset 0x38, performs no arithmetic, returns no semantic value, and removes
/// its one stack argument.
lf_checker_rt::export!(thiscall, rw_005eca80(this: *mut u8, radius_bits: u32) -> () {
    const SEEK_RADIUS_OFFSET: usize = 0x38;
    unsafe {
        this.add(SEEK_RADIUS_OFFSET)
            .cast::<u32>()
            .write_unaligned(radius_bits);
    }
});
