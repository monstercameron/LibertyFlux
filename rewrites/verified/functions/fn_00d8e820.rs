// original: 0x00d8e820 audio_struct_zero22
/// Zero the 22-byte audio parameter block at `this` and return `this`.
///
/// The original clears bytes 0..22 with overlapping dword and word stores;
/// every byte in that range ends up zero however the stores are grouped.
lf_rs89_rt::export!(thiscall, rw_00d8e820(this: *mut u8) -> u32 {
    unsafe {
        core::ptr::write_bytes(this, 0, 22);
        this as u32
    }
});
