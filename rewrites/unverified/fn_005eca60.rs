// original: 0x005ECA60 CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard>::vf7

/// Copy the task's target point to the caller's three-word output buffer.
///
/// `this` is the task in ECX; the sole stack argument points to three writable
/// 32-bit components. The source components are stored at task offsets 0x1c,
/// 0x20 and 0x24. The original returns the output pointer in EAX and removes
/// the one stack argument, so this is a thiscall method. Components are copied
/// as bits; the routine performs no floating-point arithmetic.
lf_checker_rt::export!(thiscall, rw_005eca60(this: *const u8, output: *mut u32) -> *mut u32 {
    const TARGET_X_OFFSET: usize = 0x1c;
    const TARGET_Y_OFFSET: usize = 0x20;
    const TARGET_Z_OFFSET: usize = 0x24;

    unsafe {
        let x = this.add(TARGET_X_OFFSET).cast::<u32>().read_unaligned();
        let y = this.add(TARGET_Y_OFFSET).cast::<u32>().read_unaligned();
        let z = this.add(TARGET_Z_OFFSET).cast::<u32>().read_unaligned();
        output.write_unaligned(x);
        output.add(1).write_unaligned(y);
        output.add(2).write_unaligned(z);
    }
    output
});
