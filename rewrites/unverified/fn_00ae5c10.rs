// original: 0x00AE5C10 streaming_link_chain_valid

/// Return the low-byte validity result for a linked three-object chain.
///
/// The cdecl argument is a pointer to a 32-bit object record. Its link at
/// `+0x4c` points to the first related record, whose link at `+0x4c` may point
/// to a second related record. A linked record qualifies when its word at
/// `+0x24` has bit 15 set and either its word at `+0x28` has bit 19 set or
/// the word at `+0x5c` has bit 13 set. If both linked records qualify the
/// function returns one. Otherwise, the base record returns one when bit 15
/// at `+0x24` is clear, or when bit 19 at `+0x28` is clear and bit 13 at
/// `+0x5c` is set. The original defines only AL; the checker compares that
/// byte as the Boolean result.
export!(cdecl, rw_00ae5c10(object: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_u32(base: u32, offset: u32) -> u32 {
            core::ptr::read_unaligned(base.wrapping_add(offset) as *const u32)
        }

        #[inline(always)]
        unsafe fn read_u16(base: u32, offset: u32) -> u16 {
            core::ptr::read_unaligned(base.wrapping_add(offset) as *const u16)
        }

        #[inline(always)]
        unsafe fn linked_object_qualifies(node: u32) -> bool {
            if read_u32(node, 0x24) & 0x8000 == 0 {
                return false;
            }
            read_u32(node, 0x28) & (1 << 19) != 0 || read_u16(node, 0x5c) & 0x2000 != 0
        }

        let first = read_u32(object, 0x4c);
        if first == 0 || !linked_object_qualifies(first) {
            return 0;
        }
        let second = read_u32(first, 0x4c);
        if second != 0 && linked_object_qualifies(second) {
            return 1;
        }

        if read_u32(object, 0x24) & 0x8000 == 0 {
            return 1;
        }
        if read_u32(object, 0x28) & (1 << 19) != 0 {
            return 0;
        }
        u32::from(read_u16(object, 0x5c) & 0x2000 != 0)
    }
});
