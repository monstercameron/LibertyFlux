// original: 0x009b08c0 audPoliceScanner::audPoliceScanner
/// Constructs the police scanner after its base constructor returns.
///
/// The thiscall wrapper invokes the intercepted base constructor, installs
/// the relocated police-scanner vtable, clears the words at offsets `0x60`
/// and `0x5c`, and returns the receiver.
const BASE_CONSTRUCTOR: u32 = 1;
const POLICE_SCANNER_VTABLE_VA: u32 = 0x00E9_3C58;
const FIELD_A: u32 = 0x60;
const FIELD_B: u32 = 0x5C;

export!(thiscall, rw_009b08c0(this_: u32) -> u32 {
    let _base_result = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this_);
    unsafe {
        (this_ as *mut u32).write(lf_checker_rt::relocated(POLICE_SCANNER_VTABLE_VA));
        (this_.wrapping_add(FIELD_A) as *mut u32).write(0);
        (this_.wrapping_add(FIELD_B) as *mut u32).write(0);
    }
    this_
});
