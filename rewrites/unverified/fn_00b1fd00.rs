// original: 0x00b1fd00 init_cell_state (proposed)

/// Initialises the object's cell state block and runs two passes.
///
/// Thiscall with no stack arguments. Writes the magic 0x3FA66666 to
/// +0x18274, runs two thiscall passes with no stack arguments, zeroes
/// +0x40, +0x18270, +0x18260, +0x18264, +0x18268 and +0x1826C (the last
/// is filled from the original's uninitialised stack scratch, which the
/// checker defines as zero), then writes 0x3F490FDB to +0x34, 1.0 to
/// +0x38 and 0 to +0x3C. Returns the second pass result.
lf_checker_rt::export!(thiscall, rw_00b1fd00(this: u32) -> u32 {
    unsafe {
        const MAGIC: u32 = 0x3fa66666;
        const SLOPE: u32 = 0x3f490fdb;
        const ONE: u32 = 0x3f800000;
        ((this + 0x18274) as *mut u32).write_unaligned(MAGIC);
        lf_checker_rt::callee_thiscall!(1, u32, this);
        let result: u32 = lf_checker_rt::callee_thiscall!(2, u32, this);
        for off in [0x40u32, 0x18270, 0x18260, 0x18264, 0x18268, 0x1826c, 0x3c] {
            ((this + off) as *mut u32).write_unaligned(0);
        }
        ((this + 0x34) as *mut u32).write_unaligned(SLOPE);
        ((this + 0x38) as *mut u32).write_unaligned(ONE);
        result
    }
});
