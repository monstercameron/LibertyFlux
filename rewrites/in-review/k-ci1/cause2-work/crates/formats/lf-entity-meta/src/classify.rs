//! Bone-name classifier.
//!
//! The game addresses vehicle parts, ped joints and weapon mounts by bone
//! name. [`classify_bone`] sorts a name into a [`BoneKind`]; it is a pure
//! function over the name, so it is unit-tested without any game bytes.

/// Which side of a vehicle a part sits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// Driver side (`dside`, or bare `l` in lights/wheels).
    Driver,
    /// Passenger side (`pside`, or bare `r`).
    Middle,
}

/// Vehicle light flavour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightKind {
    /// `headlight_l/r`.
    Headlight,
    /// `taillight_l/r` (and one bare `taillight`).
    Taillight,
    /// `brakelight_l/r/m`.
    Brakelight,
    /// `indicator_lf/rf/lr/rr`.
    Indicator,
    /// `reversinglight_l/r`.
    Reversing,
    /// `interiorlight` (and one `linteriorlight`).
    Interior,
}

/// What a bone is for, decided from its name.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BoneKind {
    /// `seat_*`: a place a ped can sit.
    Seat,
    /// `door_*`: an openable door.
    Door,
    /// `wheel_*` (not `wheelmesh_*`): a wheel hub position.
    Wheel,
    /// `wheelmesh_*`, `wheelmeshbk_*`: wheel render meshes.
    WheelMesh,
    /// `suspension_*`, `hub_*`.
    RunningGear,
    /// `window_*`, `windscreen*`, `wing_*`.
    Glass,
    /// `headlight_*`, `taillight_*`, `brakelight_*`, `indicator_*`,
    /// `reversinglight_*`, `interiorlight`.
    Light(LightKind),
    /// `extra_N`: a toggleable part (see VehicleExtras.dat).
    Extra(u8),
    /// `misc_a` .. `misc_h`.
    Misc,
    /// `siren*` except glass.
    Siren,
    /// `siren_glass*`, `sirenglass*`.
    SirenGlass,
    /// Structural/mechanical shell: `chassis`, `bodyshell`, `bonnet`,
    /// `boot`, `bumper_*`, `engine`, `exhaust*`, `overheat*`,
    /// `petroltank`, `fueltank`, `petrolcap`, `transmission_*`, `glass`.
    Body,
    /// Bike-only: `forks_*`, `hbgrip_*`, `swingarm`, `rider`.
    BikePart,
    /// Boat-only: `rudder*`, `static_prop*`.
    BoatPart,
    /// Heli-only: `static_rotor*`, `moving_rotor*`.
    RotorPart,
    /// Ped rig bone (`Char*`, `FB_*`, `PointFB_*`, `*_Roll`, `Neck_Roll`,
    /// `Extra_0*`, `L_ArmRoll`, `L_UpperArmRoll`).
    PedRig,
    /// Weapon mount: `gun_grip`, `gun_grip2`, `gun_muzzle`, `gun_ejac*`,
    /// `Mag`/`mag`, `sight_adjust`, `cue_grip`, `Pull_pin`, `FOLD_STOCK`.
    WeaponMount,
    /// Anything else; the name is kept on the mount for inspection.
    Other,
}

/// Sort a bone name into a [`BoneKind`].
///
/// Matching is exact for the known vocabulary (plus a small set of
/// one-off spellings found in shipped files, listed in the crate docs).
/// Unknown names return [`BoneKind::Other`]; classification never fails.
#[must_use]
pub fn classify_bone(name: &str) -> BoneKind {
    if name.starts_with("seat") {
        return BoneKind::Seat;
    }
    if name.starts_with("door") {
        return BoneKind::Door;
    }
    if name.starts_with("wheelmesh") {
        return BoneKind::WheelMesh;
    }
    if name.starts_with("wheel_") {
        return BoneKind::Wheel;
    }
    if name.starts_with("suspension_") || name.starts_with("hub_") {
        return BoneKind::RunningGear;
    }
    if name.starts_with("window") || name.starts_with("windscreen") || name.starts_with("wing_") {
        return BoneKind::Glass;
    }
    if let Some(k) = classify_light(name) {
        return BoneKind::Light(k);
    }
    if let Some(rest) = name
        .strip_prefix("extra_")
        .or_else(|| name.strip_prefix("extra"))
    {
        if let Ok(n) = rest.parse::<u8>() {
            return BoneKind::Extra(n);
        }
        return BoneKind::Other;
    }
    if name.starts_with("sirenglass") || name.starts_with("siren_glass") {
        return BoneKind::SirenGlass;
    }
    if name.starts_with("siren") {
        return BoneKind::Siren;
    }
    if name.starts_with("misc_") || name == "mics_a" {
        return BoneKind::Misc;
    }
    if name.starts_with("forks_")
        || name.starts_with("hbgrip_")
        || name.starts_with("handlebar")
        || name == "swingarm"
        || name == "rider"
    {
        return BoneKind::BikePart;
    }
    if name.starts_with("rudder") {
        return BoneKind::BoatPart;
    }
    if name.contains("rotor") {
        return BoneKind::RotorPart;
    }
    if name.starts_with("static_prop") {
        // Boat propellers, not helicopter rotors ( verified: the eight
        // base-game boats carry these, helicopters carry *_rotor* ).
        return BoneKind::BoatPart;
    }
    if is_body(name) {
        return BoneKind::Body;
    }
    if is_ped_rig(name) {
        return BoneKind::PedRig;
    }
    if is_weapon_mount(name) {
        return BoneKind::WeaponMount;
    }
    BoneKind::Other
}

