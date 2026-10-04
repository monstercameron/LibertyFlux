// original: 0x00c18cc0 shared_mem_proc_init

/// Bring up the shared-memory channel and its helper process.
///
/// Returns 0 at once when any channel slot is already filled. Otherwise takes
/// a mutex, creates and maps an 8 MB file mapping, zeroes the mapping and a
/// process-startup struct, and spawns the helper. When the helper handshake
/// succeeds, records both process ids at the head of the mapping and releases
/// the mutex. Any failure runs the teardown helper and returns 0; success
/// returns 1. All operating-system calls go through import slots, which the
/// checker rewrites to recorder stubs; the rewrite loads them from the same
/// slots, exactly like the original.
export!(thiscall, rw_00c18cc0(this: u32) -> u32 {
    unsafe {
        let slot = |va: u32| (relocated(va) as *const u32).read() as usize;
        let create_mutex_a: extern "stdcall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(slot(0x00e7_3180));
        let create_file_mapping_a: extern "stdcall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot(0x00e7_32cc));
        let map_view_of_file: extern "stdcall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot(0x00e7_32c4));
        let create_process_a: extern "stdcall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot(0x00e7_327c));
        let get_current_process_id: extern "stdcall" fn() -> u32 =
            core::mem::transmute(slot(0x00e7_3178));
        let get_process_id: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(slot(0x00e7_32c0));
        let release_mutex: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(slot(0x00e7_31b0));
        if ((this + 0x30) as *const u32).read() != 0
            || ((this + 0x2c) as *const u32).read() != 0
            || ((this + 0x28) as *const u32).read() != 0
        {
            return 0;
        }
        let mutex = create_mutex_a(0, 0, relocated(0x00ec_43d4));
        ((this + 0x30) as *mut u32).write(mutex);
        if mutex == 0 {
            return 0;
        }
        let mapping =
            create_file_mapping_a(0xffff_ffff, 0, 4, 0, 0x007e_921c, relocated(0x00ec_44f0));
        ((this + 0x2c) as *mut u32).write(mapping);
        if mapping == 0 {
            callee_thiscall!(7, u32, this);
            return 0;
        }
        let view = map_view_of_file(mapping, 0x000f_001f, 0, 0, 0x007e_921c);
        ((this + 0x28) as *mut u32).write(view);
        if view == 0 {
            callee_thiscall!(7, u32, this);
            return 0;
        }
        callee_cdecl!(4, u32, view, 0, 0x007e_921c);
        let mut startup = [0u32; 17];
        let startup_ptr = (&mut startup as *mut u32) as u32;
        callee_cdecl!(5, u32, startup_ptr, 0, 0x44);
        let ok = create_process_a(
            relocated(0x00ec_450c),
            0,
            0,
            0,
            0,
            0x8000,
            0,
            0,
            startup_ptr,
            this + 0x14,
        );
        if ok == 0 {
            callee_thiscall!(7, u32, this);
            return 0;
        }
        if callee_thiscall!(8, u32, this) as u8 == 0 {
            return 1;
        }
        ((view + 0) as *mut u32).write(get_current_process_id());
        let child = ((this + 0x14) as *const u32).read();
        ((view + 4) as *mut u32).write(get_process_id(child));
        release_mutex(((this + 0x30) as *const u32).read());
    }
    1
});
