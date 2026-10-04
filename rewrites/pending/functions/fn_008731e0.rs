// original: 0x008731e0 request_node_init
use lf_checker_rt::{export, relocated};

/// Initialise a request-node object: clear state words, install vtable, return this.
///
/// Leaf thiscall/0 constructor; also clears the link words at +0x10/+0x14.
export!(thiscall, rw_008731e0(this_: u32) -> u32 {
    unsafe {
        let base = this_ as *mut u32;
        *base.add(1) = 0;
        *base.add(2) = 0;
        *base.add(3) = 0;
        *base = relocated(0x00FE7FA0);
        *base.add(5) = 0;
        *base.add(4) = 0;
        this_
    }
});
