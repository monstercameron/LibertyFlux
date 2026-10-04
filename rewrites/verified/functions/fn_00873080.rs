// original: 0x00873080 blend_child_slot_init
use lf_checker_rt::{export, relocated};

/// Initialise a blend-child slot: clear words +4/+8/+0xc, install vtable, return this.
///
/// Original is a leaf thiscall/0 constructor.
export!(thiscall, rw_00873080(this_: u32) -> u32 {
    unsafe {
        let base = this_ as *mut u32;
        *base.add(1) = 0;
        *base = relocated(0x00FE7FB4);
        *base.add(2) = 0;
        *base.add(3) = 0;
        this_
    }
});
