// original: 0x00dfe51b tagged_block_free
// rs03f15: guarded release of a tagged block (cdecl/1).
//
// A null pointer is a no-op returning zero. Otherwise the word eight bytes
// below the pointer must hold the tag 0xDDDD; anything else is returned as
// the block base without calling out, while a matching tag releases the
// block through the heap helper (cdecl/1) and returns its answer.
export!(cdecl, rw_rs03f15(p: u32) -> u32 {
    unsafe {
        if p == 0 {
            return 0;
        }
        let base = p.wrapping_sub(8);
        if *(base as *const u32) != 0xDDDD {
            return base;
        }
        callee_cdecl!(1, u32, base)
    }
});
