//! Vehicle layout: seats, doors, wheels, lights and extras from bones.
//!
//! A vehicle fragment's skeleton names one bone per functional part. The
//! bone's global transform is the part's position in model space, which is
//! what the game drives occupants, steering, suspension and coronas from.

use crate::classify::{BoneKind, LightKind, classify_bone};
use crate::mount::Mount;

/// Best-guess vehicle shape from its bone set.
///
/// This is a heuristic for tooling, not a file field: helicopters have
/// rotor bones (checked first, since they also carry landing-gear wheel
/// bones), bikes have exactly two road wheels plus bike parts, boats have
/// a rudder or propeller and no wheels. Cars and trucks are everything
/// with road wheels left over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleKind {
    /// Four or six road wheels (cars, trucks, emergency).
    Car,
    /// Two road wheels (`wheel_lf` + `wheel_lr`) and bike parts.
    Bike,
    /// No wheels and a rudder.
    Boat,
    /// No wheels and rotor bones.
    Helicopter,
    /// None of the patterns above.
    Unknown,
}

/// The functional layout of one vehicle fragment.
#[derive(Debug, Clone, Default)]
pub struct VehicleLayout {
    /// Seat bones in skeleton order.
    pub seats: Vec<Mount>,
    /// Door bones in skeleton order.
    pub doors: Vec<Mount>,
    /// Road-wheel hubs (`wheel_*`) in skeleton order.
    pub wheels: Vec<Mount>,
    /// Light bones in skeleton order.
    pub lights: Vec<Mount>,
    /// Toggleable extras (`extra_N`) in skeleton order.
    pub extras: Vec<Mount>,
    /// Siren bones (not glass) in skeleton order.
    pub sirens: Vec<Mount>,
    /// Every other bone, in skeleton order.
    pub other: Vec<Mount>,
    /// Total bones in the fragment skeleton (0 when absent).
    pub bone_count: usize,
    /// Fragment child records (break-off parts) in the file.
    pub child_count: usize,
}

impl VehicleLayout {
    /// Build a layout from a parsed fragment.
    ///
    /// This never fails: unknown bone names land in [`VehicleLayout::other`]
    /// and a missing skeleton yields empty lists with `bone_count` 0.
    #[must_use]
    pub fn from_fragment(frag: &lf_model::Fragment) -> VehicleLayout {
        let mut out = VehicleLayout {
            child_count: frag.children.len(),
            ..VehicleLayout::default()
        };
        let Some(skel) = frag.drawable.skeleton.as_ref() else {
            return out;
        };
        out.bone_count = skel.bones.len();
        for (i, bone) in skel.bones.iter().enumerate() {
            let local = [bone.position[0], bone.position[1], bone.position[2]];
            let world = skel.global_pose.get(i).map(|m| [m[3][0], m[3][1], m[3][2]]);
            let mount = Mount::new(&bone.name, i, local, world);
            match classify_bone(&bone.name) {
                BoneKind::Seat => out.seats.push(mount),
                BoneKind::Door => out.doors.push(mount),
                BoneKind::Wheel => out.wheels.push(mount),
                BoneKind::Light(_) => out.lights.push(mount),
                BoneKind::Extra(_) => out.extras.push(mount),
                BoneKind::Siren => out.sirens.push(mount),
                _ => out.other.push(mount),
            }
        }
        out
    }

    /// Number of seats.
    #[must_use]
    pub fn seat_count(&self) -> usize {
        self.seats.len()
    }

    /// Number of road wheels.
    #[must_use]
    pub fn wheel_count(&self) -> usize {
        self.wheels.len()
    }

    /// Number of doors.
    #[must_use]
    pub fn door_count(&self) -> usize {
        self.doors.len()
    }

    /// Extra numbers present, in bone order (the `N` of `extra_N`).
    #[must_use]
    pub fn extra_numbers(&self) -> Vec<u8> {
        self.extras
            .iter()
            .filter_map(|m| match classify_bone(&m.name) {
                BoneKind::Extra(n) => Some(n),
                _ => None,
            })
            .collect()
    }

    /// Lights of one flavour.
    #[must_use]
    pub fn lights_of(&self, kind: LightKind) -> Vec<&Mount> {
        self.lights
            .iter()
            .filter(|m| classify_bone(&m.name) == BoneKind::Light(kind))
            .collect()
    }

    /// Iterate over every classified mount (seats, doors, wheels, lights,
    /// extras, sirens), in that group order.
    pub fn mounts(&self) -> impl Iterator<Item = &Mount> {
        self.seats
            .iter()
            .chain(self.doors.iter())
            .chain(self.wheels.iter())
            .chain(self.lights.iter())
            .chain(self.extras.iter())
            .chain(self.sirens.iter())
    }

