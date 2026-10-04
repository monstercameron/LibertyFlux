// original: 0x008a3010 multitrack_slot_teardown
/// Count-bounded slot teardown for a multitrack sound with tail call.
///
/// Stamps the vtable, then releases the first `count` slot voices exactly
/// like [`rw_008a17b0`], and ends by tail-calling the base destructor
/// (stubbed); its answer is the return.
export!(thiscall, rw_008a3010(this: u32) -> u32 {
    unsafe {
        let o = this as *mut u8;
        *(o as *mut u32) = relocated(0xe7ab64);
        let count = *(o.add(0xb0) as *const u32);
        let bank = *o.add(0x40);
        let mut i = 0u32;
        while i < count {
            let slot = *o.add(0x48 + (i as usize));
            i += 1;
            if slot == SLOT_EMPTY {
                continue;
            }
            let v = voice_ptr(bank, slot);
            if v == 0 {
                continue;
            }
            let vt = core::ptr::read_unaligned((v as *const u32));
            let tgt = core::ptr::read_unaligned(((vt.wrapping_add(0x14))) as *const u32);
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            release(v, 1);
            *o.add(0x48 + (i as usize) - 1) = SLOT_EMPTY;
        }
        callee_thiscall!(2, u32, this)
    }
});
