// original: 0x00A6D1F0 CEventAcquaintancePedDislike::~CEventAcquaintancePedDislike__deleting

/// Runs the real destructor and conditionally releases the object through
/// the event pool helper.
///
/// The low bit of `flags` controls the pool call. The pool pointer is read
/// from the relocated global slot; when enabled, the helper receives that
/// pointer in ECX and `this` as its one stack argument. The routine returns
/// `this` and pops the flags word.
lf_checker_rt::export!(thiscall, rw_00a6d1f0(this: u32, flags: u32) -> u32 {
    unsafe {
        const REAL_DESTRUCTOR: u32 = 1;
        const POOL_DELETE: u32 = 2;
        const EVENT_POOL_VA: u32 = 0x0166_69C8;

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00E9_ED04));
        let _ = lf_checker_rt::callee_thiscall!(REAL_DESTRUCTOR, u32, this);
        if flags as u8 & 1 != 0 {
            let pool = lf_checker_rt::global::<u32>(EVENT_POOL_VA).read();
            let _ = lf_checker_rt::callee_thiscall!(POOL_DELETE, u32, pool, this);
        }
        this
    }
});
