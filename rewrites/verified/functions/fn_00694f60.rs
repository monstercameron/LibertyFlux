// original: 0x00694F60 rage::crWeightSet::~crWeightSet__deleting (proposed)

/// Run the base weight-set destructor, then release the object through the
/// thread-local memory manager when deletion flag bit zero is set and the
/// receiver is nonnull. The release dispatch takes the receiver as its sole
/// explicit argument. The function returns the original receiver pointer.
lf_checker_rt::export!(thiscall, rw_00694F60(receiver: u32, delete_flags: u32) -> u32 {
    unsafe {
        const BASE_DESTRUCTOR_ID: u32 = 1;
        const MEMORY_RELEASE_ID: u32 = 2;
        const DELETE_FLAG: u32 = 0x01;

        let _ = unsafe { lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR_ID, u32, receiver) };
        if receiver != 0 && delete_flags & DELETE_FLAG != 0 {
            // The checker stand-in models the manager's virtual release call.
            let _ = unsafe { lf_checker_rt::callee_thiscall!(MEMORY_RELEASE_ID, u32, receiver, receiver) };
        }
        receiver
    }
});
