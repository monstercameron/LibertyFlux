// original: 0x00b64570 release_slot_and_indexed_entry
// thiscall/0. Destroys the sub-object in the slot at +0x1c when occupied,
// then, when the teardown flag at +0x108 is set, releases the indexed entry
// from the global table and clears the index/flag. Returns nothing.
export!(thiscall, rw_rs11f0(this: *mut u8) -> () {
    unsafe {
        let slot = this.add(0x1c) as *mut u32;
        if *slot != 0 {
            let destroy: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(callee_addr(1) as usize);
            destroy(*slot, slot as u32);
            callee_cdecl!(2, u32, *slot);
            *slot = 0;
        }
        if *this.add(0x108) != 0 {
            let index = *(this.add(0x104) as *const u32);
            let target = *global::<u32>(0x1295cd8 + index.wrapping_mul(4));
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(callee_addr(3) as usize);
            release(target);
            *(this.add(0x104) as *mut u32) = 0xffffffff;
            *this.add(0x108) = 0;
        }
    }
});
