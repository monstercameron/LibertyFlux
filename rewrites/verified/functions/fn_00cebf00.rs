// original: 0x00CEBF00 CPlane::vf51
//! Forwards an aircraft's current value to two virtual behavior helpers.
//!
//! The first helper receives only the object pointer; its return is ignored.
//! The method then reads the object's 32-bit field at offset `0x20` and passes
//! that value plus a zero mode word to the second helper, returning its EAX.
//! Both helper calls are intercepted by the proof contract. The method uses
//! the Windows x86 thiscall convention and has no stack arguments.
const VALUE_SLOT: usize = 0x20;

#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00cebf00(this: *mut u8) -> u32 {
    let first: unsafe extern "thiscall" fn(*mut u8) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(1) as usize)
    };
    let _ = unsafe { first(this) };

    let value = unsafe {
        core::ptr::read_unaligned(this.add(VALUE_SLOT).cast::<u32>())
    };
    let second: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(2) as usize)
    };
    unsafe { second(this, value, 0) }
}
