pub enum BoneNaming {
  Default,
  Mecanim,
}
#[derive(Clone, Copy)]
pub enum BoneType {
  Ground,
  Hips,
  SpineLower,
  SpineMiddle,
  SpineUpper,
  Neck,
  Head,
  CollarLeft,
  ShoulderLeft,
  ElbowLeft,
  HandLeft,
  ThumbLeft0,
  ThumbLeft1,
  ThumbLeft2,
  IndexLeft0,
  IndexLeft1,
  IndexLeft2,
  MiddleLeft0,
  MiddleLeft1,
  MiddleLeft2,
  RingLeft0,
  RingLeft1,
  RingLeft2,
  PinkyLeft0,
  PinkyLeft1,
  PinkyLeft2,
  CollarRight,
  ShoulderRight,
  ElbowRight,
  HandRight,
  ThumbRight0,
  ThumbRight1,
  ThumbRight2,
  IndexRight0,
  IndexRight1,
  IndexRight2,
  MiddleRight0,
  MiddleRight1,
  MiddleRight2,
  RingRight0,
  RingRight1,
  RingRight2,
  PinkyRight0,
  PinkyRight1,
  PinkyRight2,
  HipLeft,
  KneeLeft,
  FootLeft,
  ToeLeft,
  HipRight,
  KneeRight,
  FootRight,
  ToeRight,
  Jaw,
  EyelidLowerLeft,
  EyelidUpperLeft,
  EyeballLeft,
  MouthCornerLeft,
  EyebrowLeft0,
  EyebrowLeft1,
  EyebrowLeft2,
  EyelidLowerRight,
  EyelidUpperRight,
  EyeballRight,
  MouthCornerRight,
  EyebrowRight0,
  EyebrowRight1,
  EyebrowRight2,
  Pelvis,
}

pub struct Converter {
  pub bone_dictionary: std::collections::HashMap<&'static str, BoneType>,
}

