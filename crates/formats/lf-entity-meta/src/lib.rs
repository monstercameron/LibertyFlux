//! Vehicle, ped and weapon extra data inside model resources.
//!
//! Gameplay-critical data rides along with models: a vehicle fragment names
//! bones for its seats, doors, wheels and lights; a ped ships as a fragment
//! (the shared animation rig) plus a drawable dictionary (one drawable per
//! body component); a weapon drawable names bones for its grip, muzzle and
//! ejector. This crate reads that layer on top of [`lf_model`], which owns
//! the underlying resource parsing.
//!
//! The crate is original work written from public format documentation (the
//! `GTAMods` wiki model pages) and from inspecting the game's own files.
//! Open-source readers (`RageLib` in GTA4Unity/SparkIV, GPL, read for layout
//! only) informed the bone-name vocabulary; no code was copied from them.
//!
//! # Where things live
//!
//! ```text
//! archive              entries per model          what this crate reads
//! vehicles.img         <model>.wft + <model>.wtd  fragment skeleton bones
//! weapons.img          <model>.wdr (+2 .wft bags) drawable skeleton bones
//! componentpeds.img    <ped>.wft + .wdd + .wtd    rig bones + dict entries
//! pedprops.img         <prop>.wdd + .wtd          dict entries (hats etc)
//! playerped.rpf        player .wft/.wdd/.wtd      same as above (RPF)
//! ```
//!
//! Each episode ships its own set of these archives with the same layout.
//!
//! # Bone-name vocabulary (vehicles)
//!
//! Names are lowercase ASCII with a side (`dside` = driver side,
//! `pside` = passenger side) and a row (`f` = front, `r` = rear). The
//! classifier in [`classify`] recognises:
//!
//! ```text
//! seats    seat_dside_f, seat_pside_f, seat_dside_r, seat_pside_r,
//!          seat_f, seat_r (bikes)
//! doors    door_dside_f, door_pside_f, door_dside_r, door_pside_r
//! wheels   wheel_lf, wheel_rf, wheel_lr, wheel_rr (+ lm/rm on trucks),
//!          wheelmesh_* and wheelmeshbk_* (render meshes), hub_*, suspension_*
//! lights   headlight_l/r, taillight_l/r (+ bare taillight once),
//!          brakelight_l/r/m, indicator_lf/rf/lr/rr,
//!          reversinglight_l/r, interiorlight (+ linteriorlight once)
//! extras   extra_1 .. extra_10 (toggleable parts, see VehicleExtras.dat)
//! sirens   siren1..siren7 (+ siren_5 once), siren_glass1..7
//!          (+ sirenglass1..7 on two files)
//! body     chassis, bodyshell, bonnet, boot, bumper_f/r, windscreen(_r),
//!          window_*, wing_*, engine, exhaust(_2.._4), overheat(_2),
//!          petroltank (+ fueltank, petrolcap once each), transmission_*
//! bikes    seat_f/r, forks_u/l, hbgrip_l/r, swingarm, rider
//! boats    rudder, rudder2, static_prop(2)
//! helis    static_rotor1/2, moving_rotor1/2
//! misc     misc_a .. misc_h
//! ```
//!
//! Anything else classifies as [`BoneKind::Other`]; the name is kept so new
//! vocabulary shows up in output instead of failing a parse.
//!
//! # Ped rig
//!
//! Every ped fragment carries an 80-bone animation rig rooted at `Char`:
//! pelvis, legs, spine chain, neck, head, a facial-expression set
//! (`FB_*`, `PointFB_*`), arms, hands and fingers. Toe bones are spelled
//! `Char_L_Toe0` on most peds and `Char_L_Toe` on the rest; a few files
//! order the same 80 bones differently. Story characters extend the rig:
//! the three player rigs and two episode characters add articulated
//! thumbs (`Char_*_Finger4*`) for 86-90 bones, and the joint-based face
//! (`*_Jnt`, `C_*`) replaces the `FB_*` bones on some of them.
//! See [`PedRig`].
//!
//! Ped components (head, torso, legs and so on) are the entries of the
//! ped's `.wdd` drawable dictionary: each entry has a name hash and one
//! drawable, usually with two LODs. The hash algorithm is still unknown,
//! so components are reported by hash. Texture variants live in the
//! companion `.wtd`, which belongs to the texture lane. The playable
//! characters instead ship loose per-component drawables in `playerped.rpf`
//! (`head/hand/uppr/lowr/feet/hair/teef/sus2/suse` numbered variants with
//! matching per-variant texture files), plus a `player.wbs` file of unknown
//! purpose and a near-empty `player.wtd`.
//!
//! # Weapon mounts
//!
//! Firearm drawables name `gun_grip`, `gun_grip2`, `gun_muzzle`,
//! `gun_ejac` (one file spells it `gun_ejac01`), a magazine bone (`Mag`,
//! one file `mag`) and sometimes `sight_adjust`, plus one root bone named
//! after the weapon (`W_AK47` and so on). Melee weapons and ambient props
//! usually carry a single root bone and no mounts. See [`WeaponMounts`].
//!
//! # Positions
//!
//! Each [`Mount`] reports the bone's local position (from the bone record)
//! and, when the skeleton stores global transforms, its world position
//! (the translation row of the global matrix). RAGE matrices are
//! row-major, so the world offset is row 3; the integration test checks
//! this against the game files by mirroring (driver/passenger seat pairs
//! mirror in x, wheel pairs mirror in x with matching y/z).
//!
//! # What is still unknown
//!
//! - The `.wdd` entry hash algorithm (not lowercase joaat of the obvious
//!   component names; CRC32 and Jenkins also ruled out for those names).
//! - Which body component each ped hash denotes (needs per-entry geometry
//!   inspection against the texture lane's output).
//! - The `.rbs` files in the vehicle archive (two files, not resources).
//! - Fragment child records beyond bone index and flags (physics data).
//! - The `2DFX`/light-attribute block offsets that `lf-model` does not
//!   parse yet; corona positions may duplicate some light bones.

