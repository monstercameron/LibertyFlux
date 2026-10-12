// original: 0x006559b0 initialize_effect_scheduler
use lf_checker_rt::{export, global, relocated, tls_slot};

/// Initialize the process-wide effect scheduler once. The manager's TLS
/// allocator supplies the object, three direct helpers initialize and
/// register it, and the guard global prevents a second initialization.
/// The exported entry has no arguments and uses the cdecl convention.
export!(cdecl, rw_006559b0() -> u32 {
    const GUARD_VA: u32 = 0x018B_7988;
    const MANAGER_VA: u32 = 0x01BB_5520;
    const SCHEDULER_VTABLE_VA: u32 = 0x00F9_A530;
    const CALLBACK_A_VA: u32 = 0x0065_6180;
    const CALLBACK_B_VA: u32 = 0x0063_93C0;
    const TLS_MANAGER_FIELD: usize = 0x08;
    const ALLOCATOR_SLOT: usize = 2;
    const SCHEDULER_SIZE: u32 = 0x120;

    unsafe {
        let guard = global::<u32>(GUARD_VA);
        if guard.read_unaligned() != 0 {
            return 0;
        }

        let tls_context = tls_slot(0) as *mut u8;
        let tls_manager = tls_context
            .add(TLS_MANAGER_FIELD)
            .cast::<*mut u8>()
            .read_unaligned();
        let allocator_vtable = tls_manager.cast::<*mut u32>().read_unaligned();
        let _allocator_entry = allocator_vtable.add(ALLOCATOR_SLOT).read_unaligned();
        let allocated = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            tls_manager as u32,
            0x40u32,
            0x10u32,
            0u32
        );
        let scheduler = if allocated != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, allocated)
        } else {
            0
        };
        guard.write_unaligned(scheduler);

        let scheduler_ptr = scheduler as *mut u8;
        scheduler_ptr.add(0x04).cast::<u32>().write_unaligned(relocated(SCHEDULER_VTABLE_VA));
        scheduler_ptr.add(0x08).cast::<u32>().write_unaligned(0);
        scheduler_ptr.add(0x0C).cast::<u32>().write_unaligned(0);
        scheduler_ptr.add(0x10).cast::<u32>().write_unaligned(SCHEDULER_SIZE);
        scheduler_ptr.add(0x1C).cast::<u32>().write_unaligned(0);
        scheduler_ptr.add(0x20).cast::<u32>().write_unaligned(0);
        scheduler_ptr.add(0x24).cast::<u32>().write_unaligned(relocated(CALLBACK_A_VA));
        scheduler_ptr.add(0x28).cast::<u32>().write_unaligned(0);
        scheduler_ptr.add(0x2C).cast::<u32>().write_unaligned(relocated(CALLBACK_B_VA));

        lf_checker_rt::callee_thiscall!(3, u32, scheduler);
        let manager = global::<*mut u8>(MANAGER_VA).read_unaligned();
        let mut guard_address_snapshot = guard as u32;
        let mut vtable_snapshot = scheduler_ptr.add(0x04).cast::<u32>().read_unaligned();
        lf_checker_rt::callee_thiscall!(
            4,
            u32,
            manager.add(0x18) as u32,
            &mut vtable_snapshot as *mut u32 as u32,
            &mut guard_address_snapshot as *mut u32 as u32,
            guard as u32
        )
    }
});
