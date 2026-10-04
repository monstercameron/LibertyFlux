// original: 0x00bdf660 audio_dtor_chain_sub
/// Destroy the audio node with a chained sub-object at +0x20.
/// Stamps vtable 0xEB9154 and the +0x14 sub-tag 0xEB91AC, runs the
/// sub-step (id 1, thiscall/0) and the sub-object teardown (id 2,
/// thiscall/0 on object +0x20). Forwards to the shared base
/// destructor (tail id 9). Returns the tail answer.
export!(thiscall, rw_00bdf660(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB9154;
        const SUBTAG: u32 = 0xEB91AC;
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x14)) as *mut u32) = relocated(SUBTAG);
        callee_thiscall!(1, u32, this as u32);
        callee_thiscall!(2, u32, this.add(0x20) as u32);
        callee_thiscall!(9, u32, this as u32)
    }
});
