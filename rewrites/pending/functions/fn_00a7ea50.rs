// original: 0x00a7ea50 CComplexGunTaskInfo::vf6
/// Serialize `CComplexGunTaskInfo` after its base fields.
///
/// Writes one 3-bit field, three flag bytes and one 2-bit field,
/// threading one changed-flag byte through every helper. Returns the
/// helper's final answer with the flag in its low byte.
lf_checker_rt::export!(thiscall, rw_00a7ea50(this: u32, stream: u32) -> u32 {
    unsafe {
        let base = this as *const u8;
        let mut changed = lf_checker_rt::callee_thiscall!(1, u32, this, stream) as u8;
        let out = &mut changed as *mut u8 as u32;
        let f0 = (base.add(0x40) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(2, u32, stream, f0, 3, out);
        for off in [0x44u32, 0x45, 0x46] {
            let b = base.add(off as usize).read();
            lf_checker_rt::callee_thiscall!(3, u32, stream, b as u32, out);
        }
        let f1 = (base.add(0x48) as *const u32).read_unaligned();
        let last = lf_checker_rt::callee_thiscall!(2, u32, stream, f1, 2, out);
        (last & 0xffffff00) | changed as u32
    }
});
