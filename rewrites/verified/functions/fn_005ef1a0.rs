// original: 0x005ef1a0 cb_tex_ctor

/// Construct a texture-blit callback holder in place.
///
/// `this` receives the holder's vtable pointer at `+0x00`, a masked link word at
/// `+0x04` (the old word xored with the low 14 bits of itself xored with the
/// global callback counter, which is then incremented), the texture pointer
/// `tex` at `+0x08`, the dereferenced colour-source word `*colour_src` at
/// `+0x0C`, the two 16-byte rects `*src_rect`/`*dst_rect` at `+0x10`/`+0x20`
/// (default rect words are written first, then overwritten), and the
/// dereferenced colour word `*colour` at `+0x30`. Returns `this`.
///
/// Original: thiscall, five stack arguments, no calls, callee pops 0x14.
lf_checker_rt::export!(thiscall, rw_005ef1a0(this: u32, tex: u32, colour_src: u32, src_rect: u32, dst_rect: u32, colour: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00FE1550;
        const LINK: u32 = 0x04;
        const TEX: u32 = 0x08;
        const CSRC: u32 = 0x0C;
        const SRC: u32 = 0x10;
        const DST: u32 = 0x20;
        const COLOUR: u32 = 0x30;
        const COUNTER: u32 = 0x010327A0;
        const LINK_MASK: u32 = 0x3FFF;
        const RX0: u32 = 0x49742400;
        const RX1: u32 = 0xC9742400;
        let counter = (lf_checker_rt::global::<u32>(COUNTER)).read_unaligned();
        let link = ((this + LINK) as *const u32).read_unaligned();
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + LINK) as *mut u32).write_unaligned(link ^ ((link ^ counter) & LINK_MASK));
        ((this + TEX) as *mut u32).write_unaligned(tex);
        ((this + SRC) as *mut u32).write_unaligned(RX0);
        ((this + SRC + 4) as *mut u32).write_unaligned(RX1);
        ((this + SRC + 8) as *mut u32).write_unaligned(RX1);
        ((this + SRC + 12) as *mut u32).write_unaligned(RX0);
        ((this + DST) as *mut u32).write_unaligned(RX0);
        ((this + DST + 4) as *mut u32).write_unaligned(RX1);
        ((this + DST + 8) as *mut u32).write_unaligned(RX1);
        ((this + DST + 12) as *mut u32).write_unaligned(RX0);
        ((this + COLOUR) as *mut u32).write_unaligned(0xFFFF_FFFF);
        ((this + CSRC) as *mut u32).write_unaligned((colour_src as *const u32).read_unaligned());
        (lf_checker_rt::global::<u32>(COUNTER)).write_unaligned(counter.wrapping_add(1));
        core::ptr::copy_nonoverlapping(src_rect as *const u8, (this + SRC) as *mut u8, 16);
        core::ptr::copy_nonoverlapping(dst_rect as *const u8, (this + DST) as *mut u8, 16);
        ((this + COLOUR) as *mut u32).write_unaligned((colour as *const u32).read_unaligned());
    }
    this
});
