// original: 0x00B12220 release_entity_reference_if_present

/// If `this + 0xd0` contains a reference, release it with mode 3. A null
/// reference is ignored. The function returns no value and uses thiscall.
lf_checker_rt::export!(thiscall, rw_00b12220(this: u32) -> () {
    const REFERENCE: u32 = 0xD0;
    let reference = unsafe { ((this.wrapping_add(REFERENCE)) as *const u32).read_unaligned() };
    if reference != 0 {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, reference, 3u32);
    }
});
