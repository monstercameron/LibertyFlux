// original: 0x00A6D050 CTaskComplexPlayerOnFoot::~CTaskComplexPlayerOnFoot

/// Clears four process-wide task slots, invokes the optional object callback,
/// releases an optional owned pointer, destroys the embedded object, and
/// tail-calls the base destructor.
///
/// The table cleanup helper is called for four cells spaced sixteen bytes
/// apart. The global words at `0x12fa6bc..0x12fa6c8` become `u32::MAX` and
/// the four bytes beginning at `0x12fa6cc` become zero. The optional callback
/// uses the object pointer and slot-zero function in its object vtable; its
/// one stack argument is `1`. The other helpers receive their object or field
/// pointers through thiscall registers and stack arguments. The word at
/// `this + 0x28` becomes `u32::MAX`, and the byte at `+0x2c` is cleared.
lf_checker_rt::export!(thiscall, rw_00a6d050(this: u32) -> u32 {
    unsafe {
        const CLEAR_TABLE_CELL: u32 = 2;
        const RELEASE_FIELD: u32 = 3;
        const DESTROY_EMBEDDED: u32 = 4;
        const BASE_DESTRUCTOR: u32 = 5;
        const TABLE_WORDS_VA: u32 = 0x12FA_6BC;
        const TABLE_CELLS_VA: u32 = 0x12FA_6F8;
        const TABLE_FLAGS_VA: u32 = 0x12FA_6CC;
        const CELL_COUNT: u32 = 4;
        const CELL_STRIDE: u32 = 0x10;

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00E9_EE14));
        for cell_index in 0..CELL_COUNT {
            let cell = lf_checker_rt::global::<u8>(TABLE_CELLS_VA)
                .add((cell_index * CELL_STRIDE) as usize) as u32;
            let _ = lf_checker_rt::callee_thiscall!(CLEAR_TABLE_CELL, u32, cell);
            lf_checker_rt::global::<u32>(TABLE_WORDS_VA)
                .add(cell_index as usize)
                .write_unaligned(u32::MAX);
            lf_checker_rt::global::<u8>(TABLE_FLAGS_VA)
                .add(cell_index as usize)
                .write(0);
        }

        let _ = lf_checker_rt::callee_thiscall!(CLEAR_TABLE_CELL, u32, this + 0x18);
        ((this + 0x28) as *mut u32).write_unaligned(u32::MAX);
        ((this + 0x2C) as *mut u8).write(0);
        let owned = ((this + 0x48) as *const u32).read_unaligned();
        if owned != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_FIELD, u32, owned, this + 0x48);
            ((this + 0x48) as *mut u32).write_unaligned(0);
        }

        let callback = ((this + 0x74) as *const u32).read_unaligned();
        if callback != 0 {
            let vtable = (callback as *const u32).read_unaligned();
            let entry = (vtable as *const u32).read_unaligned();
            let invoke: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(entry as usize);
            let _ = invoke(callback, 1);
            ((this + 0x74) as *mut u32).write_unaligned(0);
        }

        let _ = lf_checker_rt::callee_thiscall!(DESTROY_EMBEDDED, u32, this + 0x18);
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
