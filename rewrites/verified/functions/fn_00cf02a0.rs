// original: 0x00CF02A0 CHeli::vf100
//! Updates each entry in the object's element array while its signed count is
//! positive. The array pointer is at offset `0x0F80`, the signed count is at
//! `0x0F84`, and entries are `0x170` bytes apart.
//!
//! Each update helper receives the current entry pointer and a zero word. The
//! count is read again after each helper call, so the loop terminates against
//! the latest signed count. EAX returns that count, including its original
//! bits when the initial count is nonpositive. The thiscall method has no
//! stack arguments; the helper call is intercepted by the contract.
const ELEMENTS_FIELD: usize = 0x0F80;
const COUNT_FIELD: usize = 0x0F84;
const ELEMENT_STRIDE: usize = 0x170;

#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00cf02a0(this: *mut u8) -> u32 {
    let mut count = unsafe {
        core::ptr::read_unaligned(this.add(COUNT_FIELD).cast::<i32>())
    };
    if count > 0 {
        let mut index = 0i32;
        let mut byte_offset = 0usize;
        let update: unsafe extern "thiscall" fn(*mut u8, u32) -> u32 = unsafe {
            core::mem::transmute(lf_checker_rt::callee_addr(1) as usize)
        };
        loop {
            let element = if index >= count {
                core::ptr::null_mut()
            } else {
                let elements = unsafe {
                    core::ptr::read_unaligned(this.add(ELEMENTS_FIELD).cast::<*mut u8>())
                };
                elements.wrapping_add(byte_offset)
            };
            let _ = unsafe { update(element, 0) };
            count = unsafe {
                core::ptr::read_unaligned(this.add(COUNT_FIELD).cast::<i32>())
            };
            index = index.wrapping_add(1);
            byte_offset = byte_offset.wrapping_add(ELEMENT_STRIDE);
            if index >= count {
                break;
            }
        }
    }
    count as u32
}
