// original: 0x00C38BA0 CTrain::vf0

/// Run the train base destructor, then release the object from the shared pool
/// when the low bit of the deleting-destructor flag is set. The pool pointer
/// comes from the relocated global at file address `0x012e22a4`; the object
/// pointer is passed as the pool method's sole stack argument. Return the
/// original object pointer. This thiscall method takes one 32-bit flag.
lf_checker_rt::export!(thiscall, rw_00c38ba0(
    this: *mut u8,
    deleting_flag: u32,
) -> u32 {
    const POOL_GLOBAL_VA: u32 = 0x012e22a4;
    const BASE_DESTRUCTOR: u32 = 1;
    const POOL_RELEASE: u32 = 2;

    let object = this as u32;
    unsafe {
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, object);
        if deleting_flag as u8 & 1 != 0 {
            let pool = lf_checker_rt::global::<u32>(POOL_GLOBAL_VA).read_unaligned();
            let _: u32 = lf_checker_rt::callee_thiscall!(POOL_RELEASE, u32, pool, object);
        }
    }
    object
});
