// original: 0x00E6D600 initialize_unarmed_weapon_hash
/// Calls the hash helper with the fixed image string and zero flags, stores the returned hash in the global result slot, and returns the same value in EAX.
const HASH_ID: u32 = 1;
const STRING_VA: u32 = 0x00EEEC8C;
const RESULT_VA: u32 = 0x017A4C60;
lf_checker_rt::export!(cdecl, rw_00e6d600() -> u32 {
    let value = lf_checker_rt::callee_cdecl!(HASH_ID, u32, lf_checker_rt::relocated(STRING_VA), 0);
    unsafe { lf_checker_rt::global::<u32>(RESULT_VA).write(value) };
    value
});
