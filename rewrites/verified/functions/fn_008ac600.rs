// original: 0x008ac600 rage::audCompressorEffect::vf5
/// Copies the 36-byte slot `index` over slot `(index+1) % 3`, stores the new
/// index, and unless the listener at `this+8` is null, tail-calls its
/// vtable slot +0x14. Returns the last copied word, or the call answer.
export!(thiscall, rw_008ac600(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        let idx = *(this.add(0x30) as *const u32);
        let slot = idx.wrapping_add(1) % 3;
        let src = this.add(idx.wrapping_mul(36) as usize);
        let dst = this.add(slot.wrapping_mul(36) as usize);
        let last = *(src.add(0x98) as *const u32);
        core::ptr::copy_nonoverlapping(src.add(0x78), dst.add(0x78), 36);
        let sub = *(this.add(8) as *const u32);
        *(this.add(0x30) as *mut u32) = slot;
        if sub == 0 {
            last
        } else {
            let vt = *(sub as *const u32);
            let tgt = *((vt as *const u32).add(0x14 / 4));
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(sub)
        }
    }
});

