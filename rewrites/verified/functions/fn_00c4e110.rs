// original: 0x00C4E110 CTaskComplexAimAndThrowProjectile::~CTaskComplexAimAndThrowProjectile
/// Set the derived-task type tag, release the optional owned object in the
/// field at `this + 0x14`, clear that field, and return the base destructor's
/// result. The release helper receives the owned object in ECX and the field
/// address as its only stack argument. The final base destructor is an E9
/// tail call; both callees are scripted by the proof contract.
lf_checker_rt::export!(thiscall, rw_c4e110(this: u32) -> u32 {
    unsafe {
        const TYPE_TAG_VA: u32 = 0x00ec_9efc;
        const OWNED_FIELD: u32 = 0x14;

        let field = this.wrapping_add(OWNED_FIELD);
        let object = (field as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TYPE_TAG_VA));
        if object != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, object, field);
            (field as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