/// Which side a name refers to, from its `dside`/`pside` or trailing
/// `_l`/`_r`/`lf`/`rf`/`lr`/`rr` marker. Returns `None` for middle and
/// unmarked names.
#[must_use]
pub fn side_of(name: &str) -> Option<Side> {
    if name.contains("dside") {
        return Some(Side::Driver);
    }
    if name.contains("pside") {
        return Some(Side::Middle);
    }
    // Wheel/light style suffixes.
    if name.ends_with("_l")
        || name.ends_with("_lf")
        || name.ends_with("_lr")
        || name.ends_with("light_l")
        || name == "taillight"
    {
        // Bare `taillight` sits on the left in its file; treat as driver.
        return Some(Side::Driver);
    }
    if name.ends_with("_r")
        || name.ends_with("_rf")
        || name.ends_with("_rr")
        || name.ends_with("light_r")
    {
        return Some(Side::Middle);
    }
    if name.ends_with("_m") || name.ends_with("_f") && name.starts_with("brakelight") {
        return Some(Side::Middle);
    }
    None
}

fn classify_light(name: &str) -> Option<LightKind> {
    if name.starts_with("headlight") {
        Some(LightKind::Headlight)
    } else if name.starts_with("taillight") {
        Some(LightKind::Taillight)
    } else if name.starts_with("brakelight") {
        Some(LightKind::Brakelight)
    } else if name.starts_with("indicator") {
        Some(LightKind::Indicator)
    } else if name.starts_with("reversinglight") {
        Some(LightKind::Reversing)
    } else if name == "interiorlight" || name == "linteriorlight" {
        Some(LightKind::Interior)
    } else {
        None
    }
}

fn is_body(name: &str) -> bool {
    matches!(
        name,
        "chassis"
            | "bodyshell"
            | "bonnet"
            | "boot"
            | "bumper_f"
            | "bumper_r"
            | "engine"
            | "overheat"
            | "overheat_2"
            | "overheat2"
            | "petroltank"
            | "fueltank"
            | "petrolcap"
            | "glass"
    ) || name.starts_with("exhaust")
        || name.starts_with("transmission_")
}

fn is_ped_rig(name: &str) -> bool {
    name.starts_with("Char")
        || name.starts_with("FB_")
        || name.starts_with("PointFB_")
        || name.ends_with("_Roll")
        || name == "Neck_Roll"
        || name == "L_ArmRoll"
        || name == "L_UpperArmRoll"
        || name == "R_ArmRoll"
        || name == "R_UpperArmRoll"
        || name.starts_with("Extra_0")
        // Story-character face joints (ig_niko, player rigs).
        || name.ends_with("Jnt")
        || name.starts_with("C_")
}

