// original: 0x00694B50 optional_component_helper_00694B50 (proposed)

/// Forward an optional component to its helper.
///
/// The first cdecl stack argument is unused; the second is the optional
/// component. A null component leaves incoming EAX untouched and makes no
/// call. Otherwise the component is passed both in ECX and as the helper's
/// stack argument, and the helper's EAX is returned.
lf_checker_rt::export!(cdecl, rw_00694B50(_unused: u32, component: u32) -> u32 {
    if component == 0 {
        return 0;
    }
    lf_checker_rt::callee_thiscall!(1, u32, component, component)
});
