mod gamedata;
mod vm;
use clap::Parser;
use libmem::*;
use std::{thread, time};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 120)]
    max_fps: u32,

    #[arg(short, long, default_value_t = 120.0)]
    fov: f32,
}

fn main() {
    let args = Args::parse();

    let sekiro: Option<lm_process_t>;

    let delay = time::Duration::from_millis(500);
    //_ = get_pid("sekiro.exe");
    loop {
        match vm::get_pid("sekiro.exe") {
            Some(val) => {
                sekiro = LM_GetProcessEx(val);
                break;
            }
            None => {
                println!("waiting for sekiro.exe ...");
                // let speedfix = gamedata::find_speed_fix_value(args.max_fps);
                //
                // println!(
                //     "speedfix value for max_fps {} is {}",
                //     args.max_fps, speedfix
                // );
            }
        }
        thread::sleep(delay);
    }
    let sekiro = sekiro.unwrap();

    let sekiro_size = vm::get_proc_size(&sekiro);

    let framelock_fuz_address = vm::get_signature_address(
        &sekiro,
        gamedata::patterns::FRAMELOCK_FUZZY,
        gamedata::patterns::FRAMELOCK_OFFSET,
        sekiro_size,
    );

    match framelock_fuz_address {
        Some(val) => {
            println!("framelock fuz sig found: \t0x{:x}", val);
        }
        None => println!("no sig found"),
    };

    let framelock_sf_address = vm::get_signature_address(
        &sekiro,
        gamedata::patterns::FRAMELOCK_SPEED_FIX,
        gamedata::patterns::FRAMELOCK_SPEED_FIX_OFFSET,
        sekiro_size,
    );

    match framelock_sf_address {
        Some(val) => {
            println!("framelock speedfix sig found: \t0x{:x}", val);
        }
        None => println!("no sig found"),
    };

    println!("frame_rate: {}", args.max_fps);
    println!("fov: \t{}", args.fov);

    let speedfix = gamedata::find_speed_fix_value(args.max_fps);

    println!(
        "speedfix value for max_fps {} is {}",
        args.max_fps, speedfix
    );
}
