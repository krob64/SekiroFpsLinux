mod fov;
mod fps_unlock;
mod gamedata;
mod vm;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 120)]
    max_fps: u32,

    #[arg(short, long, default_value_t = 0)]
    fov: i32,
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();
    let sekiro = gamedata::Game::new(gamedata::PROCESS_NAME);

    fps_unlock::patch(&sekiro, args.max_fps)?;

    /*
     * it seems like increasing fov leads to rendering artifacts all over the place
     * i don't know if this behavior is also present on windows
     */
    fov::patch(&sekiro, args.fov)?;

    println!("frame_rate: {}", args.max_fps);
    println!("fov: \t{}", args.fov);
    Ok(())
}
