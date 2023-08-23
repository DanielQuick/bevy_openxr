use bevy::prelude::*;
use openxr as xr;

type XrPose = (Vec3, Quat);

#[derive(Resource)]
pub struct XrInput {
    action_set: xr::ActionSet,
    right_action: xr::Action<xr::Posef>,
    left_action: xr::Action<xr::Posef>,
    right_space: xr::Space,
    left_space: xr::Space,
    stage: xr::Space,
}

impl XrInput {
    pub fn new(
        instance: xr::Instance,
        session: xr::Session<xr::AnyGraphics>,
    ) -> xr::Result<Self> {
        let action_set = instance.create_action_set("input", "input pose information", 0)?;
        let right_action =
            action_set.create_action::<xr::Posef>("right_hand", "Right Hand Controller", &[])?;
        let left_action =
            action_set.create_action::<xr::Posef>("left_hand", "Left Hand Controller", &[])?;
        instance.suggest_interaction_profile_bindings(
            instance.string_to_path("/interaction_profiles/khr/simple_controller")?,
            &[
                xr::Binding::new(
                    &right_action,
                    instance.string_to_path("/user/hand/right/input/grip/pose")?,
                ),
                xr::Binding::new(
                    &left_action,
                    instance.string_to_path("/user/hand/left/input/grip/pose")?,
                ),
            ],
        )?;
        session.attach_action_sets(&[&action_set])?;
        let right_space =
            right_action.create_space(session.clone(), xr::Path::NULL, xr::Posef::IDENTITY)?;
        let left_space =
            left_action.create_space(session.clone(), xr::Path::NULL, xr::Posef::IDENTITY)?;
        let stage =
            session.create_reference_space(xr::ReferenceSpaceType::STAGE, xr::Posef::IDENTITY)?;
        Ok(Self {
            action_set,
            right_action,
            left_action,
            right_space,
            left_space,
            stage,
        })
    }
}
