// original: 0x0097ad90 audPedAudioEntity::audPedAudioEntity
/// Constructor for the pedestrian audio entity.
///
/// Runs the base entity constructor, installs the class function table,
/// constructs the four embedded member objects, zeroes the scalar fields,
/// links the shared audio params block reached through a global handle, and
/// programs the first member with its default tuning constants. Returns the
/// object pointer.
export!(thiscall, rw_0097ad90(this_ptr: u32) -> u32 {
    const VTABLE: u32 = 0x00E8D6F4;
    const PARAMS_HANDLE: u32 = 0x01231290;
    const MEMBER_OFFS: [u32; 4] = [0x24, 0x44, 0x90, 0x17C];
    // Default tuning constants programmed into the first member: two small
    // factors, a zero, and 1.0f (0x3F800000).
    const TUNE_A: u32 = 0x3B03126F;
    const ONE: u32 = 0x3F800000;

    unsafe {
        // Base-class constructor.
        callee_thiscall!(1, u32, this_ptr);
        // Class function table.
        *(this_ptr as *mut u32) = relocated(VTABLE);
        // Embedded member constructors.
        for off in MEMBER_OFFS {
            callee_thiscall!(2, u32, this_ptr.wrapping_add(off));
        }
        // Zeroed scalar fields.
        for off in [0x120u32, 0x74, 0x08, 0x0C, 0x14, 0xAC, 0xB0, 0xB4] {
            *((this_ptr + off) as *mut u32) = 0;
        }
        *((this_ptr + 0x128) as *mut u8) = 0;
        *((this_ptr + 0x10) as *mut u8) = 0;
        // Shared params block reached through a double-indirect global handle.
        let outer = *(global::<u32>(PARAMS_HANDLE) as *const u32);
        let inner = *(outer as *const u32);
        *((this_ptr + 0xB8) as *mut u32) = inner;
        for off in [0x1Cu32, 0x130, 0x134, 0x138, 0x13C] {
            *((this_ptr + off) as *mut u32) = 0;
        }
        // Default tuning for the first member.
        callee_thiscall!(3, u32, this_ptr.wrapping_add(0x24), TUNE_A, TUNE_A, 0, ONE);
        for off in [0x20u32, 0x18, 0x124, 0x140, 0x144, 0x198, 0x19C] {
            *((this_ptr + off) as *mut u32) = 0;
        }
        *((this_ptr + 0x1A0) as *mut u8) = 0;
        this_ptr
    }
});
