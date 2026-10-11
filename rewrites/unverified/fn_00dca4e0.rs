// original: 0x00DCA4E0 CTaskComplexCarReactToVehicleCollisionGetOut::CTaskComplexCarReactToVehicleCollisionGetOut

/// Initializes a collision-reaction task with five stack arguments.
///
/// `this` is the new task object in ECX. The constructor first delegates the
/// common task setup, then installs the get-out task vtable. It stores the two
/// leading references at `+0x18` and `+0x1c`, initializes the owned reference
/// at `+0x20`, copies three words from the third argument into `+0x30..+0x38`,
/// stores the fourth argument's float bits at `+0x40`, and stores the low byte
/// of the fifth argument at `+0x44`. Non-null references at `+0x18` and `+0x1c`
/// are released through the runtime callee; a non-null result from the slot
/// lookup is also released through the pointer slot at `+0x20`. The object is
/// returned in EAX and the thiscall callee removes all five stack arguments.
lf_checker_rt::export!(thiscall, rw_00dca4e0(this: u32, primary_ref: u32, source_ref: u32, snapshot: u32, blend: u32, mode: u32) -> u32 {
    unsafe {
        const BASE_CONSTRUCTOR: u32 = 1;
        const RELEASE_SLOT: u32 = 2;
        const LOOKUP_VEHICLE_SLOT: u32 = 3;
        const GETOUT_TASK_VTABLE_FILE_VA: u32 = 0x00EF5194;
        const PRIMARY_REF: u32 = 0x18;
        const SOURCE_REF: u32 = 0x1c;
        const OWNED_REF: u32 = 0x20;
        const SNAPSHOT: u32 = 0x30;
        const BLEND: u32 = 0x40;
        const MODE: u32 = 0x44;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn write_f32(address: u32, value: f32) {
            unsafe { write_u32(address, value.to_bits()) }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this);

        write_u32(this + PRIMARY_REF, primary_ref);
        write_u32(this, lf_checker_rt::relocated(GETOUT_TASK_VTABLE_FILE_VA));
        write_u32(this + SOURCE_REF, source_ref);
        write_u32(this + OWNED_REF, 0);

        write_u32(this + SNAPSHOT, read_u32(snapshot));
        write_u32(this + SNAPSHOT + 4, read_u32(snapshot + 4));
        write_u32(this + SNAPSHOT + 8, read_u32(snapshot + 8));
        write_f32(this + BLEND, f32::from_bits(blend));
        write_u8(this + MODE, mode as u8);

        if read_u32(this + PRIMARY_REF) != 0 {
            let slot = this + PRIMARY_REF;
            let value = read_u32(slot);
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_SLOT, u32, value, slot);
        }
        if read_u32(this + SOURCE_REF) != 0 {
            let slot = this + SOURCE_REF;
            let value = read_u32(slot);
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_SLOT, u32, value, slot);

            let new_owned = lf_checker_rt::callee_thiscall!(LOOKUP_VEHICLE_SLOT, u32, source_ref, 0u32);
            write_u32(this + OWNED_REF, new_owned);
            if new_owned != 0 {
                let owned_slot = this + OWNED_REF;
                let _ = lf_checker_rt::callee_thiscall!(RELEASE_SLOT, u32, new_owned, owned_slot);
            }
        }

        this
    }
});
