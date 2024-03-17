mod fps_unlock;
mod gamedata;
mod vm;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 120)]
    max_fps: u32,

    #[arg(short, long, default_value_t = 120.0)]
    fov: f32,
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();
    let sekiro = gamedata::Game::new(gamedata::PROCESS_NAME);

    fps_unlock::patch(&sekiro, args.max_fps)?;

    println!("frame_rate: {}", args.max_fps);
    println!("fov: \t{}", args.fov);
    Ok(())
}
