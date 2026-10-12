// original: 0x0065F330 sn_migrate_claim_slot

/// Atomically claims the object's slot by comparing its current value with
/// one and replacing it with four. The Win32 compare-exchange call returns
/// the prior value; only a prior value of one causes the object's word at
/// offset four to be set to the all-bits-set sentinel. The thiscall function
/// returns the prior value unchanged.
lf_checker_rt::export!(thiscall, rw_0065f330(object: *mut u8) -> u32 {
    const OBJECT: u32 = object as u32;
    const SLOT_OFFSET: u32 = 4;
    const EXCHANGE_VALUE: u32 = 4;
    const COMPARE_VALUE: u32 = 1;
    const EMPTY_SLOT: u32 = u32::MAX;

    let previous_value = lf_checker_rt::callee_stdcall!(
        1,
        u32,
        OBJECT,
        EXCHANGE_VALUE,
        COMPARE_VALUE,
    );
    if previous_value == COMPARE_VALUE {
        unsafe { *((OBJECT + SLOT_OFFSET) as *mut u32) = EMPTY_SLOT; }
    }
    previous_value
});
