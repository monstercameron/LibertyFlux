// original: 0x00bf5550 build_emitter_cached
/// Build the emitter, refreshing the cached entry first when empty, then push
/// one float property. A null object returns entry EAX: channel none.
export!(thiscall, rw_bf5550(this: *const u8, obj: u32) -> u32 {
    unsafe {
        if obj == 0 {
            return 0;
        }
        let w8 = *((this.add(8)) as *const u32);
        let edi: u32 = callee_thiscall!(1, u32, relocated(0x1394D60), w8, 0, 0);
        if edi == 0 {
            return 0;
        }
        let mut buf = [0u32; 4];
        let _: u32 = callee_thiscall!(2, u32, this as u32, buf.as_mut_ptr() as u32);
        if *((obj as *const u8).add(0x20) as *const u32) == 0 {
            let _: u32 = callee_thiscall!(3, u32, obj);
            let w = *((obj as *const u8).add(0x20) as *const u32);
            let _: u32 = callee_thiscall!(4, u32, (obj as u32).wrapping_add(0x10), w);
        }
        let w = *((obj as *const u8).add(0x20) as *const u32);
        let _: u32 = callee_thiscall!(5, u32, edi, w);
        let _: u32 = callee_thiscall!(6, u32, edi, buf.as_mut_ptr() as u32);
        let _: u32 = callee_thiscall!(7, u32, relocated(0x1394D60), edi, obj, 0);
        let f1c = *((this.add(0x1C)) as *const u32);
        let _: u32 = callee_thiscall!(8, u32, edi, relocated(0xEBBEB0), f1c);
        callee_thiscall!(9, u32, edi)
    }
});
