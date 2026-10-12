// original: 0x00BAC280 NativeImpl_SET_CHAR_PROOFS
/// Set the five proof bits on a character selected by its pool handle.
///
/// Each flag is interpreted from its low byte. The function preserves every
/// other bit in the character's flags word and returns the resolved character
/// pointer in EAX. The handle lookup is intercepted by the checker.
lf_checker_rt::export!(cdecl, rw_bac280(
    handle: u32,
    bullet_proof: u32,
    fire_proof: u32,
    explosion_proof: u32,
    collision_proof: u32,
    melee_proof: u32,
) -> u32 {
    const PROOF_FLAGS_OFFSET: usize = 280;
    let character = unsafe { lf_checker_rt::callee_thiscall!(1, u32, 0u32, handle) };
    let flags_ptr = (character as usize as *mut u8).wrapping_add(PROOF_FLAGS_OFFSET) as *mut u32;
    let mut flags = unsafe { flags_ptr.read_unaligned() };
    flags = if (bullet_proof as u8) != 0 { flags | 0x40 } else { flags & !0x40 };
    flags = if (fire_proof as u8) != 0 { flags | 0x80 } else { flags & !0x80 };
    flags = if (explosion_proof as u8) != 0 { flags | 0x1000 } else { flags & !0x1000 };
    flags = if (collision_proof as u8) != 0 { flags | 0x100 } else { flags & !0x100 };
    flags = if (melee_proof as u8) != 0 { flags | 0x200 } else { flags & !0x200 };
    unsafe { flags_ptr.write_unaligned(flags) };
    character
});
