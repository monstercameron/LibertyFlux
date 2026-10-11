// original: 0x00c41ea0 CCamIntermezzo::notify_linked_cam
/// Walk linked camera candidates from index zero, stopping when the lookup
/// returns null or a candidate's link at +0x150 points back to this camera.
/// Notify the first matching candidate with one zero stack argument. Lookup
/// receives this, type 3, the current index and state 1.
lf_checker_rt::export!(thiscall, rw_00c41ea0(this: u32) -> u32 {
    const FIND_CANDIDATE: u32 = 1;
    const NOTIFY_CANDIDATE: u32 = 2;
    const LINKED_CAMERA: u32 = 0x150;
    let mut index = 0u32;
    loop {
        let candidate = lf_checker_rt::callee_thiscall!(FIND_CANDIDATE, u32, this, 3, index, 1);
        index = index.wrapping_add(1);
        if candidate == 0 {
            break;
        }
        let linked = unsafe { ((candidate + LINKED_CAMERA) as *const u32).read_unaligned() };
        if linked == this {
            let _ = lf_checker_rt::callee_thiscall!(NOTIFY_CANDIDATE, u32, candidate, 0);
            break;
        }
    }
    0
});
