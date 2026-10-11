// original: 0x009EF350 CPlayerPed::vf58 (stage: early guard exit)
/// Implements the initial flag-guard exit of `CPlayerPed::vf58`.
///
/// When bit 29 of the dword at player offset `0x29C` is set, the original
/// skips the update body, validates its stack cookie, and returns while
/// preserving the one stack argument. The checker contract for this stage
/// fixes that guard bit and compares the complete heap, stack adjustment,
/// termination, and cookie-check call. The cookie value in ECX is not
/// compared because it depends on the original and Rust stack frame addresses.
const PED_GUARD_FLAGS: u32 = 0x29C;
const PED_GUARD_BIT: u32 = 0x2000_0000;
const PED_MUTANT_FIELD: u32 = 0x26C;
const STACK_COOKIE_CHECK: u32 = 1;

#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_stage_009ef350(this: u32, _state: u32) {
    let flags = unsafe { (this.wrapping_add(PED_GUARD_FLAGS) as *const u32).read_unaligned() };
    if flags & PED_GUARD_BIT != 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(STACK_COOKIE_CHECK, u32, 0);
    }
}

#[unsafe(no_mangle)]
pub extern "thiscall" fn mut_stage_009ef350(this: u32, _state: u32) {
    let flags = unsafe { (this.wrapping_add(PED_GUARD_FLAGS) as *const u32).read_unaligned() };
    if flags & PED_GUARD_BIT != 0 {
        unsafe {
            let field = this.wrapping_add(PED_MUTANT_FIELD) as *mut u32;
            field.write_unaligned(field.read_unaligned() | 1);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(STACK_COOKIE_CHECK, u32, 0);
    }
}
