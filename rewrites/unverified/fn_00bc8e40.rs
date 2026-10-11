// original: 0x00BC8E40 NativeImpl_APPLY_FORCE_TO_CAR

/// Look up the target vehicle for a force operation.
///
/// This proof covers the early exit when the scripted pool lookup finds no
/// vehicle. The native is a cdecl handler whose first argument is the vehicle
/// handle; the game vehicle pool is read through the relocated global at
/// `0x012E22A4`, then queried through the intercepted `GetAtHandle` call. A
/// null lookup returns a zero word before transform data, force vectors or
/// virtual dispatch are accessed. The non-null force and transform paths are
/// outside this proof. Calling convention: cdecl, 32-bit result.
lf_checker_rt::export!(cdecl, rw_00BC8E40(vehicle_handle: u32) -> u32 {
    unsafe {
        const VEHICLE_POOL_GLOBAL: u32 = 0x012e_22a4;
        const POOL_LOOKUP: u32 = 1;
        let pool = lf_checker_rt::global::<u32>(VEHICLE_POOL_GLOBAL).read_unaligned();
        let vehicle = lf_checker_rt::callee_thiscall!(
            POOL_LOOKUP,
            u32,
            pool,
            vehicle_handle,
        );
        if vehicle == 0 { 0 } else { vehicle }
    }
});
