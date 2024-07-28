use crate::{
    gamedata::{patterns, Game},
    vm,
};
use libmem::*;
use log::info;

pub fn patch(game: &Game, fov: i32) -> Result<(), &'static str> {
    let fov_sig = vm::get_signature_address(&game.get_process(), patterns::FOV).unwrap()
        + patterns::FOV_OFFSET as usize;

    let fov_offset: u32 = LM_ReadMemoryEx(&game.get_process(), fov_sig).unwrap();
    let fov_address: usize = fov_sig + 4 + fov_offset as usize;

    let fov_value: f32 = LM_ReadMemoryEx(&game.get_process(), fov_address).unwrap();
    info!("fov_value: {}", fov_value);

    let mut actual_fov: f32 = (std::f32::consts::PI / 180.0) * ((fov as f32 / 100.0) + 1.0);

    if fov == 0 {
        actual_fov = 0.017453292;
    }

    let page = vm::find_page_from_addr(&game.get_process(), &fov_address)?;

    LM_ProtMemoryEx(
        &game.get_process(),
        page.get_base(),
        page.get_size(),
        LM_PROT_RW,
    )
    .unwrap();

    LM_WriteMemoryEx(&game.get_process(), fov_address, &actual_fov);
    let afov: f32 = LM_ReadMemoryEx(&game.get_process(), fov_address).unwrap();
    info!("fov after write: {afov}");

    LM_ProtMemoryEx(
        &game.get_process(),
        page.get_base(),
        page.get_size(),
        LM_PROT_R,
    );

    Ok(())
}
