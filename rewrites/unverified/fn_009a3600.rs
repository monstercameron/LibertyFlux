// original: 0x009a3600 aud_entity_radio_emitter_ctor
/// Constructor of the entity radio emitter.
///
/// Runs the base constructor through callee 1, stamps the two table
/// addresses, clears the word at offset 0xc, and constructs the two embedded
/// members at offsets 0x30 and 0x4c through callee 2. Returns the object.
export!(thiscall, rw_009a3600(this: u32) -> u32 {
    unsafe {
        const VTABLE_MAIN: u32 = 0xe91480;
        const VTABLE_SUB: u32 = 0xe908ac;
        callee_thiscall!(1, u32, this);
        *(this as *mut u32) = relocated(VTABLE_MAIN);
        *((this.wrapping_add(8)) as *mut u32) = relocated(VTABLE_SUB);
        *((this.wrapping_add(0xc)) as *mut u32) = 0;
        callee_thiscall!(2, u32, this.wrapping_add(0x30));
        callee_thiscall!(2, u32, this.wrapping_add(0x4c));
        this
    }
});
