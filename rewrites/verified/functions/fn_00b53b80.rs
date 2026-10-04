// original: 0x00b53b80 desc_alloc_init
/// Copies a u16 pair from the descriptor argument into the first two
/// words of `this`, allocates a fixed-size child block and attaches the
/// source to it with flags (1, 0), or records null when the allocation
/// fails. Returns `this`.
export!(thiscall, rw_00b53b80(this: *mut u8, src: *const u8) -> u32 {
    unsafe {
        *(this as *mut u32) = *(src.add(0x18) as *const u16) as u32;
        *(this.add(4) as *mut u32) = *(src.add(0x16) as *const u16) as u32;
        let child: u32 = callee_cdecl!(1, u32, 0x414u32);
        if child == 0 {
            *(this.add(8) as *mut u32) = 0;
        } else {
            let attached: u32 = callee_thiscall!(2, u32, child, src as u32, 1u32, 0u32);
            *(this.add(8) as *mut u32) = attached;
        }
        this as u32
    }
});
