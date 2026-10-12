// original: 0x00DB7840 UIMusicViewer::~UIMusicViewer__deleting

const DELETING_FLAG: u8 = 1;

/// Runs the viewer's one-argument base cleanup, conditionally releases the
/// viewer allocation when the low bit of `delete_flag` is set, and returns
/// the original object pointer. The method uses thiscall: `this_ptr` is in
/// ECX, the 32-bit delete flag is on the stack, and the callee removes that
/// one stack argument.
lf_checker_rt::export!(thiscall, rw_00db7840(this_ptr: u32, delete_flag: u32) -> u32 {
    let _base_cleanup_result = lf_checker_rt::callee_thiscall!(1, u32, this_ptr);

    if (delete_flag as u8) & DELETING_FLAG != 0 {
        let _release_result = lf_checker_rt::callee_cdecl!(2, u32, this_ptr);
    }

    this_ptr
});
