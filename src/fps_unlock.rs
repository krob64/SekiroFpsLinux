use crate::gamedata::{self, patterns, Game};
use crate::vm;
use libmem::*;
use log::info;

pub fn patch(game: &Game, fps: u32) -> Result<(), &'static str> {
    vm::pause_target(gamedata::PROCESS_NAME)?;
    patch_framelock(&game, fps)?;
    patch_framespeed(&game, fps)?;
    vm::resume_target(gamedata::PROCESS_NAME)?;

    Ok(())
}

fn patch_framelock(game: &Game, fps: u32) -> Result<(), &'static str> {
    let framelock_sig =
        match vm::get_signature_address(&game.get_process(), patterns::FRAMELOCK_FUZZY) {
            Some(sig) => sig + patterns::FRAMELOCK_OFFSET as usize,
            None => {
                return Err("Error finding framelock signature.");
            }
        };

    info!("found framelock sig at 0x{:X}", framelock_sig);

    let deltatime = 1000.0 / fps as f32 / 1000.0;

    let framelock_page = vm::find_page_from_addr(&game.get_process(), &framelock_sig)?;

    info!("framelock page: {}", framelock_page);

    let old_protect = match LM_ProtMemoryEx(
        &game.get_process(),
        framelock_page.get_base(),
        framelock_page.get_size(),
        LM_PROT_XRW,
    ) {
        Some(protect) => protect,
        None => {
            return Err("framelock mprotect failed.");
        }
    };

    info!("framelock old_protect: {}", old_protect);

    let prepatch_value: f32 = LM_ReadMemoryEx(&game.get_process(), framelock_sig).unwrap();
    info!("prepatch framelock: {}", prepatch_value);

    LM_WriteMemoryEx(&game.get_process(), framelock_sig, &deltatime);

    let postpatch_value: f32 = LM_ReadMemoryEx(&game.get_process(), framelock_sig).unwrap();
    info!("postpatch framelock: {}", postpatch_value);

    LM_ProtMemoryEx(
        &game.get_process(),
        framelock_page.get_base(),
        framelock_page.get_size(),
        old_protect,
    );

    let framelock_page = vm::find_page_from_addr(&game.get_process(), &framelock_sig)?;

    info!(
        "framelock pageprot after write: {}",
        framelock_page.get_prot()
    );

    Ok(())
}

fn patch_framespeed(game: &Game, fps: u32) -> Result<(), &'static str> {
    let framespeed_base_addr =
        match vm::get_signature_address(&game.get_process(), patterns::FRAMELOCK_SPEED_FIX) {
            Some(address) => address + patterns::FRAMELOCK_SPEED_FIX_OFFSET as usize,
            None => {
                return Err("Error finding framespeed signature.");
            }
        };

    info!("found framespeed sig at 0x{:X}", framespeed_base_addr);

    //

    let speedfix = gamedata::find_speed_fix_value(fps);

    let framespeed_offset: u32 =
        LM_ReadMemoryEx(&game.get_process(), framespeed_base_addr).unwrap();

    info!("framespeed offset: 0x{:X}", framespeed_offset);

    let framespeed_address: lm_address_t = framespeed_base_addr + 4 + framespeed_offset as usize;

    let page = vm::find_page_from_addr(&game.get_process(), &framespeed_address)?;

    info!("framespeed page: {}", page);
    info!("framespeed address: 0x{:X}", framespeed_address);

    let old_protect = match LM_ProtMemoryEx(
        &game.get_process(),
        page.get_base(),
        page.get_size(),
        LM_PROT_RW,
    ) {
        Some(protect) => protect,
        None => {
            return Err("framespeed mprotect failed");
        }
    };

    info!("framespeed old_protect: {}", old_protect);

    let value: f32 = LM_ReadMemoryEx(&game.get_process(), framespeed_address).unwrap();
    info!("framespeed value: {}", value);
    LM_WriteMemoryEx(&game.get_process(), framespeed_address, &speedfix);

    let postpatch_value: f32 = LM_ReadMemoryEx(&game.get_process(), framespeed_address).unwrap();
    info!("postpatch framespeed: {}", postpatch_value);

    LM_ProtMemoryEx(
        &game.get_process(),
        page.get_base(),
        page.get_size(),
        old_protect,
    );

    let page = vm::find_page_from_addr(&game.get_process(), &framespeed_address)?;

    info!("framespeed pageprot after write: {}", page.get_prot());

    Ok(())
}
