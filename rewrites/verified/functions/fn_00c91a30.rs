// original: 0x00C91A30 CSimpleIkManager::vf0

/// Implements the scalar-deleting destructor wrapper for CSimpleIkManager. The object pointer arrives in ECX and one destruction-flags word is on the stack. The wrapper first invokes the base destructor, conditionally sends the object pointer to the release helper when flag bit zero is set, returns the original object pointer in EAX, and removes four stack bytes.
lf_checker_rt::export!(thiscall, rw_00c91a30(this: u32, flags: u32) -> u32 {
    const DELETE_FLAG: u32 = 1;
    const BASE_DESTRUCTOR: u32 = 1;
    const RELEASE_HELPER: u32 = 2;
    let _base_result = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this);
    if flags & DELETE_FLAG != 0 {
        let _release_result = lf_checker_rt::callee_cdecl!(RELEASE_HELPER, u32, this);
    }
    this
});
