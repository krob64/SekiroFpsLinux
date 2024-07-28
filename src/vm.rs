use crate::gamedata;
use core::time;
use libmem::*;
use log::{error, info, warn};
use std::{process::Command, thread::sleep};

pub fn get_pid(process_name: &str) -> Option<lm_pid_t> {
    let mut input = Command::new("sh");

    input.arg("-c").arg("pgrep -f ".to_owned() + process_name);
    let output = String::from_utf8(input.output().unwrap().stdout).unwrap();
    let output = output.split("\n");
    let pid_vec: Vec<&str> = output.collect();

    if pid_vec.len() <= 2 {
        return None;
    }

    let pid = pid_vec[pid_vec.len() - 2].parse::<u32>().unwrap();

    if pid != 0 {
        info!("pid of sekiro: {}", pid);
        return Some(pid);
    }

    return None;
}

pub fn get_signature_address(proc: &lm_process_t, sig: &str) -> Option<lm_address_t> {
    info!("attempting to get sig for: {}", sig);

    let mut miss_count = 0;
    let sleep_dur = time::Duration::from_millis(100);
    loop {
        match LM_EnumPagesEx(&proc) {
            Some(pages) => {
                for page in pages {
                    let prot = page.get_prot();
                    if page.get_base() < gamedata::IMAGE_BASE {
                        continue;
                    }

                    match prot {
                        LM_PROT_XR | LM_PROT_X | LM_PROT_XW | LM_PROT_XRW => {
                            let sig_address =
                                LM_SigScanEx(&proc, sig, page.get_base(), page.get_size());
                            match sig_address {
                                Some(address) => {
                                    return Some(address);
                                }
                                None => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            None => {
                error!("sig LM_EnumPagesEx() returned nothing.")
            }
        }
        miss_count += 1;
        warn!("addr for sig \"{sig}\" not found. retrying... [{miss_count}]");
        sleep(sleep_dur);
    }
}

pub fn find_page_from_addr(
    process: &lm_process_t,
    address: &lm_address_t,
) -> Result<lm_page_t, &'static str> {
    let mut miss_count = 0;
    let sleep_dur = time::Duration::from_millis(100);
    loop {
        match LM_EnumPagesEx(&process) {
            Some(pages) => {
                for page in pages {
                    let page_base = page.get_base();
                    let page_end = page.get_end();
                    if page_base <= *address && page_end >= *address {
                        return Ok(page);
                    }
                }
            }
            None => {
                error!("find_page LM_EnumPagesEx() returned nothing.")
            }
        }
        miss_count += 1;
        warn!(
            "page for addr 0x{:x} not found, retrying... [{miss_count}]",
            address
        );
        sleep(sleep_dur);
    }
}

pub fn pause_target(process_name: &str) -> Result<(), &'static str> {
    let pid = match get_pid(process_name) {
        Some(pid) => pid,
        None => return Err("failed to pause target."),
    };

    let mut input = Command::new("sh");

    input
        .arg("-c")
        .arg("kill -SIGSTOP ".to_owned() + &pid.to_string());
    let output = String::from_utf8(input.output().unwrap().stdout).unwrap();
    info!("sent SIGSTOP to process.");

    Ok(())
}

pub fn resume_target(process_name: &str) -> Result<(), &'static str> {
    let pid = match get_pid(process_name) {
        Some(pid) => pid,
        None => return Err("failed to pause target."),
    };

    let mut input = Command::new("sh");

    input
        .arg("-c")
        .arg("kill -SIGCONT ".to_owned() + &pid.to_string());
    let output = String::from_utf8(input.output().unwrap().stdout).unwrap();
    info!("sent SIGCONT to process.");

    Ok(())
}
