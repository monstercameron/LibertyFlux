// original: 0x00E6D0E0 boat_shared_record_table_initialize
/// Calls the intercepted record constructor for eight global objects at a 0x50-byte stride, in ascending order, and returns the final call's EAX value.
const FIRST_OBJECT_VA: u32 = 0x017A3930;
const OBJECT_STRIDE: u32 = 0x50;
const OBJECT_COUNT: u32 = 8;
const CONSTRUCTOR_ID: u32 = 1;
lf_checker_rt::export!(cdecl, rw_00e6d0e0() -> u32 {
    let mut result = 0;
    for index in 0..OBJECT_COUNT {
        let object_va = FIRST_OBJECT_VA.wrapping_add(index * OBJECT_STRIDE);
        result = lf_checker_rt::callee_thiscall!(CONSTRUCTOR_ID, u32, lf_checker_rt::relocated(object_va));
    }
    result
});
