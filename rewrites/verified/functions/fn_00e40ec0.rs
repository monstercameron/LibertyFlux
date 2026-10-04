// original: 0x00e40ec0 row_handler_call_mark
// row tag call plus marker store.
// If base is null, does nothing. Otherwise invokes the row handler with
// (elem, tag, aux) where elem = base+index*0x2b0, then stores mark at
// elem+0x14. Returns nothing meaningful.
export!(stdcall, rw_00e40ec0(base: u32, tag: u32, index: u32, aux: u32, mark: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x2b0;
        const MARK_OFF: u32 = 0x14;
        if base == 0 {
            return 0;
        }
        let elem = base.wrapping_add(index.wrapping_mul(STRIDE));
        callee_thiscall!(1, u32, elem, tag, aux);
        *((elem.wrapping_add(MARK_OFF)) as *mut u32) = mark;
        0
    }
});
