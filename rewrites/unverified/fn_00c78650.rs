// original: 0x00C78650 phbound_register_defaults (proposed)

/// Register the default object type, create its registry object from the
/// static callback set, install the returned object, then register two
/// callback groups on the fixed manager. The initializer also stores the
/// manager pointer and sets the one-byte ready flag.
lf_checker_rt::export!(cdecl, rw_phbound_register_defaults() -> () {
    unsafe {
        const REGISTER_TYPE: u32 = 1;
        const CREATE_REGISTRY: u32 = 2;
        const INSTALL_REGISTRY: u32 = 3;
        const REGISTER_CALLBACKS_A: u32 = 4;
        const REGISTER_CALLBACKS_B: u32 = 5;
        const READY_FLAG: u32 = 0x0104_b87c;
        const MANAGER_POINTER: u32 = 0x0104_b888;
        const FIXED_MANAGER: u32 = 0x016f_7f80;

        let ready = lf_checker_rt::global::<u8>(READY_FLAG);
        unsafe { ready.write_volatile(1) };

        let type_name = lf_checker_rt::relocated(0x00ed_4364);
        let _ = lf_checker_rt::callee_cdecl!(
            REGISTER_TYPE,
            u32,
            0x14a_u32,
            type_name,
            3_u32,
            0x4248_0000_u32,
        );

        let defaults = lf_checker_rt::relocated(0x009d_3840);
        let callback_a = lf_checker_rt::relocated(0x00c7_8890);
        let callback_b = lf_checker_rt::relocated(0x00c7_8100);
        let callback_c = lf_checker_rt::relocated(0x00c7_8a50);
        let callback_d = lf_checker_rt::relocated(0x00c7_8a30);
        let common_callback = lf_checker_rt::relocated(0x0040_16a0);
        let shared_callback = lf_checker_rt::relocated(0x0042_c950);
        let process_callback = lf_checker_rt::relocated(0x0040_1680);
        let scope_callback = lf_checker_rt::relocated(0x00c7_8330);
        let entry_callback = lf_checker_rt::relocated(0x00c7_83b0);
        let registry_name = lf_checker_rt::relocated(0x00ed_4378);
        let registry_kind = lf_checker_rt::relocated(0x00ed_4374);
        let object = lf_checker_rt::callee_cdecl!(
            CREATE_REGISTRY,
            u32,
            registry_name,
            registry_kind,
            0x14a_u32,
            defaults,
            0_u32,
            callback_a,
            callback_b,
            callback_c,
            callback_d,
            common_callback,
            common_callback,
            common_callback,
            shared_callback,
            process_callback,
            scope_callback,
            entry_callback,
            0_u32,
            1_u32,
            0x20_u32,
        );
        let installed = lf_checker_rt::callee_thiscall!(
            INSTALL_REGISTRY,
            u32,
            object,
        );
        unsafe { lf_checker_rt::global::<u32>(MANAGER_POINTER).write_volatile(installed) };

        let manager = lf_checker_rt::relocated(FIXED_MANAGER);
        let group_a_name = lf_checker_rt::relocated(0x00ed_4380);
        let group_a_handler = lf_checker_rt::relocated(0x00c7_8910);
        let group_a_callback = lf_checker_rt::relocated(0x00c7_8190);
        let group_b_name = lf_checker_rt::relocated(0x00ed_4390);
        let group_b_handler = lf_checker_rt::relocated(0x00c7_8970);
        let group_b_callback = lf_checker_rt::relocated(0x00c7_81f0);
        let registration_callback = lf_checker_rt::relocated(0x0040_1690);
        let _ = lf_checker_rt::callee_thiscall!(
            REGISTER_CALLBACKS_A,
            u32,
            manager,
            group_a_name,
            group_a_handler,
            group_a_callback,
            registration_callback,
            8_u32,
        );
        let _ = lf_checker_rt::callee_thiscall!(
            REGISTER_CALLBACKS_B,
            u32,
            manager,
            group_b_name,
            group_b_handler,
            group_b_callback,
            registration_callback,
            0x30_u32,
        );
    }
});
