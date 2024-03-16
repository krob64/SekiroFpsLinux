use crate::gamedata;
use libmem::*;
use std::process::Command;

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
        println!("pid of sekiro: {}", pid);
        return Some(pid);
    }

    return None;
}

pub fn get_proc_size(process: &lm_process_t) -> lm_size_t {
    let mut proc_size: usize = 0;

    let modules = LM_EnumModulesEx(&process);
    for module in modules.unwrap() {
        if module.get_name() == "sekiro" {
            println!("===============================================");
            println!("MODULE_NAME: \t{}", module.get_name());
            println!("MODULE_BASE: \t0x{:x}", module.get_base());
            println!("MODULE_END: \t0x{:x}", module.get_end());
            println!("MODULE_SIZE: \t0x{:x}", module.get_size());
            println!("===============================================");
        }
        proc_size += module.get_size();
    }

    proc_size
}

pub fn get_signature_address(
    proc: &lm_process_t,
    sig: &str,
    offset: i32,
    proc_size: usize,
) -> Option<lm_address_t> {
    let sig_address = LM_SigScanEx(proc, sig, gamedata::IMAGE_BASE, proc_size);

    match sig_address {
        Some(addr) => {
            return sig_address;
        }
        None => None,
    }
}

pub fn find_page_from_addr(
    process: &lm_process_t,
    address: &lm_address_t,
) -> Result<lm_page_t, &'static str> {
    for page in LM_EnumPagesEx(&process).unwrap() {
        let page_base = page.get_base();
        let page_end = page.get_end();
        if page_base <= *address && page_end >= *address {
            return Ok(page);
        }
    }
    Err("page for address {address} could not be found.")
}
