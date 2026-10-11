// original: 0x00A6D1B0 swim_task_cleanup

/// Releases the optional object at `this + 0x50`, destroys the subobject at
/// `this + 0x14`, and tail-calls the base destructor.
///
/// The first helper receives the stored pointer in ECX and the field address
/// as its stack argument. The embedded-object helper receives `this + 0x14`;
/// the tail-called destructor receives `this` and supplies EAX.
lf_checker_rt::export!(thiscall, rw_00a6d1b0(this: u32) -> u32 {
    unsafe {
        const RELEASE_FIELD: u32 = 1;
        const DESTROY_EMBEDDED: u32 = 2;
        const BASE_DESTRUCTOR: u32 = 3;
        const OWNED_OFFSET: u32 = 0x50;

        let owned = ((this + OWNED_OFFSET) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00E9_EF1C));
        if owned != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_FIELD, u32, owned, this + OWNED_OFFSET);
        }
        let _ = lf_checker_rt::callee_thiscall!(DESTROY_EMBEDDED, u32, this + 0x14);
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
