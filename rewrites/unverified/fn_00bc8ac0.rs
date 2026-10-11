// original: 0x00BC8AC0 Vehicle_TestInArea

/// Test whether a vehicle's stored position lies inside an axis-aligned area.
///
/// The cdecl arguments are an area mode, a vehicle handle, three lower-bound
/// coordinates, three upper-bound coordinates, and a debug flag. This proof
/// exercises mode zero, which tests the inclusive X/Y rectangle. It sorts
/// each pair of bounds before comparing them. The vehicle pool lookup is
/// intercepted by the checker; the returned object points at its position
/// vector through field `+0x20`, whose X and Y coordinates are at `+0x30` and
/// `+0x34`. The original overwrites the incoming Z-bound words as scratch in
/// this mode, so stack comparison is disabled and the final return observes
/// only the normalized X/Y bounds. Calling convention: cdecl, low-byte bool.
lf_checker_rt::export!(cdecl, rw_00BC8AC0(
    mode: u32,
    vehicle_handle: u32,
    first_x_bits: u32,
    first_y_bits: u32,
    first_z_bits: u32,
    second_x_bits: u32,
    second_y_bits: u32,
    second_z_bits: u32,
    debug: u32,
) -> u32 {
    unsafe {
        const VEHICLE_POOL_GLOBAL: u32 = 0x012e_22a4;
        const VEHICLE_POSITION_FIELD: u32 = 0x20;
        const POSITION_X: u32 = 0x30;
        const POSITION_Y: u32 = 0x34;
        const POOL_LOOKUP: u32 = 1;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let pool = read_u32(lf_checker_rt::global::<u32>(VEHICLE_POOL_GLOBAL) as u32);
        let vehicle = lf_checker_rt::callee_thiscall!(
            POOL_LOOKUP,
            u32,
            pool,
            vehicle_handle,
        );
        let vector = read_u32(vehicle.wrapping_add(VEHICLE_POSITION_FIELD));
        let x = f32::from_bits(read_u32(vector.wrapping_add(POSITION_X)));
        let y = f32::from_bits(read_u32(vector.wrapping_add(POSITION_Y)));

        let mut first_x = f32::from_bits(first_x_bits);
        let mut second_x = f32::from_bits(second_x_bits);
        let mut first_y = f32::from_bits(first_y_bits);
        let mut second_y = f32::from_bits(second_y_bits);
        if first_x > second_x {
            core::mem::swap(&mut first_x, &mut second_x);
        }
        if first_y > second_y {
            core::mem::swap(&mut first_y, &mut second_y);
        }

        let _ = (mode, first_z_bits, second_z_bits, debug);
        u32::from(x >= first_x && x <= second_x && y >= first_y && y <= second_y)
    }
});