pub mod classify;
pub mod mount;
pub mod ped;
pub mod vehicle;
pub mod weapon;

pub use classify::{BoneKind, LightKind, Side, classify_bone};
pub use mount::Mount;
pub use ped::{Component, PedModel, PedRig};
pub use vehicle::{VehicleKind, VehicleLayout};
pub use weapon::WeaponMounts;

use std::fmt;

/// Error returned when parsing a whole file image through the convenience
/// constructors. Classification itself never fails.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The underlying model resource failed to parse.
    Model(lf_model::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Model(e) => write!(f, "model parse failed: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Model(e) => Some(e),
        }
    }
}

impl From<lf_model::Error> for Error {
    fn from(e: lf_model::Error) -> Self {
        Error::Model(e)
    }
}

/// Parse a vehicle fragment (`.wft` file image) into a [`VehicleLayout`].
///
/// # Errors
///
/// Returns an error when the bytes are not a readable model resource
/// or hold no vehicle fragment.
pub fn parse_vehicle(bytes: &[u8]) -> Result<VehicleLayout, Error> {
    let res = lf_model::Resource::open(bytes)?;
    let frag = lf_model::Fragment::parse(&res)?;
    Ok(VehicleLayout::from_fragment(&frag))
}

/// Parse a weapon or prop drawable (`.wdr` file image) into [`WeaponMounts`].
///
/// # Errors
///
/// Returns an error when the bytes are not a readable model resource
/// or hold no drawable.
pub fn parse_weapon(bytes: &[u8]) -> Result<WeaponMounts, Error> {
    let res = lf_model::Resource::open(bytes)?;
    let draw = lf_model::Drawable::parse(&res)?;
    Ok(WeaponMounts::from_drawable(&draw))
}

/// Parse a ped fragment plus its optional dictionary into a [`PedModel`].
///
/// Pass the `.wft` bytes and, when available, the same ped's `.wdd` bytes.
///
/// # Errors
///
/// Returns an error when the fragment (or dictionary) bytes are not a
/// readable model resource or hold no ped fragment.
pub fn parse_ped(fragment: &[u8], dictionary: Option<&[u8]>) -> Result<PedModel, Error> {
    let res = lf_model::Resource::open(fragment)?;
    let frag = lf_model::Fragment::parse(&res)?;
    let dict = dictionary
        .map(|b| {
            let r = lf_model::Resource::open(b)?;
            lf_model::DrawableDictionary::parse(&r).map_err(Error::Model)
        })
        .transpose()?;
    Ok(PedModel::from_parts(&frag, dict.as_ref()))
}
