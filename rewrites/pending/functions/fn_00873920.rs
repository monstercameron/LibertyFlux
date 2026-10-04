// original: 0x00873920 weight_slot_drop
use lf_checker_rt::{export};

/// Drop weight slot `index`: shift later slots down one place, then clear it.
///
/// Slots are eight bytes at +0x20; indices below 31 shift slots
/// `index..31` up by one element. Out-of-range indices skip the shift but
/// still clear the addressed slot (faulting on wild indices, like the
/// original). Leaf thiscall/1; EAX is left over, so no return channel.
export!(thiscall, rw_00873920(this_: u32, index: u32) -> () {
    unsafe {
        if index < 0x1f {
            let mut count = 0x1fu32.wrapping_sub(index);
            let mut slot = this_.wrapping_add(0x118);
            while count != 0 {
                *(slot as *mut u32) = *(slot.wrapping_sub(8) as *const u32);
                *(slot.wrapping_add(4) as *mut u32) =
                    *(slot.wrapping_sub(4) as *const u32);
                slot = slot.wrapping_sub(8);
                count -= 1;
            }
        }
        let base = this_
            .wrapping_add(index.wrapping_mul(8))
            .wrapping_add(0x20);
        *(base as *mut u32) = 0;
        *(base.wrapping_add(4) as *mut u32) = 0;
    }
});
