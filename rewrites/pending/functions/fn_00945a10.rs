// original: 0x00945a10 advance_slot_pair_and_mix_key
/// Touch the current and next slots of a dual-slot object, then mix a key.
///
/// The object holds a current-slot index byte; each slot is `0xbd0` bytes and
/// carries a live flag. Both the indexed slot and its successor (mod 2) are
/// handed to the slot worker when live. Afterwards two state words are reset
/// and seven key bytes from the argument block are folded into the object:
/// the low three bytes of one word are xored, the rest stored or masked.
/// Returns the last key byte (what the original leaves in `eax`).
lf_checker_rt::export!(thiscall, rw_00945a10(this_ptr: u32, arg: u32) -> u32 {
    unsafe {
        let idx = *(this_ptr.wrapping_add(0x1917) as *const u8) as u32;
        let slot = this_ptr.wrapping_add(idx.wrapping_mul(0xbd0));
        if *(slot.wrapping_add(0xbca) as *const u8) != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, slot);
        }
        let next = (idx.wrapping_add(1)) % 2;
        let slot2 = this_ptr.wrapping_add(next.wrapping_mul(0xbd0));
        if *(slot2.wrapping_add(0xbca) as *const u8) != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, slot2);
        }
        *(this_ptr.wrapping_add(0x17c8) as *mut u32) = 0;
        *(this_ptr.wrapping_add(0x17cc) as *mut u32) = 2;
        let cur = *(this_ptr.wrapping_add(0x17a8) as *const u32);
        let blk = *(arg as *const u32);
        *(this_ptr.wrapping_add(0x17a8) as *mut u32) = cur ^ ((cur ^ blk) & 0x00ff_ffff);
        let b5 = *(arg.wrapping_add(5) as *const u8);
        *(this_ptr.wrapping_add(0x17ad) as *mut u8) = b5;
        let b4 = *(arg.wrapping_add(4) as *const u8);
        let w = this_ptr.wrapping_add(0x17b0) as *mut u32;
        *w = (*w) & 0xff00_0000;
        *(this_ptr.wrapping_add(0x17ac) as *mut u8) = b4;
        let b6 = *(arg.wrapping_add(6) as *const u8);
        *(this_ptr.wrapping_add(0x17b4) as *mut u8) = b6;
        let b7 = *(arg.wrapping_add(7) as *const u8);
        *(this_ptr.wrapping_add(0x17b5) as *mut u8) = b7;
        *(this_ptr.wrapping_add(0x17bc) as *mut u8) = 0xff;
        *(this_ptr.wrapping_add(0x17c4) as *mut u8) = 0xff;
        *(this_ptr.wrapping_add(0x1923) as *mut u8) = 1;
        b7 as u32
    }
});
