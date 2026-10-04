// original: 0x008730e0 blend_child_slot_teardown
use lf_checker_rt::{callee_thiscall, export, relocated};

/// Tear down a blend-child slot: release the child, then swap the vtable.
///
/// Installs the live vtable, releases the child at +8 through the unlink
/// helper when one is attached, then installs the torn-down vtable.
/// EAX is untouched, so no return channel.
export!(thiscall, rw_008730e0(this_: u32) -> () {
    unsafe {
        *(this_ as *mut u32) = relocated(0x00FE7FB4);
        let child = *((this_.wrapping_add(8)) as *const u32);
        if child != 0 {
            callee_thiscall!(1, u32, child, this_);
        }
        *(this_ as *mut u32) = relocated(0x00E86AFC);
    }
});
