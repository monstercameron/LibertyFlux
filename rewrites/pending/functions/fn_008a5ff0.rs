// original: 0x008a5ff0 rage::audMathOperationSound::audMathOperationSound_2
/// Constructor: set the vtable, detach two table entries, tail into the base.
///
/// Installs this class's vtable, then unless bit 6 of the flag byte at +0x39
/// is set, resolves the entity-table slot for the index/count bytes (+0x40,
/// +0x48) and notifies through a helper when the resolved total is non-zero.
/// When the tag byte at +0xb5 is not 0xff, resolves the same table with the
/// tag as count, forwards (total, index) to the table owner, and resets the
/// tag to 0xff. Tail-calls the base-class constructor and returns its result.
/// Note: the tail call returns straight from the stub, so the rewrite frame
/// is still live at return; the contract therefore skips the ESP check.
export!(thiscall, rw_008a5ff0(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xe7b974);
        if ((*this.add(0x39)) & 0x40) == 0 {
            let count = *this.add(0x48);
            if count != 0xff {
                let index = *this.add(0x40) as u32;
                let stride = *global::<u32>(0x115d964);
                let table = *global::<u32>(0x115d988);
                let slot = table
                    .wrapping_add(index.wrapping_mul(0x6f40))
                    .wrapping_add(0x6f10) as *const u32;
                let total = (*slot).wrapping_add(stride.wrapping_mul(count as u32));
                if total != 0 {
                    let _: u32 = callee_thiscall!(1, u32, this as u32, 0);
                }
            }
        }
        let tag = *this.add(0xb5);
        if tag != 0xff {
            let index = *this.add(0x40) as u32;
            let stride = *global::<u32>(0x115d964);
            let table = *global::<u32>(0x115d988);
            let slot = table
                .wrapping_add(index.wrapping_mul(0x6f40))
                .wrapping_add(0x6f10) as *const u32;
            let total = (*slot).wrapping_add(stride.wrapping_mul(tag as u32));
            let _: u32 = callee_thiscall!(2, u32, relocated(0x115d8a0), total, index);
            *this.add(0xb5) = 0xff;
        }
        let r: u32 = callee_thiscall!(3, u32, this as u32);
        core::ptr::read_volatile(&r)
    }
});
