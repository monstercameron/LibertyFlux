// original: 0x00a7eb10 CComplexNewExitVehicleTaskInfo::vf6
/// Serialize `CComplexNewExitVehicleTaskInfo` after its base fields.
///
/// Writes the low bit of the `+0x30` byte and the `+0x38` word,
/// threading one changed-flag byte through both helpers. Returns the
/// helper's final answer with the flag in its low byte.
lf_checker_rt::export!(thiscall, rw_00a7eb10(this: u32, stream: u32) -> u32 {
    unsafe {
        let base = this as *const u8;
        let mut changed = lf_checker_rt::callee_thiscall!(1, u32, this, stream) as u8;
        let out = &mut changed as *mut u8 as u32;
        let bit = (base.add(0x30).read() & 1) as u32;
        lf_checker_rt::callee_thiscall!(2, u32, stream, bit, out);
        let w = (base.add(0x38) as *const u16).read_unaligned();
        let last = lf_checker_rt::callee_thiscall!(3, u32, stream, w as u32, out);
        (last & 0xffffff00) | changed as u32
    }
});
