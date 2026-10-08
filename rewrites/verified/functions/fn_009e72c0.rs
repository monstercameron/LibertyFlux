//! Proof scope: complete body under the declared valid object and scripted collaborator fixture.
//! Four call shapes and all configured comparisons passed; this does not prove arbitrary engine states.
//! Collaborator implementations are untested; no heap, stack or global writes occurred in the fixture.
//! Positive export always selects the tested return bias; the unused mutant export alone is removed.
//! The projection is not separately built. Earlier setup and return failures remain preserved.
//! An invalid historical receipt pointer is excluded; the final immutable build and run receipts are used.
#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn call_slot(receiver: u32, byte_offset: u32) -> u32 {
    let vtable = unsafe { read_word(receiver) };
    let target = unsafe { read_word(vtable.wrapping_add(byte_offset)) };
    let function: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(target as usize) };
    function(receiver)
}

fn implementation(this: u32, arg: u32, return_bias: u32) -> u32 {
    unsafe {
        const SLOT_40: u32 = 0xA0;
        const SLOT_56: u32 = 0xE0;
        const FALLBACK_FIELD: u32 = 0x100;
        const FALLBACK_RETURN: u32 = 0x20;

        let first = call_slot(this, SLOT_40);
        let value = if first != 0 {
            let next = call_slot(this, SLOT_40);
            call_slot(next, SLOT_56)
        } else {
            read_word(this.wrapping_add(FALLBACK_FIELD))
        };

        if value == 0 {
            read_word(this.wrapping_add(FALLBACK_RETURN)).wrapping_add(return_bias)
        } else {
            let first_arg = read_word(value.wrapping_add(4));
            let mapped = lf_checker_rt::callee_cdecl!(3, u32, first_arg, arg);
            lf_checker_rt::callee_thiscall!(4, u32, this, mapped).wrapping_add(return_bias)
        }
    }
}

lf_checker_rt::export!(thiscall, rw_009e72c0(this: u32, arg: u32) -> u32 {
    implementation(this, arg, 0x30)
});
