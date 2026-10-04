// original: 0x008a0a50 crossfade_slot_teardown
/// Slot teardown for a crossfade sound with pool release and tail call.
///
/// Stamps the vtable and releases the 8 slot voices exactly like
/// [`rw_008a17b0`], then releases the crossfade voice selected by
/// [this+0xb0] through the sound pool and marks it empty, and ends by
/// tail-calling the base destructor (stubbed); its answer is the return.
export!(thiscall, rw_008a0a50(this: u32) -> u32 {
    unsafe {
        let o = this as *mut u8;
        *(o as *mut u32) = relocated(0xe7a730);
        let bank = *o.add(0x40);
        let mut i = 0usize;
        while i < 8 {
            let slot = *o.add(0x48 + i);
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
            *o.add(0x48 + i - 1) = SLOT_EMPTY;
        }
        let sel = *o.add(0xb0);
        if sel != SLOT_EMPTY {
            let stride = *global::<u32>(VOICE_STRIDE_GV);
            let table = *global::<u32>(VOICE_TABLE_GV);
            let base = *((table.wrapping_add((bank as u32).wrapping_mul(BANK_SCALE)).wrapping_add(TABLE_HDR)) as *const u32);
            let v = base.wrapping_add(stride.wrapping_mul(sel as u32));
            callee_thiscall!(2, u32, relocated(SOUND_POOL), v, bank as u32);
            *o.add(0xb0) = SLOT_EMPTY;
        }
        callee_thiscall!(3, u32, this)
    }
});