impl Converter {
  pub fn new() -> Converter {
    let mut bone_dictionary = std::collections::HashMap::new();
    bone_dictionary.insert("root ground", BoneType::Ground);
    bone_dictionary.insert("root hips", BoneType::Hips);
    bone_dictionary.insert("pelvis", BoneType::Pelvis);
    bone_dictionary.insert("leg left thigh", BoneType::HipLeft);
    bone_dictionary.insert("leg left knee", BoneType::KneeLeft);
    bone_dictionary.insert("leg left ankle", BoneType::FootLeft);
    bone_dictionary.insert("leg left toes", BoneType::ToeLeft);
    bone_dictionary.insert("leg right thigh", BoneType::HipRight);
    bone_dictionary.insert("leg right knee", BoneType::KneeRight);
    bone_dictionary.insert("leg right ankle", BoneType::FootRight);
    bone_dictionary.insert("leg right toes", BoneType::ToeRight);
    bone_dictionary.insert("spine lower", BoneType::SpineLower);
    bone_dictionary.insert("spine middle", BoneType::SpineMiddle);
    bone_dictionary.insert("spine upper", BoneType::SpineUpper);
    bone_dictionary.insert("head neck lower", BoneType::Neck);
    bone_dictionary.insert("head neck upper", BoneType::Head);
    bone_dictionary.insert("head jaw", BoneType::Jaw);
    bone_dictionary.insert("head eyeball left", BoneType::EyeballLeft);
    bone_dictionary.insert("head eyeball right", BoneType::EyeballRight);
    bone_dictionary.insert("head eyelid upper right", BoneType::EyelidUpperRight);
    bone_dictionary.insert("head eyelid lower right", BoneType::EyelidLowerRight);
    bone_dictionary.insert("head eyelid upper left", BoneType::EyelidUpperLeft);
    bone_dictionary.insert("head eyelid lower left", BoneType::EyelidLowerLeft);
    bone_dictionary.insert("head eyelid right upper", BoneType::EyelidUpperRight);
    bone_dictionary.insert("head eyelid right lower", BoneType::EyelidLowerRight);
    bone_dictionary.insert("head eyelid left upper", BoneType::EyelidUpperLeft);
    bone_dictionary.insert("head eyelid left lower", BoneType::EyelidLowerLeft);
    bone_dictionary.insert("head eyebrow right a", BoneType::EyebrowRight0);
    bone_dictionary.insert("head eyebrow right b", BoneType::EyebrowRight1);
    bone_dictionary.insert("head eyebrow right c", BoneType::EyebrowRight2);
    bone_dictionary.insert("head eyebrow left a", BoneType::EyebrowLeft0);
    bone_dictionary.insert("head eyebrow left b", BoneType::EyebrowLeft1);
    bone_dictionary.insert("head eyebrow left c", BoneType::EyebrowLeft2);
    bone_dictionary.insert("head eyebrow right 1", BoneType::EyebrowRight0);
    bone_dictionary.insert("head eyebrow right 2", BoneType::EyebrowRight1);
    bone_dictionary.insert("head eyebrow right 3", BoneType::EyebrowRight2);
    bone_dictionary.insert("head eyebrow left 1", BoneType::EyebrowLeft0);
    bone_dictionary.insert("head eyebrow left 2", BoneType::EyebrowLeft1);
    bone_dictionary.insert("head eyebrow left 3", BoneType::EyebrowLeft2);
    bone_dictionary.insert("head mouth corner right", BoneType::MouthCornerRight);
    bone_dictionary.insert("head mouth corner left", BoneType::MouthCornerLeft);
    bone_dictionary.insert("arm left shoulder 1", BoneType::CollarLeft);
    bone_dictionary.insert("arm left shoulder 2", BoneType::ShoulderLeft);
    bone_dictionary.insert("arm left shoulder a", BoneType::CollarLeft);
    bone_dictionary.insert("arm left shoulder b", BoneType::ShoulderLeft);
    bone_dictionary.insert("arm left elbow", BoneType::ElbowLeft);
    bone_dictionary.insert("arm left wrist", BoneType::HandLeft);
    bone_dictionary.insert("arm left wirst", BoneType::HandLeft);
    bone_dictionary.insert("arm left finger 1a", BoneType::ThumbLeft0);
    bone_dictionary.insert("arm left finger 1b", BoneType::ThumbLeft1);
    bone_dictionary.insert("arm left finger 1c", BoneType::ThumbLeft2);
    bone_dictionary.insert("arm left finger 2a", BoneType::IndexLeft0);
    bone_dictionary.insert("arm left finger 2b", BoneType::IndexLeft1);
    bone_dictionary.insert("arm left finger 2c", BoneType::IndexLeft2);
    bone_dictionary.insert("arm left finger 3a", BoneType::MiddleLeft0);
    bone_dictionary.insert("arm left finger 3b", BoneType::MiddleLeft1);
    bone_dictionary.insert("arm left finger 3c", BoneType::MiddleLeft2);
    bone_dictionary.insert("arm left finger 4a", BoneType::RingLeft0);
    bone_dictionary.insert("arm left finger 4b", BoneType::RingLeft1);
    bone_dictionary.insert("arm left finger 4c", BoneType::RingLeft2);
    bone_dictionary.insert("arm left finger 5a", BoneType::PinkyLeft0);
    bone_dictionary.insert("arm left finger 5b", BoneType::PinkyLeft1);
    bone_dictionary.insert("arm left finger 5c", BoneType::PinkyLeft2);
    bone_dictionary.insert("arm right shoulder 1", BoneType::CollarRight);
    bone_dictionary.insert("arm right shoulder 2", BoneType::ShoulderRight);
    bone_dictionary.insert("arm right shoulder a", BoneType::CollarRight);
    bone_dictionary.insert("arm right shoulder b", BoneType::ShoulderRight);
    bone_dictionary.insert("arm right elbow", BoneType::ElbowRight);
    bone_dictionary.insert("arm right wrist", BoneType::HandRight);
    bone_dictionary.insert("arm right wirst", BoneType::HandRight);
    bone_dictionary.insert("arm right finger 1a", BoneType::ThumbRight0);
    bone_dictionary.insert("arm right finger 1b", BoneType::ThumbRight1);
    bone_dictionary.insert("arm right finger 1c", BoneType::ThumbRight2);
    bone_dictionary.insert("arm right finger 2a", BoneType::IndexRight0);
    bone_dictionary.insert("arm right finger 2b", BoneType::IndexRight1);
    bone_dictionary.insert("arm right finger 2c", BoneType::IndexRight2);
    bone_dictionary.insert("arm right finger 3a", BoneType::MiddleRight0);
    bone_dictionary.insert("arm right finger 3b", BoneType::MiddleRight1);
    bone_dictionary.insert("arm right finger 3c", BoneType::MiddleRight2);
    bone_dictionary.insert("arm right finger 4a", BoneType::RingRight0);
    bone_dictionary.insert("arm right finger 4b", BoneType::RingRight1);
    bone_dictionary.insert("arm right finger 4c", BoneType::RingRight2);
    bone_dictionary.insert("arm right finger 5a", BoneType::PinkyRight0);
    bone_dictionary.insert("arm right finger 5b", BoneType::PinkyRight1);
    bone_dictionary.insert("arm right finger 5c", BoneType::PinkyRight2);
    bone_dictionary.insert("mixamorig_Hips", BoneType::Hips);
    bone_dictionary.insert("mixamorig_Spine", BoneType::SpineLower);
    bone_dictionary.insert("mixamorig_Spine1", BoneType::SpineMiddle);
    bone_dictionary.insert("mixamorig_Spine2", BoneType::SpineUpper);
    bone_dictionary.insert("mixamorig_Neck", BoneType::Neck);
    bone_dictionary.insert("mixamorig_Head", BoneType::Head);
    bone_dictionary.insert("mixamorig_LeftShoulder", BoneType::CollarLeft);
    bone_dictionary.insert("mixamorig_LeftArm", BoneType::ShoulderLeft);
    bone_dictionary.insert("mixamorig_LeftForeArm", BoneType::ElbowLeft);
    bone_dictionary.insert("mixamorig_LeftHand", BoneType::HandLeft);
    bone_dictionary.insert("mixamorig_LeftHandThumb1", BoneType::ThumbLeft0);
    bone_dictionary.insert("mixamorig_LeftHandThumb2", BoneType::ThumbLeft1);
    bone_dictionary.insert("mixamorig_LeftHandThumb3", BoneType::ThumbLeft2);
    bone_dictionary.insert("mixamorig_LeftHandIndex1", BoneType::IndexLeft0);
    bone_dictionary.insert("mixamorig_LeftHandIndex2", BoneType::IndexLeft1);
    bone_dictionary.insert("mixamorig_LeftHandIndex3", BoneType::IndexLeft2);
    bone_dictionary.insert("mixamorig_LeftHandMiddle1", BoneType::MiddleLeft0);
    bone_dictionary.insert("mixamorig_LeftHandMiddle2", BoneType::MiddleLeft1);
    bone_dictionary.insert("mixamorig_LeftHandMiddle3", BoneType::MiddleLeft2);
    bone_dictionary.insert("mixamorig_LeftHandRing1", BoneType::RingLeft0);
    bone_dictionary.insert("mixamorig_LeftHandRing2", BoneType::RingLeft1);
    bone_dictionary.insert("mixamorig_LeftHandRing3", BoneType::RingLeft2);
    bone_dictionary.insert("mixamorig_LeftHandPinky1", BoneType::PinkyLeft0);
    bone_dictionary.insert("mixamorig_LeftHandPinky2", BoneType::PinkyLeft1);
    bone_dictionary.insert("mixamorig_LeftHandPinky3", BoneType::PinkyLeft2);
    bone_dictionary.insert("mixamorig_RightShoulder", BoneType::CollarRight);
    bone_dictionary.insert("mixamorig_RightArm", BoneType::ShoulderRight);
    bone_dictionary.insert("mixamorig_RightForeArm", BoneType::ElbowRight);
    bone_dictionary.insert("mixamorig_RightHand", BoneType::HandRight);
    bone_dictionary.insert("mixamorig_RightHandThumb1", BoneType::ThumbRight0);
    bone_dictionary.insert("mixamorig_RightHandThumb2", BoneType::ThumbRight1);
    bone_dictionary.insert("mixamorig_RightHandThumb3", BoneType::ThumbRight2);
    bone_dictionary.insert("mixamorig_RightHandIndex1", BoneType::IndexRight0);
    bone_dictionary.insert("mixamorig_RightHandIndex2", BoneType::IndexRight1);
    bone_dictionary.insert("mixamorig_RightHandIndex3", BoneType::IndexRight2);
    bone_dictionary.insert("mixamorig_RightHandMiddle1", BoneType::MiddleRight0);
    bone_dictionary.insert("mixamorig_RightHandMiddle2", BoneType::MiddleRight1);
    bone_dictionary.insert("mixamorig_RightHandMiddle3", BoneType::MiddleRight2);
    bone_dictionary.insert("mixamorig_RightHandRing1", BoneType::RingRight0);
    bone_dictionary.insert("mixamorig_RightHandRing2", BoneType::RingRight1);
    bone_dictionary.insert("mixamorig_RightHandRing3", BoneType::RingRight2);
    bone_dictionary.insert("mixamorig_RightHandPinky1", BoneType::PinkyRight0);
    bone_dictionary.insert("mixamorig_RightHandPinky2", BoneType::PinkyRight1);
    bone_dictionary.insert("mixamorig_RightHandPinky3", BoneType::PinkyRight2);
    bone_dictionary.insert("mixamorig_LeftUpLeg", BoneType::HipLeft);
    bone_dictionary.insert("mixamorig_LeftLeg", BoneType::KneeLeft);
    bone_dictionary.insert("mixamorig_LeftFoot", BoneType::FootLeft);
    bone_dictionary.insert("mixamorig_LeftToeBase", BoneType::ToeLeft);
    bone_dictionary.insert("mixamorig_RightUpLeg", BoneType::HipRight);
    bone_dictionary.insert("mixamorig_RightLeg", BoneType::KneeRight);
    bone_dictionary.insert("mixamorig_RightFoot", BoneType::FootRight);
    bone_dictionary.insert("mixamorig_RightToeBase", BoneType::ToeRight);
    bone_dictionary.insert("Genesis", BoneType::Ground);
    bone_dictionary.insert("hip", BoneType::Hips);
    bone_dictionary.insert("lThigh", BoneType::HipLeft);
    bone_dictionary.insert("lShin", BoneType::KneeLeft);
    bone_dictionary.insert("lFoot", BoneType::FootLeft);
    bone_dictionary.insert("lToe", BoneType::ToeLeft);
    bone_dictionary.insert("rThigh", BoneType::HipRight);
    bone_dictionary.insert("rShin", BoneType::KneeRight);
    bone_dictionary.insert("rFoot", BoneType::FootRight);
    bone_dictionary.insert("rToe", BoneType::ToeRight);
    bone_dictionary.insert("abdomen", BoneType::SpineLower);
    bone_dictionary.insert("abdomen2", BoneType::SpineMiddle);
    bone_dictionary.insert("chest", BoneType::SpineUpper);
    bone_dictionary.insert("neck", BoneType::Neck);
    bone_dictionary.insert("head", BoneType::Head);
    bone_dictionary.insert("rEye", BoneType::EyeballRight);
    bone_dictionary.insert("lEye", BoneType::EyeballRight);
    bone_dictionary.insert("upperJaw", BoneType::Jaw);
    bone_dictionary.insert("rCollar", BoneType::CollarRight);
    bone_dictionary.insert("rShldr", BoneType::ShoulderRight);
    bone_dictionary.insert("rForeArm", BoneType::ElbowRight);
    bone_dictionary.insert("rHand", BoneType::HandRight);
    bone_dictionary.insert("rThumb1", BoneType::ThumbRight0);
    bone_dictionary.insert("rThumb2", BoneType::ThumbRight1);
    bone_dictionary.insert("rThumb3", BoneType::ThumbRight2);
    bone_dictionary.insert("rIndex1", BoneType::IndexRight0);
    bone_dictionary.insert("rIndex2", BoneType::IndexRight1);
    bone_dictionary.insert("rIndex3", BoneType::IndexRight2);
    bone_dictionary.insert("rMid1", BoneType::MiddleRight0);
    bone_dictionary.insert("rMid2", BoneType::MiddleRight1);
    bone_dictionary.insert("rMid3", BoneType::MiddleRight2);
    bone_dictionary.insert("rRing1", BoneType::RingRight0);
    bone_dictionary.insert("rRing2", BoneType::RingRight1);
    bone_dictionary.insert("rRing3", BoneType::RingRight2);
    bone_dictionary.insert("rPinky1", BoneType::PinkyRight0);
    bone_dictionary.insert("rPinky2", BoneType::PinkyRight1);
    bone_dictionary.insert("rPinky3", BoneType::PinkyRight2);
    bone_dictionary.insert("lCollar", BoneType::CollarLeft);
    bone_dictionary.insert("lShldr", BoneType::ShoulderLeft);
    bone_dictionary.insert("lForeArm", BoneType::ElbowLeft);
    bone_dictionary.insert("lHand", BoneType::HandLeft);
    bone_dictionary.insert("lThumb1", BoneType::ThumbLeft0);
    bone_dictionary.insert("lThumb2", BoneType::ThumbLeft1);
    bone_dictionary.insert("lThumb3", BoneType::ThumbLeft2);
    bone_dictionary.insert("lIndex1", BoneType::IndexLeft0);
    bone_dictionary.insert("lIndex2", BoneType::IndexLeft1);
    bone_dictionary.insert("lIndex3", BoneType::IndexLeft2);
    bone_dictionary.insert("lMid1", BoneType::MiddleLeft0);
    bone_dictionary.insert("lMid2", BoneType::MiddleLeft1);
    bone_dictionary.insert("lMid3", BoneType::MiddleLeft2);
    bone_dictionary.insert("lRing1", BoneType::RingLeft0);
    bone_dictionary.insert("lRing2", BoneType::RingLeft1);
    bone_dictionary.insert("lRing3", BoneType::RingLeft2);
    bone_dictionary.insert("lPinky1", BoneType::PinkyLeft0);
    bone_dictionary.insert("lPinky2", BoneType::PinkyLeft1);
    bone_dictionary.insert("lPinky3", BoneType::PinkyLeft2);
    bone_dictionary.insert("rShldrBend", BoneType::ShoulderRight);
    bone_dictionary.insert("rForearmBend", BoneType::ElbowRight);
    bone_dictionary.insert("rThighBend", BoneType::HipRight);
    bone_dictionary.insert("lShldrBend", BoneType::ShoulderLeft);
    bone_dictionary.insert("lForearmBend", BoneType::ElbowLeft);
    bone_dictionary.insert("lThighBend", BoneType::HipLeft);
    bone_dictionary.insert("abdomenLower", BoneType::SpineLower);
    bone_dictionary.insert("abdomenUpper", BoneType::SpineMiddle);
    bone_dictionary.insert("chestLower", BoneType::SpineUpper);
    bone_dictionary.insert("neckLower", BoneType::Neck);
    bone_dictionary.insert("Hips", BoneType::Hips);
    bone_dictionary.insert("Chest", BoneType::SpineLower);
    bone_dictionary.insert("Chest2", BoneType::SpineMiddle);
    bone_dictionary.insert("Chest3", BoneType::SpineUpper);
    bone_dictionary.insert("Neck", BoneType::Neck);
    bone_dictionary.insert("Head", BoneType::Head);
    bone_dictionary.insert("LeftCollar", BoneType::CollarLeft);
    bone_dictionary.insert("LeftShoulder", BoneType::ShoulderLeft);
    bone_dictionary.insert("LeftElbow", BoneType::ElbowLeft);
    bone_dictionary.insert("LeftHand", BoneType::HandLeft);
    bone_dictionary.insert("LeftFinger0", BoneType::ThumbLeft0);
    bone_dictionary.insert("LeftFinger01", BoneType::ThumbLeft1);
    bone_dictionary.insert("LeftFinger1", BoneType::IndexLeft0);
    bone_dictionary.insert("LeftFinger11", BoneType::IndexLeft1);
    bone_dictionary.insert("RightCollar", BoneType::CollarRight);
    bone_dictionary.insert("RightShoulder", BoneType::ShoulderRight);
    bone_dictionary.insert("RightElbow", BoneType::ElbowRight);
    bone_dictionary.insert("RightHand", BoneType::HandRight);
    bone_dictionary.insert("RightFinger0", BoneType::ThumbRight0);
    bone_dictionary.insert("RightFinger01", BoneType::ThumbRight1);
    bone_dictionary.insert("RightFinger1", BoneType::IndexRight0);
    bone_dictionary.insert("RightFinger11", BoneType::IndexRight1);
    bone_dictionary.insert("LeftHip", BoneType::HipLeft);
    bone_dictionary.insert("LeftKnee", BoneType::KneeLeft);
    bone_dictionary.insert("LeftAnkle", BoneType::FootLeft);
    bone_dictionary.insert("LeftToe", BoneType::ToeLeft);
    bone_dictionary.insert("RightHip", BoneType::HipRight);
    bone_dictionary.insert("RightKnee", BoneType::KneeRight);
    bone_dictionary.insert("RightAnkle", BoneType::FootRight);
    bone_dictionary.insert("RightToe", BoneType::ToeRight);
    Converter {
      bone_dictionary: bone_dictionary,
    }
  }
}


