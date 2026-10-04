// original: 0x00873160 blend_slot_attach_payload
use lf_checker_rt::{callee_thiscall, export};

/// Attach a node's payload to a blend slot, when the node has one.
///
/// Reads the payload link at node+8; a null link ends the call early and
/// the argument is returned. Otherwise behaves like `rw_00873100` with the
/// payload as the node. Returns the argument on the early path, else the
/// notify answer.
export!(thiscall, rw_00873160(this_: u32, node: u32) -> u32 {
    unsafe {
        let child = *((this_.wrapping_add(8)) as *const u32);
        if child != 0 {
            callee_thiscall!(1, u32, child, this_);
        }
        let payload = *((node.wrapping_add(8)) as *const u32);
        if payload == 0 {
            return node;
        }
        let vt = *(this_ as *const u32);
        let step: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(4)) as *const u32));
        step(this_);
        let child = *((this_.wrapping_add(8)) as *const u32);
        if child != 0 {
            callee_thiscall!(1, u32, child, this_);
        }
        *((this_.wrapping_add(8)) as *mut u32) = payload;
        *((this_.wrapping_add(0xc)) as *mut u32) =
            *((payload.wrapping_add(0x14)) as *const u32);
        *((payload.wrapping_add(0x14)) as *mut u32) = this_;
        let vt = *(this_ as *const u32);
        let notify: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(0xc)) as *const u32));
        let msg = [0x10001u32, 0u32];
        notify(this_, msg.as_ptr() as u32)
    }
});
