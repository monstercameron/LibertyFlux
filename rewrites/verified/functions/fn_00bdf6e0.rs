// original: 0x00bdf6e0 audio_dtor_chain_sub14
/// Destroy the audio node with a chained sub-object at +0x14.
/// Stamps vtable 0xEB90FC, runs the sub-step (id 1, thiscall/0) and the
/// sub-object teardown (id 2, thiscall/0 on object +0x14). Forwards to the
/// shared base destructor (tail id 9). Returns the tail answer.
export!(thiscall, rw_00bdf6e0(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB90FC;
        *(this as *mut u32) = relocated(VTABLE);
        callee_thiscall!(1, u32, this as u32);
        callee_thiscall!(2, u32, this.add(0x14) as u32);
        callee_thiscall!(9, u32, this as u32)
    }
});
