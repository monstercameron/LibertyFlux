// original: 0x00A6CFD0 CTaskComplexPlayerGun::~CTaskComplexPlayerGun

/// Releases the optional object at `this + 0x34`, destroys two owned
/// subobjects, and tail-calls the base destructor.
///
/// The first cleanup helper receives the stored object in ECX and the address
/// of its field as a stack argument; after it returns the field is cleared.
/// The two subsequent thiscalls receive `this` and `this + 0x14`. The final
/// base call is intercepted as a tail call and supplies the EAX result.
lf_checker_rt::export!(thiscall, rw_00a6cfd0(this: u32) -> u32 {
    unsafe {
        const RELEASE_FIELD: u32 = 1;
        const DESTROY_INTERNAL: u32 = 2;
        const RELEASE_SUBOBJECT: u32 = 3;
        const BASE_DESTRUCTOR: u32 = 4;
        const VTABLE: u32 = 0x00E9_EE6C;
        const OWNED_OFFSET: u32 = 0x34;

        let owned = ((this + OWNED_OFFSET) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if owned != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_FIELD, u32, owned, this + OWNED_OFFSET);
            ((this + OWNED_OFFSET) as *mut u32).write_unaligned(0);
        }
        let _ = lf_checker_rt::callee_thiscall!(DESTROY_INTERNAL, u32, this);
        let _ = lf_checker_rt::callee_thiscall!(RELEASE_SUBOBJECT, u32, this + 0x14);
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
