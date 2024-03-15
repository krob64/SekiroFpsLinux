use clap::Parser;
use libmem::*;
use std::{process::Command, thread, time};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 120)]
    max_fps: u32,

    #[arg(short, long, default_value_t = 120.0)]
    fov: f32,
}

fn set_framerate(pid: &lm_process_t, module: &Option<lm_module_t>) -> Option<lm_address_t> {
    const PATTERN_FRAMELOCK_FUZZY: &str = "C7 43 ?? ?? ?? ?? ?? 4C 89 AB";
    const PATTERN_FRAMELOCK_FUZZY_OFFSET: i8 = 3;

    match module {
        Some(f_mod) => {
            let framelock_address = LM_SigScanEx(
                pid,
                PATTERN_FRAMELOCK_FUZZY,
                f_mod.get_base(),
                f_mod.get_size(),
            );

            return framelock_address;
        }
        None => {
            return None;
        }
    }
}

fn set_framerate2(pid: &lm_process_t, module: &lm_module_t) -> Option<lm_address_t> {
    let image_base: u64 = 0x140000000;
    const PATTERN_FRAMELOCK_FUZZY: &str = "C7 43 ?? ?? ?? ?? ?? 4C 89 AB";
    const PATTERN_FRAMELOCK: &str = "88 88 3C 4C 89 AB";
    const FRAMELOCK_SPEED_FIX: &str =
        "3 0F 58 ?? 0F C6 ?? 00 0F 51 ?? F3 0F 59 ?? ?? ?? ?? ?? 0F 2F";
    const PATTERN_FRAMELOCK_FUZZY_OFFSET: i8 = 3;

    let framelock_address = LM_SigScanEx(
        pid,
        PATTERN_FRAMELOCK_FUZZY,
        module.get_base(),
        module.get_size(),
    );

    return framelock_address;
}
fn get_pid(process_name: &str) -> Option<lm_pid_t> {
    let mut input = Command::new("sh");

    input.arg("-c").arg("pgrep -f ".to_owned() + process_name);
    let output = String::from_utf8(input.output().unwrap().stdout).unwrap();
    let output = output.split("\n");
    let pid_vec: Vec<&str> = output.collect();

    let pid = pid_vec[pid_vec.len() - 2].parse::<u32>().unwrap();

    if pid != 0 {
        println!("pid of sekiro: {}", pid);
        return Some(pid);
    }

    return None;
}

fn main() {
    let args = Args::parse();

    let sekiro: Option<lm_process_t>;

    let delay = time::Duration::from_millis(500);
    _ = get_pid("sekiro.exe");
    loop {
        let pid = get_pid("sekiro.exe");
        match pid {
            Some(val) => {
                sekiro = LM_GetProcessEx(val);
                break;
            }
            None => println!("waiting for sekiro.exe ..."),
        }
        thread::sleep(delay);
    }
    let sekiro = sekiro.unwrap();

    let sek_modules = LM_EnumModulesEx(&sekiro);

    //let found_module: Option<lm_module_t> = None;
    let mut missed_modules_counter = 0;
    for module in sek_modules.unwrap() {
        let temp_addr = set_framerate2(&sekiro, &module);
        match temp_addr {
            Some(val) => {
                println!(
                    "sig hit in MODULE: {}. address: {:x?}",
                    module.get_name(),
                    val
                );
            }
            None => {
                missed_modules_counter += 1;
            }
        }
    }
    println!("{} modules had no sig hit.", missed_modules_counter);

    //let _fl_address = set_framerate(&sekiro, &found_module);

    println!("frame_rate: {}", args.max_fps);
    println!("fov: \t{}", args.fov);
}
