// original: 0x00628FD0 rain_ptx_effect_create_context

/// Creates a context through the manager object selected by a global pointer.
///
/// The `thiscall` receiver is the destination state, and the single stack
/// argument is forwarded to both virtual methods. The global points to a
/// manager object whose vtable slot `+4` creates a second object; that
/// object's vtable slot `+0x0c` receives the same input with seven zero
/// options. The created object is stored at receiver offset `+0x0c`, byte
/// `+0x10` is cleared, and the function returns one in `AL`.
lf_checker_rt::export!(thiscall, rw_00628FD0(this: u32, input: u32) -> u32 {
    const MANAGER_GLOBAL_VA: u32 = 0x017F_5A04;
    const CREATE_SLOT: u32 = 0x04;
    const INITIALIZE_SLOT: u32 = 0x0C;
    const CREATED_OBJECT_OFFSET: u32 = 0x0C;
    const STATE_BYTE_OFFSET: u32 = 0x10;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn write_byte(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    unsafe {
        let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
        let manager_vtable = read_word(manager);
        let create_address = read_word(manager_vtable + CREATE_SLOT);
        let create: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(create_address as usize);
        let created = create(manager, input, 0, 0);

        write_word(this + CREATED_OBJECT_OFFSET, created);

        let created_vtable = read_word(created);
        let initialize_address = read_word(created_vtable + INITIALIZE_SLOT);
        let initialize: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(initialize_address as usize);
        initialize(created, input, 0, 0, 0, 0, 0, 0, 0);

        write_byte(this + STATE_BYTE_OFFSET, 0);
    }
    1
});
