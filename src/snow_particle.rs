pub mod snow_particle {
    use crossterm::{
        QueueableCommand, cursor,
        style::{Color, PrintStyledContent, StyledContent, Stylize},
    };
    use rand::{Rng, rngs::ThreadRng};
    use std::io::{self, StdoutLock};

    static COLORS: &'static [Color] = &[
        Color::White,
        Color::Black,
        Color::Blue,
        Color::Magenta,
        Color::Green,
        Color::Yellow,
    ];

    pub struct Particle {
        pub x: u16,
        pub y: u16,
        time_since_last_swing: u16,
        last_swing_direction: bool,
        swing_time: u16,
        content: StyledContent<&'static str>,
    }

    impl Particle {
        fn get_rand_swing_time(rng: &mut ThreadRng) -> u16 {
            rng.random_range(10..20)
        }

        pub fn new(
            beg_x: u16,
            beg_y: u16,
            colorful: bool,
            is_sausage: bool,
            rng: &mut ThreadRng,
        ) -> Particle {
            let color = if !colorful {
                Color::White
            } else {
                // use gen_range instead of random_range
                COLORS[rng.random_range(0..COLORS.len())]
            };
            let content = if is_sausage {
                "🌭".stylize()
            } else {
                "*".with(color)
            };

            let dir = rng.random_bool(0.5);
            let swing_time = Self::get_rand_swing_time(rng);

            Particle {
                x: beg_x,
                y: beg_y,
                time_since_last_swing: 0,
                last_swing_direction: dir,
                swing_time,
                content,
            }
        }

        pub fn float(&mut self, rng: &mut ThreadRng) {
            self.y += 1;
            if self.last_swing_direction {
                self.x = self.x.saturating_add(1);
            } else {
                self.x = self.x.saturating_sub(1);
            }
            self.time_since_last_swing += 1;
            if self.time_since_last_swing > self.swing_time {
                self.last_swing_direction = !self.last_swing_direction;
                self.time_since_last_swing = 0;
                self.swing_time = Particle::get_rand_swing_time(rng);
            }
        }

        pub fn respawn(&mut self, beg_x: u16, beg_y: u16, rng: &mut ThreadRng) {
            self.x = beg_x.into();
            self.y = beg_y.into();
            self.time_since_last_swing = 0;
            self.swing_time = Particle::get_rand_swing_time(rng);
        }

        pub fn print(&self, handle: &mut StdoutLock) -> io::Result<()> {
            handle
                .queue(cursor::MoveTo(self.x, self.y))?
                .queue(PrintStyledContent(self.content))?;
            Ok(())
        }
    }
}
