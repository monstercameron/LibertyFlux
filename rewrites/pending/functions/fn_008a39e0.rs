// original: 0x008a39e0 aud_header_byte_copy
/// Copy the byte after a length-prefixed header and publish its address.
///
/// Original 0x008A39E0 (`cdecl(obj, out_byte, out_ptr)`): reads the count
/// byte at `obj+5`, copies the byte at `obj+count+6` to `*out_byte`, stores
/// `obj+count+7` to `*out_ptr`, and returns `out_ptr`.
export!(cdecl, rw_008a39e0(obj: u32, out_byte: u32, out_ptr: u32) -> u32 {    let count = unsafe { ((obj + 5) as *const u8).read_unaligned() } as u32;
    let value = unsafe { ((obj + count + 6) as *const u8).read_unaligned() };
    unsafe { (out_byte as *mut u8).write_unaligned(value) };
    unsafe { (out_ptr as *mut u32).write_unaligned(obj + count + 7) };
    out_ptr
});
