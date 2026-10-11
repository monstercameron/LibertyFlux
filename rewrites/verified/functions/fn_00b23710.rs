// original: 0x00B23710 physics_dispatch_slot_90

/// Checks the entity's physics record and indexed runtime-table flags, calls
/// the pre-dispatch helper when the low two flag bits equal one, and queries
/// the physics object twice. A null record or null first lookup returns
/// without dispatch. Otherwise it tail-dispatches the forwarded word through
/// the returned object's named vtable slot. The record index is read as an
/// unsigned 16-bit value.
lf_checker_rt::export!(thiscall, rw_bb23710(this: u32, forwarded: u32) -> () {
    const PHYSICS_RECORD: u32 = 0x38;
    const RECORD_INDEX: u32 = 0x08;
    const TABLE_ROOT_GLOBAL: u32 = 0x12B9_C78;
    const TABLE_POINTER: u32 = 0x70;
    const ENTRY_STRIDE: u32 = 8;
    const ENTRY_FLAGS: u32 = 4;
    const PRE_DISPATCH_FLAG: u32 = 1;
    const LOOKUP_SLOT: u32 = 2;
    const OBJECT_DISPATCH_SLOT: u32 = 3;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let physics = read_u32(this.wrapping_add(PHYSICS_RECORD));
        if physics == 0 {
            return;
        }
        let index = read_u32(physics.wrapping_add(RECORD_INDEX)) & 0xFFFF;
        let root = lf_checker_rt::global::<u32>(TABLE_ROOT_GLOBAL).read_unaligned();
        let table = read_u32(root.wrapping_add(TABLE_POINTER));
        let entry = table
            .wrapping_add(index.wrapping_mul(ENTRY_STRIDE))
            .wrapping_add(ENTRY_FLAGS);
        if read_u32(entry) & 3 == PRE_DISPATCH_FLAG {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        }
        if lf_checker_rt::callee_thiscall!(LOOKUP_SLOT, u32, this) == 0 {
            return;
        }
        let object = lf_checker_rt::callee_thiscall!(LOOKUP_SLOT, u32, this);
        let _ = lf_checker_rt::callee_thiscall!(OBJECT_DISPATCH_SLOT, u32, object, forwarded);
    }
});
