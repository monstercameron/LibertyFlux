// original: 0x0069a3a0 init_vtable_and_extend_slot
/// Install the channel vtable, then extend the sample slot through the allocator.
///
/// Does nothing for a null `obj`. Otherwise stores the relocated vtable
/// address at `obj+0`; when the old `obj+8` slot value is nonzero it is passed
/// (with `aux` as `this`) to the allocator callee and the answer is added back
/// into the slot. The return channel is unchecked: the null-object path leaves
/// entry EAX untouched, which a rewrite cannot reproduce.
export!(cdecl, rw_0069a3a0(obj: u32, aux: u32) -> u32 {
    unsafe {
        if obj == 0 {
            return 0;
        }
        *(obj as *mut u32) = relocated(0xFE3D44);
        let slot = (obj + 8) as *mut u32;
        let old = *slot;
        if old == 0 {
            return 0;
        }
        let grown = callee_thiscall!(1, u32, aux, old);
        *slot = old.wrapping_add(grown);
        grown
    }
});
