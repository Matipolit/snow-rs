pub(crate) mod snow_particle;
use crossterm::{ExecutableCommand, QueueableCommand, cursor, style::Print, terminal};
use ctrlc;
use rand::rngs::ThreadRng;
use rand::{Rng, rng};
use snow_particle::snow_particle::Particle;
use std::io::{self, Stdout, Write};
use std::{env, thread, time};

fn init_particles(
    particles: &mut Vec<snow_particle::snow_particle::Particle>,
    amount: u16,
    max_x: u16,
    max_y: u16,
    colorful: bool,
    rng: &mut ThreadRng,
    sausage: bool,
) {
    let sausage_idx = rng.random_range(0..amount);
    for i in 0..amount {
        particles.push(snow_particle::snow_particle::Particle::new(
            rng.random_range(0..max_x).into(),
            rng.random_range(0..max_y).into(),
            colorful,
            i == sausage_idx && sausage,
            rng,
        ))
    }
}

fn print_particles(
    stdout: &mut Stdout,
    particles: &Vec<snow_particle::snow_particle::Particle>,
    debug_info: bool,
) -> io::Result<()> {
    let mut handle = stdout.lock();
    for (i, particle) in particles.iter().enumerate() {
        particle.print(&mut handle)?;
        if debug_info {
            handle.queue(cursor::MoveTo(1, i as u16))?.queue(Print(
                format!("X: {}, Y: {}", particle.x, particle.y).as_str(),
            ))?;
        }
    }
    handle.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut delay: time::Duration = time::Duration::from_millis(30);
    let max_x: u16;
    let max_y: u16;
    let mut rng = rng();
    let args: Vec<String> = env::args().collect();
    const DEFAULT_PARTICLE_AMOUNT: u16 = 300;
    let mut particle_amount = DEFAULT_PARTICLE_AMOUNT;
    let mut particles: Vec<Particle> = Vec::with_capacity(particle_amount as usize);

    if args.contains(&"storm".to_owned()) {
        particle_amount = 1000;
    }
    if args.contains(&"fast".to_owned()) {
        delay = time::Duration::from_millis(20);
    }
    let debug = args.contains(&"debug".to_owned());
    let colorful = args.contains(&"colorful".to_owned())
        | args.contains(&"color".to_owned())
        | args.contains(&"colour".to_owned())
        | args.contains(&"colourful".to_owned());
    let sausage = args.contains(&"sausage".into());
    // for accurate performance measurement
    let no_draw = args.iter().any(|s| s == "benchmark");

    {
        let mut handle = stdout.lock();
        handle.execute(cursor::Hide)?;
        handle.execute(terminal::Clear(terminal::ClearType::All))?;
    }
    (max_x, max_y) = terminal::size()?;

    print!("x:{} ", max_x);
    print!("y: {} ", max_y);

    init_particles(
        &mut particles,
        particle_amount,
        max_x,
        max_y,
        colorful,
        &mut rng,
        sausage,
    );

    ctrlc::set_handler(move || {
        let mut stdout = io::stdout();
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        stdout.execute(cursor::Show).unwrap();
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        std::process::exit(0);
    })
    .unwrap();

    loop {
        if !no_draw {
            stdout.execute(terminal::Clear(terminal::ClearType::All))?;
            print_particles(&mut stdout, &particles, debug)?;
        }
        for particle in &mut particles {
            particle.float(&mut rng);
            if particle.y > max_y {
                let new_x = rng.random_range(0..max_x);
                particle.respawn(new_x, 0, &mut rng);
            }
        }
        thread::sleep(delay);
    }
}
