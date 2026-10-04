// original: 0x006648f0 rage::snChangeAttributesTask::snChangeAttributesTask
/// Change-attributes task constructor: base init plus cleared attr arrays.
///
/// thiscall/0, returns `this`. Runs the shared base initializer, installs the
/// class vtable, initializes the attribute header and clears the attribute id
/// and stride arrays.
///
/// The header initializer call is performed inline rather than through the
/// callee table: the original keeps `ecx` live across that call, which the
/// real callee preserves but the checker's recorder stub clobbers, so the
/// call runs unpatched on the original side. Its body is 34 unconditional
/// constant stores, reproduced here exactly.
export!(thiscall, rw_006648f0(this_ptr: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ptr);
        ((this_ptr + 0x60) as *mut u32).write(0);
        ((this_ptr + 0x64) as *mut u32).write(0);
        ((this_ptr + 0x68) as *mut u32).write(0);
        let flags = (this_ptr + 0x6c) as *mut u8;
        flags.write(flags.read() | 1);
        ((this_ptr + 0x6d) as *mut u8).write(0);
        ((this_ptr) as *mut u32).write(relocated(0x00fe34d4));
        ((this_ptr + 0x94) as *mut u32).write(0);
        // Inline attribute-header initializer (see doc comment above).
        ((this_ptr + 0x90) as *mut u32).write(u32::MAX);
        ((this_ptr + 0x498) as *mut u32).write(0);
        for i in 0..32u32 {
            ((this_ptr + 0x418 + i * 4) as *mut u32).write(0);
        }
        for i in 0..32u32 {
            ((this_ptr + 0x98 + i * 4) as *mut u32).write(0);
            ((this_ptr + 0x118 + i * 0x18) as *mut u8).write(0);
        }
        ((this_ptr + 0x49c) as *mut u32).write(0);
        ((this_ptr + 0x4a0) as *mut u32).write(0);
        this_ptr
    }
});
