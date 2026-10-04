// original: 0x00a7eb60 CComplexNewGetInVehicleTaskInfo::vf6
/// Serialize the `CComplexNewGetInVehicleTaskInfo` flag byte.
///
/// Runs the base serializer, writes the `+0x30` byte through a helper
/// sharing the changed-flag byte, and returns the helper's answer
/// with the flag in its low byte.
lf_checker_rt::export!(thiscall, rw_00a7eb60(this: u32, stream: u32) -> u32 {
    unsafe {
        let mut changed = lf_checker_rt::callee_thiscall!(1, u32, this, stream) as u8;
        let b = (this as *const u8).add(0x30).read();
        let last = lf_checker_rt::callee_thiscall!(2, u32, stream, b as u32,
            &mut changed as *mut u8 as u32);
        (last & 0xffffff00) | changed as u32
    }
});
