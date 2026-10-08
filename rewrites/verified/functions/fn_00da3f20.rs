/// Proof scope: Scripted direct and virtual helper bodies and live engine identity are excluded.
/// Two call-sequence shapes are covered; optional XMM/TLS exports are absent.
use lf_checker_rt::{callee_addr,export};
// original: 0x00da3f20
/// Run both direct gate helpers, then delegate through virtual slot 5 when the
/// second helper returns a zero low byte. Otherwise preserve its upper return
/// bits and set the low byte to one.

#[inline(always)]
unsafe fn thiscall_stub(id: u32, this_ptr: u32, arg: u32) -> u32 {
    let address = callee_addr(id);
    let call: extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(address as usize) };
    call(this_ptr, arg)
}

#[inline(always)]
unsafe fn virtual_slot_five(this_ptr: u32, arg: u32) -> u32 {
    let vtable = unsafe { (this_ptr as *const u32).read_unaligned() };
    let target = unsafe { (vtable as *const u32).add(5).read_unaligned() };
    let call: extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(target as usize) };
    call(this_ptr, arg)
}

#[inline(always)]
unsafe fn implementation(this_ptr: u32, arg: u32, invert_branch: bool) -> u32 {
    let _ignored = unsafe { thiscall_stub(1, this_ptr, arg) };
    let predicate = unsafe { thiscall_stub(2, this_ptr, arg) };
    let low_byte_nonzero = (predicate & 0xff) != 0;
    let call_virtual = if invert_branch { low_byte_nonzero } else { !low_byte_nonzero };
    if call_virtual {
        unsafe { virtual_slot_five(this_ptr, arg) }
    } else {
        (predicate & 0xffff_ff00) | 1
    }
}

export!(thiscall, rw_00da3f20(this_ptr: u32, arg: u32) -> u32 {
    unsafe { implementation(this_ptr, arg, false) }
});
