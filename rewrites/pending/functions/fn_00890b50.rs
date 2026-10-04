// original: 0x00890b50 audio_copy_params
/// Copy a parameter block into this object, merging one flag bit.
///
/// Copies five dwords from fixed source offsets to fixed object offsets,
/// merges bit 4 of the source byte at +0x19 into the object's flag byte at
/// +0x39 (only that bit can change), then invokes the counter helper on the
/// source handle at +8 when non-null and stores the handle at +0x74.
/// Returns the handle.
export!(thiscall, rw_00890b50(this: u32, src: u32) -> u32 {
    unsafe {
        ((this + 0x78) as *mut u32).write((src as *const u32).read());
        ((this + 0x90) as *mut u32).write(((src + 4) as *const u32).read());
        ((this + 0x60) as *mut u32).write(((src + 0x0c) as *const u32).read());
        ((this + 0x6c) as *mut u32).write(((src + 0x10) as *const u32).read());
        let incoming = ((src + 0x19) as *const u8).read() << 4;
        let flagp = (this + 0x39) as *mut u8;
        flagp.write(flagp.read() ^ ((incoming ^ flagp.read()) & 0x10));
        ((this + 0x44) as *mut u32).write(((src + 0x14) as *const u32).read());
        let handle = ((src + 8) as *const u32).read();
        if handle != 0 {
            let _: u32 = callee_thiscall!(1, u32, handle);
        }
        ((this + 0x74) as *mut u32).write(handle);
        handle
    }
});