pub fn bone_type_to_mecanim_name(bone_type: BoneType) -> &'static str {
  match bone_type {
    BoneType::Hips => return "Hips",
    BoneType::HipLeft => return "LeftUpperLeg",
    BoneType::KneeLeft => return "LeftLowerLeg",
    BoneType::FootLeft => return "LeftFoot",
    BoneType::ToeLeft => return "LeftToes",

    BoneType::HipRight => return "RightUpperLeg",
    BoneType::KneeRight => return "RightLowerLeg",
    BoneType::FootRight => return "RightFoot",
    BoneType::ToeRight => return "RightToes",

    BoneType::SpineLower => return "Spine",
    BoneType::SpineMiddle => return "Chest",
    BoneType::SpineUpper => return "UpperChest",

    BoneType::Neck => return "Neck",
    BoneType::Head => return "Head",
    BoneType::Jaw => return "Jaw",

    BoneType::EyeballLeft => return "LeftEye",
    BoneType::EyeballRight => return "RightEye",

    BoneType::CollarLeft => return "LeftShoulder",
    BoneType::ShoulderLeft => return "LeftUpperArm",
    BoneType::ElbowLeft => return "LeftLowerArm",
    BoneType::HandLeft => return "LeftHand",

    BoneType::ThumbLeft0 => return "Left Thumb Proximal",
    BoneType::ThumbLeft1 => return "Left Thumb Intermediate",
    BoneType::ThumbLeft2 => return "Left Thumb Distal",
    BoneType::IndexLeft0 => return "Left Index Proximal",
    BoneType::IndexLeft1 => return "Left Index Intermediate",
    BoneType::IndexLeft2 => return "Left Index Distal",
    BoneType::MiddleLeft0 => return "Left Middle Proximal",
    BoneType::MiddleLeft1 => return "Left Middle Intermediate",
    BoneType::MiddleLeft2 => return "Left Middle Distal",
    BoneType::RingLeft0 => return "Left Ring Proximal",
    BoneType::RingLeft1 => return "Left Ring Intermediate",
    BoneType::RingLeft2 => return "Left Ring Distal",
    BoneType::PinkyLeft0 => return "Left Little Proximal",
    BoneType::PinkyLeft1 => return "Left Little Intermediate",
    BoneType::PinkyLeft2 => return "Left Little Distal",

    BoneType::CollarRight => return "RightShoulder",
    BoneType::ShoulderRight => return "RightUpperArm",
    BoneType::ElbowRight => return "RightLowerArm",
    BoneType::HandRight => return "RightHand",

    BoneType::ThumbRight0 => return "Right Thumb Proximal",
    BoneType::ThumbRight1 => return "Right Thumb Intermediate",
    BoneType::ThumbRight2 => return "Right Thumb Distal",
    BoneType::IndexRight0 => return "Right Index Proximal",
    BoneType::IndexRight1 => return "Right Index Intermediate",
    BoneType::IndexRight2 => return "Right Index Distal",
    BoneType::MiddleRight0 => return "Right Middle Proximal",
    BoneType::MiddleRight1 => return "Right Middle Intermediate",
    BoneType::MiddleRight2 => return "Right Middle Distal",
    BoneType::RingRight0 => return "Right Ring Proximal",
    BoneType::RingRight1 => return "Right Ring Intermediate",
    BoneType::RingRight2 => return "Right Ring Distal",
    BoneType::PinkyRight0 => return "Right Little Proximal",
    BoneType::PinkyRight1 => return "Right Little Intermediate",
    BoneType::PinkyRight2 => return "Right Little Distal",

    _ => return "default",
  }
}

