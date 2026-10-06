// original: 0x00a989c0 filemem_open_entry

/// Open one entry: probe it, create its object, set it up and register it.
///
/// `this` is the owning manager, `a1`/`a2` the entry key. The probe callee
/// answers nonzero (low byte, an exact zero check) when the entry exists;
/// otherwise returns null. The create callee then builds the object (null
/// means failure, returned as-is), the setup callee binds the key to it,
/// and the register callee files it under the global registry. Returns the
/// object, or null on either early exit.
///
/// Original: 0x00A989C0 (thiscall, two stack words; four direct callees).
lf_checker_rt::export!(thiscall, rw_00a989c0(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        /// Global object registry the entry is filed under (file VA).
        const REGISTRY: u32 = 0x0139497c;
        /// Probe callee: nonzero low byte means the entry exists.
        const PROBE: u32 = 1;
        /// Create callee: builds the object for `this`.
        const CREATE: u32 = 2;
        /// Setup callee: binds the key to the new object.
        const SETUP: u32 = 3;
        /// Register callee: files the object in the registry.
        const REGISTER: u32 = 4;

        let ok: u32 = lf_checker_rt::callee_stdcall!(PROBE, u32, a1, a2);
        if (ok as u8) == 0 {
            return 0;
        }
        let obj: u32 = lf_checker_rt::callee_thiscall!(CREATE, u32, this);
        if obj == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(SETUP, u32, obj, a1, a2);
        let _: u32 = lf_checker_rt::callee_thiscall!(REGISTER, u32, lf_checker_rt::relocated(REGISTRY), obj);
        obj
    }
});
