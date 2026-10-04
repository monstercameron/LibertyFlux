// original: 0x008a29d0 rage::audSimpleSound::vf5
/// Deleting destructor of `rage::audSimpleSound` (vf5).
///
/// Stamps the vtable, releases the slot voice unless suppressed by flag
/// 0x40 (thiscall/1, stubbed), runs the base destructor (thiscall/0,
/// stubbed), then frees through the sound pool when the flag bit is set.
/// Returns the object pointer.
export!(thiscall, rw_008a29d0(this: u32, flags: u32) -> u32 {
    unsafe {
        let obj = this as *mut u8;
        let f39 = *obj.add(0x39);
        *(obj as *mut u32) = relocated(0xe7a9fc);
        if f39 & 0x40 == 0 {
            let slot = *obj.add(0x48);
            if slot != SLOT_EMPTY {
                let bank = *obj.add(0x40);
                if voice_ptr(bank, slot) != 0 {
                    callee_thiscall!(1, u32, this, 0);
                }
            }
        }
        callee_thiscall!(2, u32, this);
        if flags & 1 != 0 {
            let bank = *obj.add(0x40);
            callee_thiscall!(3, u32, relocated(SOUND_POOL), this, bank as u32);
        }
        this
    }
});
