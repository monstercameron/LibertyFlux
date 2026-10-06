// original: 0x00D78990 clear_field_0ee0 (proposed)

/// Clear the word at `obj + 0xee0` and return `obj`.
///
/// Cdecl, one stack word; `eax` still holds the argument at return.
use lf_checker_rt::export;

export!(cdecl, rw_00d78990(obj: u32) -> u32 {
    unsafe {
        const FIELD_OFF: u32 = 0xee0;
        ((obj + FIELD_OFF) as *mut u32).write_unaligned(0);
    }
    obj
});
