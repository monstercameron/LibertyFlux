// original: 0x00bdf700 audio_dtor_manager_handle
/// Destroy the audio node with a manager handle flag at +0x82 and a slot at +0x30.
/// Stamps vtable 0xEB8E8C. When the flag byte is set, resolves a ticket
/// (id 1, cdecl/0), routes it through the manager found at the shared
/// registry word (id 2, thiscall/1), releases the returned handle
/// (id 3, cdecl/1) and clears the flag. When the +0x30 word is non-null
/// its slot is released through the shared release helper (id 4) and
/// cleared. Forwards to the shared base destructor (tail id 9).
/// Returns the tail answer.
export!(thiscall, rw_00bdf700(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB8E8C;
        const REGISTRY: u32 = 0x16DD63C;
        *(this as *mut u32) = relocated(VTABLE);
        if *(this.add(0x82)) != 0 {
            let ticket = callee_cdecl!(1, u32,);
            let manager = *(global::<u32>(REGISTRY) as *const u32);
            let handle = callee_thiscall!(2, u32, manager, ticket);
            callee_cdecl!(3, u32, handle);
            *(this.add(0x82)) = 0;
        }
        if *((this.add(0x30)) as *const u32) != 0 {
            callee_stdcall!(4, u32, this.add(0x30) as u32);
            *((this.add(0x30)) as *mut u32) = 0;
        }
        callee_thiscall!(9, u32, this as u32)
    }
});
