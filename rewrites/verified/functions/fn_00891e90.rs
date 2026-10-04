// original: 0x00891e90 rage::audSound::vf4
/// Virtual slot 4 helper: forwards the argument through the looked-up object.
///
/// A null argument returns 0. When the link byte at 5 reads 0xff the global
/// fallback object handles the argument instead. Otherwise the target is
/// resolved from the link byte and the table index byte, and its virtual
/// slot 4 is invoked with the argument. Returns the callee's answer.
export!(thiscall, rw_00891e90(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        if arg == 0 {
            return 0;
        }
        let b = *(this.add(5));
        if b == 0xff {
            return callee_thiscall!(1, u32, relocated(0x115db1c), arg);
        }
        let stride = *global::<u32>(0x115d964);
        let table = *global::<u32>(0x115d988);
        let idx = *(this.add(0x40)) as u32;
        let entry = *((table
            .wrapping_add(idx.wrapping_mul(0x6f40))
            .wrapping_add(0x6f10)) as *const u32);
        let target = stride.wrapping_mul(b as u32).wrapping_add(entry);
        let vtable = *(target as *const u32);
        let slot = *((vtable.wrapping_add(0x10)) as *const u32);
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        f(target, arg)
    }
});
