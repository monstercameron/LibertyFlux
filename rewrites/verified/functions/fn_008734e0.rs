// original: 0x008734e0 request_node_build_and_link
use lf_checker_rt::{callee_thiscall, export};

/// Build a request node through the vtable factory and link it in.
///
/// Creates the node with the two arguments, steps the inner object,
/// releases any previous child, links the new node at +8 of the inner
/// object and notifies through the vtable. A null creation still releases
/// the old child. Returns the created node (zero on failure).
export!(thiscall, rw_008734e0(this_: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        let vt = *(this_ as *const u32);
        let create: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(8)) as *const u32));
        let node = create(this_, a0, a1);
        let inner = this_.wrapping_add(4);
        if node == 0 {
            let child = *((inner.wrapping_add(8)) as *const u32);
            if child != 0 {
                callee_thiscall!(3, u32, child, inner);
            }
            0
        } else {
            let vt = *(inner as *const u32);
            let step: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(*((vt.wrapping_add(4)) as *const u32));
            step(inner);
            let child = *((inner.wrapping_add(8)) as *const u32);
            if child != 0 {
                callee_thiscall!(3, u32, child, inner);
            }
            *((inner.wrapping_add(8)) as *mut u32) = node;
            *((inner.wrapping_add(0xc)) as *mut u32) =
                *((node.wrapping_add(0x14)) as *const u32);
            *((node.wrapping_add(0x14)) as *mut u32) = inner;
            let vt = *(inner as *const u32);
            let notify: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(*((vt.wrapping_add(0xc)) as *const u32));
            let msg = [0x10001u32, 0u32];
            notify(inner, msg.as_ptr() as u32);
            node
        }
    }
});
