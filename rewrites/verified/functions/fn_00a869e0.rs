// original: 0x00A869E0 CCamTrans::~CCamTrans__deleting
/// Stores this class's relocated virtual-table pointer, runs the base
/// destructor, and conditionally releases the object through the global pool
/// when bit zero of the mode byte is set. The method returns the object.
///
/// The object pointer is in ECX and the one-word flags argument is on the
/// stack. The pool pointer is read only on the flagged path and is passed to
/// the intercepted release helper with the object pointer as its argument.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_a869e0(object: u32, flags: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00EA215C;
    const RELEASE_POOL_VA: u32 = 0x012FB1A0;
    const RELEASE_FLAG: u32 = 1;

    unsafe { (object as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_VA)); }
    lf_checker_rt::callee_thiscall!(1, u32, object);
    if flags & RELEASE_FLAG != 0 {
        let pool = unsafe { lf_checker_rt::global::<u32>(RELEASE_POOL_VA).read_unaligned() };
        lf_checker_rt::callee_thiscall!(2, u32, pool, object);
    }
    object
}
