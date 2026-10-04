// original: 0x00e5f3a0 store_constpair_ef78
/// Store this unit's constant pair into its two global slots.
///
/// The first slot gets the relocated code constant (its immediate carries a
/// relocation fixup, so it follows the image base) and a zero high word; the
/// second slot gets zero and the relocated pointer constant.
///
/// Note: the original fills the first slot with one 64-bit load over a stack
/// temporary of which only the low 32 bits were initialized, so the high word
/// it stores is uninitialized stack garbage. Under the checker's defined
/// stack fill that word is 0, which is what the rewrite stores; the value is
/// indeterminate in the running game.
export!(cdecl, rw_00e5f3a0() -> u32 {
    unsafe {
        const CODE_CONST: u32 = 0x43EA90;
        const SLOT_A: u32 = 0x110EF78;
        const SLOT_B: u32 = 0x110EF80;
        const PTR_CONST: u32 = 0x409610;
        (global::<u32>(SLOT_A)).write(relocated(CODE_CONST));
        (global::<u32>(SLOT_A + 4)).write(0);
        (global::<u32>(SLOT_B)).write(0);
        (global::<u32>(SLOT_B + 4)).write(relocated(PTR_CONST));
        0
    }
});
