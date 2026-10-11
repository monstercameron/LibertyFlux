// original: 0x00A6D0F0 CTaskComplexPlayerPlaceCarBomb::~CTaskComplexPlayerPlaceCarBomb

/// Releases the optional object at `this + 0x14`, destroys the embedded
/// object at `this + 0x1c`, and tail-calls the base destructor.
///
/// The cleanup helper receives the stored object and field address. The
/// embedded-object helper receives `this + 0x1c`; the tail-called base
/// destructor receives `this` and supplies the EAX result.
lf_checker_rt::export!(thiscall, rw_00a6d0f0(this: u32) -> u32 {
    unsafe {
        const RELEASE_FIELD: u32 = 1;
        const DESTROY_EMBEDDED: u32 = 2;
        const BASE_DESTRUCTOR: u32 = 3;
        const VTABLE: u32 = 0x00E9_F064;
        const OWNED_OFFSET: u32 = 0x14;

        let owned = ((this + OWNED_OFFSET) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if owned != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_FIELD, u32, owned, this + OWNED_OFFSET);
            ((this + OWNED_OFFSET) as *mut u32).write_unaligned(0);
        }
        let _ = lf_checker_rt::callee_thiscall!(DESTROY_EMBEDDED, u32, this + 0x1C);
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
