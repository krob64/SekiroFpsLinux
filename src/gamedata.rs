#![allow(dead_code)]

pub const IMAGE_BASE: usize = 0x140000000;
pub const PROCESS_NAME: &str = "sekiro.exe";

pub mod patterns {
    pub const FRAMELOCK_FUZZY: &str = "C7 43 ?? ?? ?? ?? ?? 4C 89 AB";
    pub const FRAMELOCK_OFFSET: i32 = 3;

    pub const FRAMELOCK_SPEED_FIX: &str =
        "F3 0F 58 ?? 0F C6 ?? 00 0F 51 ?? F3 0F 59 ?? ?? ?? ?? ?? 0F 2F";
    pub const FRAMELOCK_SPEED_FIX_OFFSET: i32 = 15;

    pub const FOV: &str = "";
    pub const FOV_OFFSET: i32 = 8;

    pub const CAMADJUST_PITCH: &str = "";
    pub const CAMADJUST_PITCH_OVERWRITE_LENGTH: i32 = 7;

    pub const CAMADJUST_YAW_Z: &str = "";
    pub const CAMADJUST_YAW_Z_OFFSET: i32 = 5;
    pub const CAMADJUST_YAW_Z_OVERWRITE_LENGTH: i32 = 8;

    pub const CAMADJUST_PITCH_XY: &str = "";
    pub const CAMADJUST_PITCH_XY_OVERWRITE_LENGTH: i32 = 12;

    pub const CAMADJUST_YAW_XY: &str = "";
    pub const CAMADJUST_YAW_XY_OFFSET: i32 = 5;
    pub const CAMADJUST_YAW_XY_OVERWRITE: i32 = 8;

    pub const CAMRESET_LOCKON: &str = "";
    pub const CAMRESET_LOCKON_OFFSET: i32 = 6;

    pub const AUTOLOOT: &str = "";
    pub const AUTOLOOT_OFFSET: i32 = 18;

    pub const DRAGONROT_EFFECT: &str = "";
    pub const DRAGONROT_EFFECT_OFFSET: i32 = 13;
}

const FRAMELOCK_SPEED_FIX_MATRIX: [f32; 35] = [
    15.0, 16.0, 16.6667, 18.0, 18.6875, 18.8516, 20.0, 24.0, 25.0, 27.5, 30.0, 32.0, 38.5, 40.0,
    48.0, 49.5, 50.0, 57.2958, 60.0, 64.0, 66.75, 67.0, 78.8438, 80.0, 84.0, 90.0, 93.8, 100.0,
    120.0, 127.0, 128.0, 130.0, 140.0, 144.0, 150.0,
];

const FRAMELOCK_DEFAULT_VALUE: f32 = 30.0;

pub fn find_speed_fix_value(frame_limit: u32) -> f32 {
    let ideal_speed_fix: f32 = frame_limit as f32 / 2.0;
    let mut closest_speed_value = FRAMELOCK_DEFAULT_VALUE;

    for speedfix in FRAMELOCK_SPEED_FIX_MATRIX {
        if f32::abs(ideal_speed_fix - speedfix) <= f32::abs(ideal_speed_fix - closest_speed_value) {
            closest_speed_value = speedfix;
        }
    }
    closest_speed_value
}
