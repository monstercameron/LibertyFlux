// original: 0x00872e40 blend_list_insert
use lf_checker_rt::{callee_thiscall, export};

/// Insert a node into a blend list, by index or at the head.
///
/// With index zero, links the node at the head of this list. Otherwise
/// resolves slot index-1 through the vtable and splices the node after it.
/// Reference count at +4 is bumped before linking. Returns the previous
/// head, or the link that used to follow the slot (zero on a lookup miss).
export!(thiscall, rw_00872e40(this_: u32, index: u32, node: u32) -> u32 {
    unsafe {
        if index == 0 {
            let count = (node.wrapping_add(4)) as *mut u16;
            *count = (*count).wrapping_add(1);
            callee_thiscall!(1, u32, node);
            let head = *((this_.wrapping_add(0x1c)) as *const u32);
            if head != 0 {
                *((node.wrapping_add(0x10)) as *mut u32) = head;
            }
            *((this_.wrapping_add(0x1c)) as *mut u32) = node;
            *((node.wrapping_add(0xc)) as *mut u32) = this_;
            head
        } else {
            let vt = *(this_ as *const u32);
            let slot_at: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(*((vt.wrapping_add(0x30)) as *const u32));
            let slot = slot_at(this_, index.wrapping_sub(1));
            if slot == 0 {
                0
            } else {
                let count = (node.wrapping_add(4)) as *mut u16;
                *count = (*count).wrapping_add(1);
                callee_thiscall!(1, u32, node);
                let next = *((slot.wrapping_add(0x10)) as *const u32);
                *((node.wrapping_add(0x10)) as *mut u32) = next;
                *((node.wrapping_add(0xc)) as *mut u32) = this_;
                *((slot.wrapping_add(0x10)) as *mut u32) = node;
                next
            }
        }
    }
});
