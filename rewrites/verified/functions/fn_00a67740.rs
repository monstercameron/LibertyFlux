// original: 0x00A67740 CEventOnFire::~CEventOnFire

/// Allocate an event from the global event pool. A null allocation returns
/// zero; otherwise the base constructor runs with the new event in ECX, the
/// event vtable is restored, and the event pointer is returned. Both intercepted
/// helpers use thiscall with no stack arguments.
lf_checker_rt::export!(cdecl, rw_00a67740() -> u32 {
    const EVENT_POOL_GLOBAL: u32 = 0x016669C8;
    const EVENT_VTABLE_FILE_VA: u32 = 0x00E9DD14;
    const ALLOCATE_CALLEE: u32 = 1;
    const BASE_CONSTRUCTOR_CALLEE: u32 = 2;

    unsafe {
        let pool = lf_checker_rt::global::<u32>(EVENT_POOL_GLOBAL).read_unaligned();
        let event = lf_checker_rt::callee_thiscall!(ALLOCATE_CALLEE, u32, pool);
        if event == 0 {
            return 0;
        }

        let _ignored_base_result = lf_checker_rt::callee_thiscall!(
            BASE_CONSTRUCTOR_CALLEE,
            u32,
            event,
        );
        let event_vtable = lf_checker_rt::relocated(EVENT_VTABLE_FILE_VA);
        (event as *mut u32).write_unaligned(event_vtable);
        event
    }
});
