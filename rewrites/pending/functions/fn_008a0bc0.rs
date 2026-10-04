// original: 0x008a0bc0 crossfade_voice_setup
/// Crossfade voice setup with a two-slot update sweep.
///
/// Programs the crossfade voice selected by the raw byte [this+0xb0] from
/// the classifier answer (thiscall/0, stubbed), runs two configuration
/// passes (thiscall/1 each, stubbed), then sweeps two slots through the
/// update pair (thiscall/2 then thiscall/1, stubbed), returning the last
/// answer. The per-slot null arm is dead in practice (a null voice here
/// means the setup voice above already faulted) and is kept for shape.
export!(thiscall, rw_008a0bc0(this: u32, a1: u32) -> u32 {
    unsafe {
        let o = this as *const u8;
        let bank = *o.add(0x40);
        let stride = *global::<u32>(VOICE_STRIDE_GV);
        let table = *global::<u32>(VOICE_TABLE_GV);
        let base = *((table.wrapping_add((bank as u32).wrapping_mul(BANK_SCALE)).wrapping_add(TABLE_HDR)) as *const u32);
        let sel = *o.add(0xb0);
        let voice0 = base.wrapping_add(stride.wrapping_mul(sel as u32));
        let r0: u32 = callee_thiscall!(1, u32, this);
        core::ptr::write_unaligned(((voice0.wrapping_add(0x70))) as *mut u16, (r0 as u8) as u16);
        let saved = *(o.add(0x54) as *const u32);
        callee_thiscall!(2, u32, this, voice0);
        let mut ans: u32 = callee_thiscall!(3, u32, this, voice0);
        let mut k = 0u32;
        while k < 2 {
            let slot = *o.add(0x48 + (k as usize));
            k += 1;
            if slot == SLOT_EMPTY {
                continue;
            }
            let v = voice_ptr(bank, slot);
            if v == 0 {
                continue;
            }
            let _r4: u32 = callee_thiscall!(4, u32, v, saved, 0);
            let v2 = voice_ptr(bank, slot);
            ans = callee_thiscall!(5, u32, v2, a1);
        }
        ans
    }
});
