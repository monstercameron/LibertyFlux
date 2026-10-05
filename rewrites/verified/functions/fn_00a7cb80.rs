// original: 0x00a7cb80 attach_cam_to_chain
/// List splice-or-forward: follow the live link, or splice in the new one.
///
/// Takes the object pointer in ECX and a link pointer on the stack. When
/// the object's stored link is non-null, control transfers to the shared
/// successor with that link, returning whatever that call answers.
/// Otherwise the new link is spliced in with back-links both ways and the
/// link itself is the result.
export!(thiscall, rw_00a7cb80(this: u32, link: u32) -> u32 {
    unsafe {
        let next = *(this.wrapping_add(0x124) as *const u32);
        if next != 0 {
            return callee_thiscall!(1, u32, next);
        }
        *(link.wrapping_add(0x118) as *mut u32) = this;
        *(this.wrapping_add(0x124) as *mut u32) = link;
        link
    }
});
