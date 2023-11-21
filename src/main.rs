pub(crate) mod snow_particle;
use crossterm::{
    cursor,
    style::Print,
    terminal, ExecutableCommand, QueueableCommand,
};
use rand::Rng;
use rand::{rngs::ThreadRng, thread_rng};
use std::io::{self, Write, Stdout};
use std::{env, thread, time};
use ctrlc;


fn init_particles(
    particles: &mut Vec<snow_particle::snow_particle::Particle>,
    amount: i32,
    max_x: u16,
    max_y: u16,
    colorful: bool,
    rng: &mut ThreadRng,
    sausage: bool
) {
    let sausage_idx = rng.gen_range(0..amount);
    for i in 0..amount {
        particles.push(snow_particle::snow_particle::Particle::new(
            rng.gen_range(0..max_x).into(),
            rng.gen_range(0..max_y).into(),
            colorful,
            i == sausage_idx && sausage,
            rng
        ))
    }
}

fn print_particles(stdout: &mut Stdout, particles: &Vec<snow_particle::snow_particle::Particle>, debug_info: bool) -> io::Result<()> {
    let mut i = 0;
    for particle in particles {
        particle.print(stdout)?;
        if debug_info {
            stdout.queue(cursor::MoveTo(1, i))?
                .queue(Print(format!("X: {}, Y: {}", particle.x, particle.y).as_str()))?;
        }
        i += 1;
    }
    stdout.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut delay: time::Duration = time::Duration::from_millis(30);
    let mut particles: Vec<snow_particle::snow_particle::Particle> = Vec::new();
    let max_x: u16;
    let max_y: u16;
    let mut rng = thread_rng();
    let args: Vec<String> = env::args().collect();
    const DEFAULT_PARTICLE_AMOUNT: i32 = 300;
    let mut particle_amount = DEFAULT_PARTICLE_AMOUNT;
    
    if args.contains(&"storm".to_owned()) {
        particle_amount = 1000;
    }
    if args.contains(&"fast".to_owned()) {
        delay = time::Duration::from_millis(20);
    }
    let debug = args.contains(&"debug".to_owned());    
    let colorful = args.contains(&"colorful".to_owned()) | args.contains(&"color".to_owned()) | args.contains(&"colour".to_owned()) | args.contains(&"colourful".to_owned());
    let sausage = args.contains(&"sausage".into());
    
    stdout.execute(cursor::Hide)?;    
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    (max_x, max_y) = terminal::size()?;

    print!("x:{} ", max_x);
    print!("y: {} ", max_y);

    init_particles(&mut particles, particle_amount, max_x, max_y, colorful, &mut rng, sausage);
    print_particles(&mut stdout, &particles, debug)?;
    stdout.flush()?;
        
    ctrlc::set_handler(move || {
        let mut stdout = io::stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout.execute(cursor::Show).unwrap();
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        std::process::exit(0);
        }
        ).unwrap();

    loop {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        print_particles(&mut stdout, &particles, debug)?;
        for particle in &mut particles {
            particle.float();
            if particle.y > max_y {
                particle.respawn(rng.gen_range(0..max_x), 0);
            }
        }
        thread::sleep(delay);
    }
}
