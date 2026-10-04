// original: 0x00873100 blend_slot_attach_node
use lf_checker_rt::{callee_thiscall, export};

/// Attach a node to a blend slot: release the old child, link the new one.
///
/// Releases the current child at +8 (before and after stepping the vtable),
/// links the argument node in, then notifies through the vtable with a
/// two-word message. Returns the notify answer.
export!(thiscall, rw_00873100(this_: u32, node: u32) -> u32 {
    unsafe {
        let child = *((this_.wrapping_add(8)) as *const u32);
        if child != 0 {
            callee_thiscall!(1, u32, child, this_);
        }
        let vt = *(this_ as *const u32);
        let step: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(4)) as *const u32));
        step(this_);
        let child = *((this_.wrapping_add(8)) as *const u32);
        if child != 0 {
            callee_thiscall!(1, u32, child, this_);
        }
        *((this_.wrapping_add(8)) as *mut u32) = node;
        *((this_.wrapping_add(0xc)) as *mut u32) =
            *((node.wrapping_add(0x14)) as *const u32);
        *((node.wrapping_add(0x14)) as *mut u32) = this_;
        let vt = *(this_ as *const u32);
        let notify: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(0xc)) as *const u32));
        let msg = [0x10001u32, 0u32];
        notify(this_, msg.as_ptr() as u32)
    }
});
