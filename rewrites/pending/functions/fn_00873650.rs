// original: 0x00873650 blend_node_init
use lf_checker_rt::{export, relocated};

/// Initialise a blend-node object with a 16-bit tag taken from the argument.
///
/// Clears the header, stores the tag at +6, installs the vtable, zeroes the
/// 32 eight-byte weight slots at +0x20 and the tail words at +0x120/+0x124.
/// Leaf thiscall/1 constructor; returns this.
export!(thiscall, rw_00873650(this_: u32, arg: u32) -> u32 {
    unsafe {
        let base = this_ as *mut u8;
        *(base.add(4) as *mut u16) = 0;
        *(base.add(6) as *mut u16) = arg as u16;
        for off in [8usize, 0xc, 0x10, 0x14, 0x18, 0x1c] {
            *(base.add(off) as *mut u32) = 0;
        }
        *(base as *mut u32) = relocated(0x00FE8024);
        for i in 0..0x20u32 {
            *(base.add(0x20 + (i * 8) as usize) as *mut u64) = 0;
        }
        *(base.add(0x120) as *mut u32) = 0;
        *(base.add(0x124) as *mut u32) = 0;
        this_
    }
});
