mod fps_unlock;
mod gamedata;
mod vm;
use clap::Parser;
use gamedata::patterns;
use libmem::*;
use std::{thread, time};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 120)]
    max_fps: u32,

    #[arg(short, long, default_value_t = 120.0)]
    fov: f32,
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    // let sekiro: Option<lm_process_t>;
    //
    // let delay = time::Duration::from_millis(500);
    // //_ = get_pid("sekiro.exe");
    // loop {
    //     match vm::get_pid("sekiro.exe") {
    //         Some(val) => {
    //             sekiro = LM_GetProcessEx(val);
    //             break;
    //         }
    //         None => {
    //             println!("waiting for sekiro.exe ...");
    //         }
    //     }
    //     thread::sleep(delay);
    // }
    // let sekiro = sekiro.unwrap();
    //
    // let sekiro_size = vm::get_proc_size(&sekiro);

    let sekiro = gamedata::Game::new(gamedata::PROCESS_NAME);

    let framelock_fuz_address = vm::get_signature_address(
        &sekiro.get_process(),
        gamedata::patterns::FRAMELOCK_FUZZY,
        gamedata::patterns::FRAMELOCK_OFFSET,
        sekiro.get_size(),
    );

    match framelock_fuz_address {
        Some(mut val) => {
            val = val + 3;
            println!("framelock fuz sig found: \t0x{:x}", val - 3);

            let prepatch: f32 = LM_ReadMemoryEx(&sekiro.get_process(), val).unwrap();
            println!("deltatime value before patch: {}", prepatch);

            let deltatime = 1000.0 / args.max_fps as f32 / 1000.0;

            let framelock_page = vm::find_page_from_addr(&sekiro.get_process(), &val)?;

            let old_protect = LM_ProtMemoryEx(
                &sekiro.get_process(),
                framelock_page.get_base(),
                framelock_page.get_size(),
                lm_prot_t::LM_PROT_XRW,
            );

            match old_protect {
                Some(val) => {
                    println!("page protection before write: {}", val);
                }
                None => {
                    println!("protect failed.");
                }
            }
            LM_WriteMemoryEx(&sekiro.get_process(), val, &deltatime);
            let prot_patch = LM_ProtMemoryEx(
                &sekiro.get_process(),
                framelock_page.get_base(),
                framelock_page.get_size(),
                old_protect.unwrap(),
            )
            .unwrap();

            println!("page protection after write: {}", prot_patch);

            let postpatch: f32 = LM_ReadMemoryEx(&sekiro.get_process(), val).unwrap();
            println!("deltatime value after patch: {}", postpatch);
        }
        None => println!("no sig found"),
    };

    let framelock_sf_address = vm::get_signature_address(
        &sekiro.get_process(),
        gamedata::patterns::FRAMELOCK_SPEED_FIX,
        gamedata::patterns::FRAMELOCK_SPEED_FIX_OFFSET,
        sekiro.get_size(),
    );

    match framelock_sf_address {
        Some(val) => {
            println!("framelock speedfix sig found: \t0x{:x}", val);

            let speedfix_offset: u32 = LM_ReadMemoryEx(
                &sekiro.get_process(),
                val + patterns::FRAMELOCK_SPEED_FIX_OFFSET as usize,
            )
            .unwrap();
            println!("speedfix_offset: 0x{:x}", speedfix_offset);

            let speedfix_address: lm_address_t = (val + 19) + speedfix_offset as usize;
            println!("speedfix_address: {:x}", speedfix_address);

            let testval: f32 = LM_ReadMemoryEx(&sekiro.get_process(), speedfix_address).unwrap();
            println!("speedfix_value: {}", testval);
        }
        None => println!("no sig found"),
    };

    println!("frame_rate: {}", args.max_fps);
    println!("fov: \t{}", args.fov);
    Ok(())
}