// pub fn mecanim_name_to_bone_type(mecanimName: &str) -> BoneType {
//   match mecanimName {
//     "Hips" => return BoneType::Hips,
//     "LeftUpperLeg" => return BoneType::HipLeft,
//     "LeftLowerLeg" => return BoneType::KneeLeft,
//     "LeftFoot" => return BoneType::FootLeft,
//     "LeftToes" => return BoneType::ToeLeft,
//     "RightUpperLeg" => return BoneType::HipRight,
//     "RightLowerLeg" => return BoneType::KneeRight,
//     "RightFoot" => return BoneType::FootRight,
//     "RightToes" => return BoneType::ToeRight,
//     "Spine" => return BoneType::SpineLower,
//     "Chest" => return BoneType::SpineMiddle,
//     "UpperChest" => return BoneType::SpineUpper,
//     "Neck" => return BoneType::Neck,
//     "Head" => return BoneType::Head,
//     "Jaw" => return BoneType::Jaw,
//     "LeftEye" => return BoneType::EyeballLeft,
//     "RightEye" => return BoneType::EyeballRight,
//     "LeftShoulder" => return BoneType::CollarLeft,
//     "LeftUpperArm" => return BoneType::ShoulderLeft,
//     "LeftLowerArm" => return BoneType::ElbowLeft,
//     "LeftHand" => return BoneType::HandLeft,
//     "Left Thumb Proximal" => return BoneType::ThumbLeft0,
//     "Left Thumb Intermediate" => return BoneType::ThumbLeft1,
//     "Left Thumb Distal" => return BoneType::ThumbLeft2,
//     "Left Index Proximal" => return BoneType::IndexLeft0,
//     "Left Index Intermediate" => return BoneType::IndexLeft1,
//     "Left Index Distal" => return BoneType::IndexLeft2,
//     "Left Middle Proximal" => return BoneType::MiddleLeft0,
//     "Left Middle Intermediate" => return BoneType::MiddleLeft1,
//     "Left Middle Distal" => return BoneType::MiddleLeft2,
//     "Left Ring Proximal" => return BoneType::RingLeft0,
//     "Left Ring Intermediate" => return BoneType::RingLeft1,
//     "Left Ring Distal" => return BoneType::RingLeft2,
//     "Left Little Proximal" => return BoneType::PinkyLeft0,
//     "Left Little Intermediate" => return BoneType::PinkyLeft1,
//     "Left Little Distal" => return BoneType::PinkyLeft2,
//     "RightShoulder" => return BoneType::CollarRight,
//     "RightUpperArm" => return BoneType::ShoulderRight,
//     "RightLowerArm" => return BoneType::ElbowRight,
//     "RightHand" => return BoneType::HandRight,
//     "Right Thumb Proximal" => return BoneType::ThumbRight0,
//     "Right Thumb Intermediate" => return BoneType::ThumbRight1,
//     "Right Thumb Distal" => return BoneType::ThumbRight2,
//     "Right Index Proximal" => return BoneType::IndexRight0,
//     "Right Index Intermediate" => return BoneType::IndexRight1,
//     "Right Index Distal" => return BoneType::IndexRight2,
//     "Right Middle Proximal" => return BoneType::MiddleRight0,
//     "Right Middle Intermediate" => return BoneType::MiddleRight1,
//     "Right Middle Distal" => return BoneType::MiddleRight2,
//     "Right Ring Proximal" => return BoneType::RingRight0,
//     "Right Ring Intermediate" => return BoneType::RingRight1,
//     "Right Ring Distal" => return BoneType::RingRight2,
//     "Right Little Proximal" => return BoneType::PinkyRight0,
//     "Right Little Intermediate" => return BoneType::PinkyRight1,
//     "Right Little Distal" => return BoneType::PinkyRight2,
//     _ => return BoneType::Generic,
//   }
// }
