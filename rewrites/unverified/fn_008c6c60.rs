// original: 0x008C6C60 table_sort_publish
/// Hand the table at `this` to the sorting callee with its comparator.
///
/// Passes the base pointer at `this`, the count at `this + 4`, the element
/// size 8 and the address of the record comparator as the sort key, exactly
/// as the original pushes them. Returns the callee's answer. The comparator
/// address is data the function forwards, not a call this rewrite makes.
/// Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_008c6c60(this: u32) -> u32 {
    unsafe {
        const SORT_CALLEE: u32 = 1;
        const ELEMENT_SIZE: u32 = 8;
        const COMPARATOR: u32 = 0x008C6D70;
        let base = (this as *const u32).read_unaligned();
        let count = ((this + 4) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(
            SORT_CALLEE, u32, base, count, ELEMENT_SIZE, COMPARATOR)
    }
});
