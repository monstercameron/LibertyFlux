// original: 0x00A66D70 CEventOnFire::~CEventOnFire__deleting

/// Restore the event's vtable, call the base destructor with the event in ECX,
/// and optionally invoke the manager's delete method when flag bit 0 is set.
/// The manager pointer is loaded from the event-pool global. Both helper calls
/// are thiscall; the outer deleting destructor consumes one stack flag and
/// returns the event pointer in EAX.
lf_checker_rt::export!(thiscall, rw_00a66d70(event: u32, flags: u32) -> u32 {
    const EVENT_VTABLE: u32 = 0x00E9DD14;
    const EVENT_POOL_GLOBAL: u32 = 0x016669C8;
    const DELETE_FLAG: u8 = 1;
    const BASE_DESTRUCTOR_CALLEE: u32 = 1;
    const DELETE_CALLEE: u32 = 2;

    unsafe {
        (event as *mut u32).write_unaligned(EVENT_VTABLE);
        let _ignored_base_result = lf_checker_rt::callee_thiscall!(
            BASE_DESTRUCTOR_CALLEE,
            u32,
            event,
        );

        if (flags as u8) & DELETE_FLAG != 0 {
            let manager = lf_checker_rt::global::<u32>(EVENT_POOL_GLOBAL).read_unaligned();
            let _ignored_delete_result = lf_checker_rt::callee_thiscall!(
                DELETE_CALLEE,
                u32,
                manager,
                event,
            );
        }
        event
    }
});
