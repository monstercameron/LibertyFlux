// original: 0x00C78530 phbound_registry_configure (proposed)

/// Publish the configured registry key, create its default resource, and ask
/// the global manager to install four fixed bounds. If the manager returns a
/// resource, pass those bounds and the caller's mode to the installer and
/// publish its result; otherwise clear the installed-resource pointer. Copy
/// the caller's final float word to its global setting and clear the ready
/// byte.
lf_checker_rt::export!(cdecl, rw_phbound_registry_configure(
    descriptor_id: u32,
    descriptor_name: u32,
    registration_mode: u32,
    limit_bits: u32,
) -> () {
    unsafe {
        const INITIALIZE_REGISTRY: u32 = 1;
        const LOOKUP_DESCRIPTOR: u32 = 2;
        const CONFIGURE_DESCRIPTOR: u32 = 3;
        const GET_MANAGER_RESOURCE: u32 = 4;
        const INSTALL_RESOURCE: u32 = 5;
        const MANAGER_GLOBAL: u32 = 0x012f_b254;
        const REGISTRY_KEY_GLOBAL: u32 = 0x016e_c7b4;
        const INSTALLED_RESOURCE_GLOBAL: u32 = 0x016e_c7a8;
        const LIMIT_GLOBAL: u32 = 0x0104_b878;
        const READY_GLOBAL: u32 = 0x016e_c7a4;
        const NEGATIVE_BOUND_BITS: u32 = 0xc53b_8000;
        const POSITIVE_BOUND_BITS: u32 = 0x453b_8000;

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let _ = lf_checker_rt::callee_cdecl!(
            INITIALIZE_REGISTRY,
            u32,
            descriptor_id,
            descriptor_name,
        );
        let descriptor_key = lf_checker_rt::callee_cdecl!(
            LOOKUP_DESCRIPTOR,
            u32,
            descriptor_id,
        );
        unsafe { write_u32(lf_checker_rt::relocated(REGISTRY_KEY_GLOBAL), descriptor_key) };
        let _ = lf_checker_rt::callee_cdecl!(
            CONFIGURE_DESCRIPTOR,
            u32,
            descriptor_key,
            0_u32,
            descriptor_id,
        );

        let manager = unsafe {
            lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_volatile()
        };
        let resource = lf_checker_rt::callee_thiscall!(
            GET_MANAGER_RESOURCE,
            u32,
            manager,
            descriptor_id,
            NEGATIVE_BOUND_BITS,
            POSITIVE_BOUND_BITS,
            POSITIVE_BOUND_BITS,
            NEGATIVE_BOUND_BITS,
        );

        if resource != 0 {
            let mut bounds = [
                NEGATIVE_BOUND_BITS,
                POSITIVE_BOUND_BITS,
                POSITIVE_BOUND_BITS,
                NEGATIVE_BOUND_BITS,
            ];
            let bounds_pointer = bounds.as_mut_ptr() as u32;
            let installed = lf_checker_rt::callee_thiscall!(
                INSTALL_RESOURCE,
                u32,
                resource,
                bounds_pointer,
                registration_mode,
            );
            unsafe {
                write_u32(
                    lf_checker_rt::relocated(INSTALLED_RESOURCE_GLOBAL),
                    installed,
                )
            };
        } else {
            unsafe {
                write_u32(
                    lf_checker_rt::relocated(INSTALLED_RESOURCE_GLOBAL),
                    0,
                )
            };
        }

        unsafe { write_u32(lf_checker_rt::relocated(LIMIT_GLOBAL), limit_bits) };
        unsafe {
            lf_checker_rt::global::<u8>(READY_GLOBAL).write_volatile(0)
        };
    }
});
