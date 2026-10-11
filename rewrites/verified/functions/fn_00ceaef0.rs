// original: 0x00CEAEF0 CBike::vf72
//! Clears one indexed word in a nested table when the object's byte index is
//! nonnegative, then forwards a pointer to the object's final helper.
//!
//! The object links through offsets `0x0DC4`, `+4`, and `+0x0C` to the state
//! block. Its signed byte index is at `0x1440`; `(an instruction of the original)` followed by JLE
//! skips negative values. A nonnegative index clears the dword at
//! `state[0x94] + index * 8 + 4` when that table pointer is non-null. The
//! final helper receives the state block in ECX and a pointer to
//! `[this + 0x0DC8] + 0x20` on the stack; its EAX is returned. Both calls are
//! scripted by the contract. The method uses thiscall with no stack args.
const TABLE_ROOT: usize = 0x0DC4;
const FINAL_ROOT: usize = 0x0DC8;
const SIGNED_INDEX: usize = 0x1440;
const STATE_TABLE_POINTER: usize = 0x94;
const TABLE_SLOT_STRIDE: usize = 8;
const TABLE_WORD_OFFSET: usize = 4;

#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00ceaef0(this: *mut u8) -> u32 {
    let initialize: unsafe extern "thiscall" fn(*mut u8) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(1) as usize)
    };
    let _ = unsafe { initialize(this) };

    let first = unsafe {
        core::ptr::read_unaligned(this.add(TABLE_ROOT).cast::<*mut u8>())
    };
    let second = unsafe {
        core::ptr::read_unaligned(first.add(4).cast::<*mut u8>())
    };
    let state = unsafe {
        core::ptr::read_unaligned(second.add(12).cast::<*mut u8>())
    };
    let index = unsafe { this.add(SIGNED_INDEX).read() as i8 };
    if index > -1 {
        let table = unsafe {
            core::ptr::read_unaligned(state.add(STATE_TABLE_POINTER).cast::<*mut u8>())
        };
        if !table.is_null() {
            let slot = (index as usize)
                .wrapping_mul(TABLE_SLOT_STRIDE)
                .wrapping_add(TABLE_WORD_OFFSET);
            unsafe { core::ptr::write_unaligned(table.add(slot).cast::<u32>(), 0) };
        }
    }

    let target = unsafe {
        core::ptr::read_unaligned(this.add(FINAL_ROOT).cast::<*mut u8>())
    };
    let finish: unsafe extern "thiscall" fn(*mut u8, *mut u8) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(2) as usize)
    };
    unsafe { finish(state, target.wrapping_add(0x20)) }
}
