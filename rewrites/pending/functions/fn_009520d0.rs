// original: 0x009520d0 object_kind_dispatcher
/// Forwards a call to one of four handlers selected by the object's kind tag.
///
/// The byte at +0x18 of the object picks the handler; the object becomes the
/// callee's `this` while the remaining arguments keep their order. Kinds 0
/// and 2 share the default handler; any other unlisted kind returns without
/// calling (the original's upper bytes there are entry-eax residue, so only
/// the low byte, the kind itself, is meaningful). Returns the handler's
/// answer, or the kind when no handler runs.
export!(cdecl, rw_009520d0(first: u32, this_obj: u32, third: u32, fbits: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 0x18;
        let kind = *((this_obj.wrapping_add(KIND_OFF)) as *const u8);
        match kind {
            4 => callee_thiscall!(1, u32, this_obj, first, third, fbits),
            3 => callee_thiscall!(2, u32, this_obj, first, third, fbits),
            1 | 6 => callee_thiscall!(3, u32, this_obj, first, third, fbits),
            0 | 2 => callee_thiscall!(4, u32, this_obj, first, third, fbits),
            _ => kind as u32,
        }
    }
});
