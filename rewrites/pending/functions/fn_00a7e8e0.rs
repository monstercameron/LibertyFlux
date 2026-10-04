// original: 0x00a7e8e0 CAnimTaskInfo::vf6
/// Serialize the `CAnimTaskInfo` fields into the network bit stream.
///
/// Writes the two body fields with 16 bits each, then a fixed pattern of
/// bit counts, threading one changed-flag byte through every helper.
/// Returns the helper's final answer with the flag in its low byte.
lf_checker_rt::export!(thiscall, rw_00a7e8e0(this: u32, stream: u32) -> u32 {
    unsafe {
        let base = this as *const u8;
        let f0 = (base.add(0x14) as *const u32).read_unaligned();
        let f1 = (base.add(0x18) as *const u32).read_unaligned();
        let mut changed: u8 = 0;
        let out = &mut changed as *mut u8 as u32;
        lf_checker_rt::callee_thiscall!(1, u32, stream, f0, 0x10, out);
        lf_checker_rt::callee_thiscall!(1, u32, stream, f1, 0x10, out);
        lf_checker_rt::callee_thiscall!(2, u32, stream, 8, 1);
        lf_checker_rt::callee_thiscall!(3, u32, stream, out);
        lf_checker_rt::callee_thiscall!(2, u32, stream, 3, 1);
        lf_checker_rt::callee_thiscall!(2, u32, stream, 8, 1);
        let last = lf_checker_rt::callee_thiscall!(2, u32, stream, 0x1f, 1);
        (last & 0xffffff00) | changed as u32
    }
});