    /// Best-guess vehicle shape from the bone set (see [`VehicleKind`]).
    #[must_use]
    pub fn kind(&self) -> VehicleKind {
        let names: Vec<&str> = self
            .mounts()
            .chain(self.other.iter())
            .map(|m| m.name.as_str())
            .collect();
        let has = |want: BoneKind| names.iter().any(|n| classify_bone(n) == want);
        let by_name = |p: &str| names.iter().any(|n| n.contains(p));
        // Rotors first: helicopters also carry landing-gear wheel bones.
        if has(BoneKind::RotorPart) {
            return VehicleKind::Helicopter;
        }
        if self.wheels.len() == 2 && (has(BoneKind::BikePart) || by_name("seat_f")) {
            return VehicleKind::Bike;
        }
        if !self.wheels.is_empty() {
            return VehicleKind::Car;
        }
        if has(BoneKind::BoatPart) {
            return VehicleKind::Boat;
        }
        VehicleKind::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a fragment-like skeleton holder out of bare names: a real
    /// [`lf_model::Fragment`] with a hand-made skeleton (no game bytes).
    fn fragment_with(names: &[&str]) -> lf_model::Fragment {
        use lf_model::{Drawable, Fragment, Skeleton};
        let bones = names
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let idx = i16::try_from(i).expect("fixture fits");
                lf_model::skeleton::Bone {
                    offset: 0,
                    name: n.to_string(),
                    index: idx,
                    bone_id: idx,
                    dofs: 0,
                    position: [f32::from(idx), 0.0, 0.0, 1.0],
                    rotation_euler: [0.0; 4],
                    rotation_quat: [0.0; 4],
                    parent: 0,
                    first_child: 0,
                    next_sibling: 0,
                }
            })
            .collect();
        let skel = Skeleton {
            bones,
            parents: vec![],
            default_pose: vec![],
            inverse_pose: vec![],
            global_pose: vec![],
            id_mappings: vec![],
        };
        Fragment {
            drawable: Drawable {
                vtable: 0,
                shaders: None,
                skeleton: Some(skel),
                center: [0.0; 4],
                bounds_min: [0.0; 4],
                bounds_max: [0.0; 4],
                abs_max: [0.0; 4],
                lods: vec![],
            },
            children: vec![],
        }
    }

    #[test]
    // Fixture test: the hand-built positions must pass through exactly.
    #[allow(clippy::float_cmp)]
    fn groups_a_car() {
        let frag = fragment_with(&[
            "chassis",
            "seat_dside_f",
            "seat_pside_f",
            "door_dside_f",
            "wheel_lf",
            "wheel_rr",
            "headlight_l",
            "extra_1",
            "siren2",
        ]);
        let lay = VehicleLayout::from_fragment(&frag);
        assert_eq!(lay.seat_count(), 2);
        assert_eq!(lay.door_count(), 1);
        assert_eq!(lay.wheel_count(), 2);
        assert_eq!(lay.lights.len(), 1);
        assert_eq!(lay.extra_numbers(), vec![1]);
        assert_eq!(lay.sirens.len(), 1);
        assert_eq!(lay.other.len(), 1);
        assert_eq!(lay.kind(), VehicleKind::Car);
        // Local positions come from the bone records.
        assert_eq!(lay.seats[0].local, [1.0, 0.0, 0.0]);
        assert!(lay.seats[0].world.is_none());
    }

    #[test]
    fn kinds_bike_boat_heli() {
        let bike = fragment_with(&["wheel_lf", "wheel_lr", "forks_u", "seat_f"]);
        assert_eq!(
            VehicleLayout::from_fragment(&bike).kind(),
            VehicleKind::Bike
        );
        let boat = fragment_with(&["rudder", "seat_dside_f"]);
        assert_eq!(
            VehicleLayout::from_fragment(&boat).kind(),
            VehicleKind::Boat
        );
        let heli = fragment_with(&[
            "moving_rotor1",
            "wheel_lf",
            "wheel_rf",
            "wheel_lr",
            "wheel_rr",
            "seat_dside_f",
        ]);
        assert_eq!(
            VehicleLayout::from_fragment(&heli).kind(),
            VehicleKind::Helicopter
        );
    }

    #[test]
    fn missing_skeleton_is_empty_not_error() {
        let mut frag = fragment_with(&["seat_dside_f"]);
        frag.drawable.skeleton = None;
        let lay = VehicleLayout::from_fragment(&frag);
        assert_eq!(lay.bone_count, 0);
        assert!(lay.seats.is_empty());
    }

    #[test]
    fn lights_filter_by_flavour() {
        let frag = fragment_with(&["headlight_l", "headlight_r", "brakelight_m"]);
        let lay = VehicleLayout::from_fragment(&frag);
        assert_eq!(lay.lights_of(LightKind::Headlight).len(), 2);
        assert_eq!(lay.lights_of(LightKind::Brakelight).len(), 1);
        assert!(lay.lights_of(LightKind::Indicator).is_empty());
    }
}
