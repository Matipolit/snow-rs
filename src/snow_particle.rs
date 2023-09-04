pub mod snow_particle{
    use rand::{thread_rng, Rng, rngs::ThreadRng};
    use std::io::{self, Stdout};
        use crossterm::{
        cursor,
        style::{Stylize, PrintStyledContent, Color, StyledContent},QueueableCommand,
    };
    
    static COLORS: &'static [Color] = &[Color::White, Color::Black, Color::Blue, Color::Magenta, Color::Green, Color::Yellow];

    pub struct Particle {
        pub x: u16,
        pub y: u16,
        time_since_last_swing: u16,
        last_swing_direction: bool,
        swing_time: u16,
        rng: ThreadRng,
        content: StyledContent<&'static str>
    }

    impl Particle {
        fn get_rand_swing_time(&mut self) -> u16{
            return self.rng.gen_range(10..20);
        }
        pub fn new(beg_x: u16, beg_y: u16, colorful: bool, is_sausage: bool, rng: &mut ThreadRng) -> Particle {
            let color: Color;
            if !colorful{
                color = Color::White;
            }else{
                color = COLORS[rng.gen_range(0..COLORS.len())];
            }
            let content: StyledContent<&'static str>;
            if is_sausage{
                content = "🌭".stylize();
            }else{
                content = "*".with(color);
            }
            
            let dir = rng.gen_range(0..=1)==0;
            let swing_time = rng.gen_range(10..20);
            
            Particle{x: beg_x, y: beg_y, time_since_last_swing: 0,
                last_swing_direction: dir, swing_time, rng: thread_rng(), content }
        }
        
        pub fn float(&mut self) {
            self.y+=1;
            if self.last_swing_direction{
                self.x += 1;
            }else{
                if self.x > 0{
                    self.x -=1;
                }else{
                    self.x = 0;
                }

            }
            self.time_since_last_swing+=1;
            if self.time_since_last_swing > self.swing_time {
                self.last_swing_direction = !self.last_swing_direction;
                self.time_since_last_swing = 0;
                self.swing_time = self.get_rand_swing_time();
            }
        }
        
        pub fn respawn(&mut self, beg_x: u16, beg_y: u16) {
            self.x = beg_x.into();
            self.y = beg_y.into();
            self.time_since_last_swing = 0;
        }
        
        pub fn print(& self,stdout: &mut Stdout) -> io::Result<()> {
            stdout.queue(cursor::MoveTo(self.x, self.y))?
                .queue(PrintStyledContent(self.content))?;
            Ok(())
        }
    }
}