fn is_weapon_mount(name: &str) -> bool {
    name.starts_with("gun_")
        || name == "Mag"
        || name == "mag"
        || name == "sight_adjust"
        || name == "cue_grip"
        || name == "Pull_pin"
        || name == "FOLD_STOCK"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vehicle_vocabulary() {
        assert_eq!(classify_bone("seat_dside_f"), BoneKind::Seat);
        assert_eq!(classify_bone("seat_r"), BoneKind::Seat);
        assert_eq!(classify_bone("door_pside_r"), BoneKind::Door);
        assert_eq!(classify_bone("wheel_lm"), BoneKind::Wheel);
        assert_eq!(classify_bone("wheelmesh_lf_l1"), BoneKind::WheelMesh);
        assert_eq!(classify_bone("wheelmeshbk_f"), BoneKind::WheelMesh);
        assert_eq!(classify_bone("suspension_rr"), BoneKind::RunningGear);
        assert_eq!(classify_bone("hub_lf"), BoneKind::RunningGear);
        assert_eq!(classify_bone("windscreen_r"), BoneKind::Glass);
        assert_eq!(classify_bone("extra_10"), BoneKind::Extra(10));
        assert_eq!(classify_bone("extra1"), BoneKind::Extra(1));
        assert_eq!(classify_bone("mics_a"), BoneKind::Misc);
        assert_eq!(classify_bone("handlebars"), BoneKind::BikePart);
        assert_eq!(classify_bone("misc_h"), BoneKind::Misc);
        assert_eq!(classify_bone("siren3"), BoneKind::Siren);
        assert_eq!(classify_bone("siren_5"), BoneKind::Siren);
        assert_eq!(classify_bone("siren_glass4"), BoneKind::SirenGlass);
        assert_eq!(classify_bone("sirenglass2"), BoneKind::SirenGlass);
        assert_eq!(classify_bone("chassis"), BoneKind::Body);
        assert_eq!(classify_bone("exhaust_4"), BoneKind::Body);
        assert_eq!(classify_bone("overheat2"), BoneKind::Body);
        assert_eq!(classify_bone("forks_u"), BoneKind::BikePart);
        assert_eq!(classify_bone("swingarm"), BoneKind::BikePart);
        assert_eq!(classify_bone("rudder2"), BoneKind::BoatPart);
        assert_eq!(classify_bone("static_prop"), BoneKind::BoatPart);
        assert_eq!(classify_bone("moving_rotor1"), BoneKind::RotorPart);
        assert_eq!(classify_bone("static_rotor2"), BoneKind::RotorPart);
    }

    #[test]
    fn lights() {
        assert_eq!(
            classify_bone("headlight_l"),
            BoneKind::Light(LightKind::Headlight)
        );
        assert_eq!(
            classify_bone("taillight"),
            BoneKind::Light(LightKind::Taillight)
        );
        assert_eq!(
            classify_bone("brakelight_m"),
            BoneKind::Light(LightKind::Brakelight)
        );
        assert_eq!(
            classify_bone("indicator_rf"),
            BoneKind::Light(LightKind::Indicator)
        );
        assert_eq!(
            classify_bone("reversinglight_l"),
            BoneKind::Light(LightKind::Reversing)
        );
        assert_eq!(
            classify_bone("linteriorlight"),
            BoneKind::Light(LightKind::Interior)
        );
    }

    #[test]
    fn ped_and_weapon() {
        assert_eq!(classify_bone("Char_Spine2"), BoneKind::PedRig);
        assert_eq!(classify_bone("FB_L_Eyeball"), BoneKind::PedRig);
        assert_eq!(classify_bone("PointFB_C_Jaw"), BoneKind::PedRig);
        assert_eq!(classify_bone("L_Calf_Roll"), BoneKind::PedRig);
        assert_eq!(classify_bone("Neck_Roll"), BoneKind::PedRig);
        assert_eq!(classify_bone("Extra_01"), BoneKind::PedRig);
        assert_eq!(classify_bone("C_jawJnt"), BoneKind::PedRig);
        assert_eq!(classify_bone("C_forehead"), BoneKind::PedRig);
        assert_eq!(classify_bone("r_EyeJnt"), BoneKind::PedRig);
        assert_eq!(classify_bone("gun_muzzle"), BoneKind::WeaponMount);
        assert_eq!(classify_bone("gun_ejac01"), BoneKind::WeaponMount);
        assert_eq!(classify_bone("Mag"), BoneKind::WeaponMount);
        assert_eq!(classify_bone("mag"), BoneKind::WeaponMount);
        assert_eq!(classify_bone("sight_adjust"), BoneKind::WeaponMount);
        assert_eq!(classify_bone("W_AK47"), BoneKind::Other);
        assert_eq!(classify_bone("AMB_broom"), BoneKind::Other);
        assert_eq!(classify_bone("Cereal"), BoneKind::Other);
    }

    #[test]
    fn sides() {
        assert_eq!(side_of("seat_dside_f"), Some(Side::Driver));
        assert_eq!(side_of("door_pside_r"), Some(Side::Middle));
        assert_eq!(side_of("wheel_lf"), Some(Side::Driver));
        assert_eq!(side_of("wheel_rr"), Some(Side::Middle));
        assert_eq!(side_of("headlight_r"), Some(Side::Middle));
        assert_eq!(side_of("brakelight_m"), Some(Side::Middle));
        assert_eq!(side_of("bodyshell"), None);
        assert_eq!(side_of("seat_f"), None);
    }
}
