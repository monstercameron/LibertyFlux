// original: 0x00d8e3f0 audio_entity_bind_params
/// Bind the entity's parameter slots to `arg` and attach a child object.
///
/// Resolves the slot block at `this + 0x70`, binds the slots at fixed
/// offsets (three of them only when they are already nonzero) through the
/// shared binder with the caller's argument as its object, then allocates
/// an 80-byte child, constructs it in place and stores it at `this + 0x90`
/// (or stores null when allocation fails). Returns `this`.
lf_rs89_rt::export!(thiscall, rw_00d8e3f0(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        lf_rs89_rt::callee_cdecl!(1, u32, this.wrapping_add(0x70) as u32);
        if *(this.wrapping_add(0x58) as *const u32) != 0 {
            lf_rs89_rt::callee_thiscall!(2, u32, arg, this.wrapping_add(0x58) as u32);
        }
        if *(this.wrapping_add(0x5C) as *const u32) != 0 {
            lf_rs89_rt::callee_thiscall!(2, u32, arg, this.wrapping_add(0x5C) as u32);
        }
        lf_rs89_rt::callee_thiscall!(2, u32, arg, this.wrapping_add(0x60) as u32);
        lf_rs89_rt::callee_thiscall!(2, u32, arg, this.wrapping_add(0x64) as u32);
        lf_rs89_rt::callee_thiscall!(2, u32, arg, this.wrapping_add(0x6C) as u32);
        if *(this.wrapping_add(0x74) as *const u32) != 0 {
            lf_rs89_rt::callee_thiscall!(2, u32, arg, this.wrapping_add(0x74) as u32);
        }
        let block: u32 = lf_rs89_rt::callee_cdecl!(3, u32, 0x50);
        let child_at = this.wrapping_add(0x90) as *mut u32;
        if block == 0 {
            *child_at = 0;
        } else {
            *child_at = lf_rs89_rt::callee_thiscall!(4, u32, block);
        }
        this as u32
    }
});
