#![allow(dead_code)]
use std::{thread::sleep, time};

use libmem::*;

use crate::vm;

pub const IMAGE_BASE: usize = 0x140000000;
pub const PROCESS_NAME: &str = "sekiro.exe";

pub mod patterns {

    /**
       <float>fFrameTick determines default frame rate limit in seconds.
                                  these bytes represent the games deltatime and therefore need to be overwritten by our own deltatime
                                  vvvvvvvv
       0000000141161FCD | C743 18 8988883C             | mov dword ptr ds:[rbx+18],3C888889                    | fFrameTick
       0000000141161FD4 | 4C:89AB 70020000             | mov qword ptr ds:[rbx+270],r13                        |

       0000000141161694 (Version 1.2.0.0)
    */
    pub const FRAMELOCK_FUZZY: &str = "C7 43 ?? ?? ?? ?? ?? 4C 89 AB";
    pub const FRAMELOCK_OFFSET: i32 = 3;

    /**
       Reference pointer pFrametimeRunningSpeed to speed table entry that gets used in calculations.
       Add or remove multiplications of 4bytes to pFrametimeRunningSpeed address to use a higher or lower <float>fFrametimeCriticalRunningSpeed from table.
       fFrametimeCriticalRunningSpeed should be roughly half the frame rate: 30 @ 60FPS limit, 50 @ 100FPS limit...
       00000001407D4F3D | F3:0F58D0                    | addss xmm2,xmm0                                       |
       00000001407D4F41 | 0FC6D2 00                    | shufps xmm2,xmm2,0                                    |
       00000001407D4F45 | 0F51C2                       | sqrtps xmm0,xmm2                                      |
                                                                                  this address is the result of [rip+029240E8]
                                                                                  vvvvvvvvv
       00000001407D4F48 | F3:0F5905 E8409202           | mulss xmm0,dword ptr ds:[1430F9038]                   | pFrametimeRunningSpeed->fFrametimeCriticalRunningSpeed
       00000001407D4F50 | 0F2FF8                       | comiss xmm7,xmm0                                      |

       00000001407D4E08 (Version 1.2.0.0)
    */
    pub const FRAMELOCK_SPEED_FIX: &str =
        "F3 0F 58 ?? 0F C6 ?? 00 0F 51 ?? F3 0F 59 ?? ?? ?? ?? ?? 0F 2F";
    pub const FRAMELOCK_SPEED_FIX_OFFSET: i32 = 15;

    pub const FOV: &str = "F3 0F 10 08 F3 0F 59 0D ?? ?? ?? ?? F3 0F 5C 4E";
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

pub struct Game {
    process: lm_process_t,
}

impl Game {
    pub fn new(process_name: &str) -> Game {
        let game = Game {
            process: find_process(process_name),
        };

        game
    }

    pub fn get_process(&self) -> lm_process_t {
        self.process
    }
}

fn find_process(process_name: &str) -> lm_process_t {
    let delay = time::Duration::from_millis(500);

    loop {
        match vm::get_pid(process_name) {
            Some(val) => {
                return LM_GetProcessEx(val).unwrap();
            }
            None => {
                println!("waiting for sekiro.exe...");
            }
        };

        sleep(delay);
    }
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
