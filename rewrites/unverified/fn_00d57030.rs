// original: 0x00d57030 CCamViewports::vf0

/// Implements the deleting virtual for CCamViewports: call its destructor, release the object when bit zero of `delete_flag` is set, and return the original object pointer.
///
/// `this` is a 32-bit pointer to the CCamViewports object; the object uses the shared camera layout.
/// Original calling convention: thiscall with 1 explicit stack argument(s).
lf_checker_rt::export!(thiscall, rw_00d57030(this: u32, delete_flag: u32) -> u32 {
        #[inline(always)]
        unsafe fn read_global(file_va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
        }
        const CALLEE_CLASS_DESTRUCTOR: u32 = 1;
        const CALLEE_POOL_RELEASE: u32 = 2;
        const DELETE_FLAG_MASK: u32 = 1;
        const POOL_GLOBAL_FILE_VA: u32 = 0x012fb1a0;
        let _ = lf_checker_rt::callee_thiscall!(CALLEE_CLASS_DESTRUCTOR, u32, this);
        if delete_flag & DELETE_FLAG_MASK != 0 {
            let allocator = unsafe { read_global(POOL_GLOBAL_FILE_VA) };
            let _ = lf_checker_rt::callee_thiscall!(CALLEE_POOL_RELEASE, u32, allocator, this);
        }
        this
});
