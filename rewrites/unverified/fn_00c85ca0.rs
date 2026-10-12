// original: 0x00C85CA0 gtaFragType::~gtaFragType__deleting

/// Destroy the fragment receiver, optionally release its allocation when the
/// low bit of the deleting flag is set, and return the original receiver.
/// The deleting flag is the single 32-bit thiscall stack argument.
lf_checker_rt::export!(thiscall, rw_00c85ca0(this: u32, deleting_flag: u32) -> u32 {
    const DESTROY_FRAGMENT: u32 = 1;
    const RELEASE_ALLOCATION: u32 = 2;

    let _ = lf_checker_rt::callee_thiscall!(DESTROY_FRAGMENT, u32, this);
    if deleting_flag & 1 != 0 {
        let _ = lf_checker_rt::callee_cdecl!(RELEASE_ALLOCATION, u32, this);
    }
    this
});
