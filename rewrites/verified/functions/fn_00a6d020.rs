// original: 0x00A6D020 CTaskComplexPlayerInCover::~CTaskComplexPlayerInCover

/// Releases the optional object stored at `this + 0x14`, clears that field,
/// and tail-calls the base destructor.
///
/// The cleanup helper receives the stored object in ECX and the field address
/// as its stack argument. The base destructor receives `this` in ECX; its
/// scripted return is the result of this routine.
lf_checker_rt::export!(thiscall, rw_00a6d020(this: u32) -> u32 {
    unsafe {
        const RELEASE_FIELD: u32 = 1;
        const BASE_DESTRUCTOR: u32 = 2;
        const VTABLE: u32 = 0x00E9_F00C;
        const OWNED_OFFSET: u32 = 0x14;

        let owned = ((this + OWNED_OFFSET) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if owned != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_FIELD, u32, owned, this + OWNED_OFFSET);
            ((this + OWNED_OFFSET) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
