// original: 0x008b26d0 rage::audCompressorEffectPc::audCompressorEffectPc
/// Construct an `audCompressorEffectPc`: install its vtable pointer, store the
/// default parameter block, set the enabled flag, and zero the three
/// per-channel state blocks. Returns `this`, like the original.
export!(thiscall, rw_008b26d0(this: *mut u8) -> u32 {
    #[inline(always)]
    unsafe fn wf32(base: *mut u8, off: usize, v: f32) {
        *(base.add(off) as *mut f32) = v;
    }
    #[inline(always)]
    unsafe fn w32(base: *mut u8, off: usize, v: u32) {
        *(base.add(off) as *mut u32) = v;
    }
    unsafe {
        // Class vtable (file VA of the vtable; relocated at load).
        *(this as *mut u32) = relocated(0x00E7CF5C);
        // Default parameter block at +0x270.
        w32(this, 0x270, 0);
        wf32(this, 0x274, f32::from_bits(0xC0C0_0000)); // -6.0
        wf32(this, 0x278, f32::from_bits(0x4000_0000)); // 2.0
        wf32(this, 0x27C, f32::from_bits(0x3BA3_D70A));
        wf32(this, 0x280, f32::from_bits(0x3DF5_C28F));
        wf32(this, 0x284, f32::from_bits(0x3D75_C28F));
        wf32(this, 0x288, f32::from_bits(0x3E4C_CCCD)); // 0.2
        wf32(this, 0x28C, f32::from_bits(0x3C23_D70A));
        *this.add(0x290) = 1;
        // Three contiguous per-channel state blocks, +0x208..+0x250.
        core::ptr::write_bytes(this.add(0x208), 0, 0x250 - 0x208);
        this as u32
    }
});
