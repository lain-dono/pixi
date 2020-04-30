pub struct Animation<Item> {
    items: Vec<Item>,
    durations: Vec<f32>,

    looped: bool,
    speed: f32,
    playing: bool,
    current_time: f32,
}

impl<Item> Animation<Item> {
    pub fn new(items: Vec<Item>, durations: Vec<f32>) -> Self {
        assert!(durations.is_empty() || items.len() == durations.len());

        Self {
            items,
            durations,

            looped: true,
            speed: 1.0,
            playing: false,
            current_time: 0.0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// The total number of frames in the Animation.
    ///
    /// This is the same as number of items assigned to the Animation.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// The speed that the Animation will play at.
    ///
    /// Higher is faster, lower is slower.
    pub fn speed(&self) -> f32 {
        self.speed
    }

    /// Sets the speed that the Animation will play at.
    ///
    /// Higher is faster, lower is slower.
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    /// Whether or not the Animation repeats after playing.
    pub fn is_looped(&self) -> bool {
        self.looped
    }

    /// Sets whether or not the Animation repeats after playing.
    pub fn set_looped(&mut self, looped: bool) {
        self.looped = looped;
    }

    /// Indicates if the Animation is currently playing.
    pub fn is_playing(&self) -> bool {
        self.playing
    }

    /// Plays the Animation.
    pub fn play(&mut self) {
        self.playing = true;
    }

    /// Stops the Animation.
    pub fn stop(&mut self) {
        self.playing = false;
    }

    /// Goes to a specific frame and begins playing the Animation.
    pub fn goto_and_play(&mut self, frame: usize) {
        self.current_time = frame as f32;
        self.play();
    }

    /// Stops the Animation and goes to a specific frame.
    pub fn goto_and_stop(&mut self, frame: usize) {
        self.stop();
        self.current_time = frame as f32;
    }

    /// Current frame index.
    pub fn current_frame(&self) -> usize {
        let count = self.items.len() as isize;
        let frame = self.current_time.floor() as isize;
        frame.rem_euclid(count) as usize
    }

    /// Updates the object transform for rendering.
    pub fn update(&mut self, dt: f32) -> Option<(usize, bool)> {
        let elapsed = self.speed * dt;
        let prev = self.current_frame();

        if !self.durations.is_empty() {
            let mut lag = self.current_time % 1.0 * self.durations[self.current_frame()];
            lag += elapsed / 60.0 * 1000.0;

            while lag < 0.0 {
                self.current_time -= 1.0;
                lag += self.durations[self.current_frame()];
            }

            self.current_time = self.current_time.floor();

            let sign = elapsed.signum();
            while lag >= self.durations[self.current_frame()] {
                lag -= self.durations[self.current_frame()] * sign;
                self.current_time += sign;
            }

            self.current_time += lag / self.durations[self.current_frame()];
        } else {
            self.current_time += elapsed;
        }

        let frame = self.current_frame();
        if self.current_time < 0.0 && !self.looped {
            self.playing = false;
            self.current_time = 0.0;
            Some((0, true))
        } else if self.current_time >= self.items.len() as f32 && !self.looped {
            let frame = self.items.len() - 1;
            self.playing = false;
            self.current_time = frame as f32;
            Some((frame, true))
        } else if prev != frame {
            let a = self.speed > 0.0 && frame < prev;
            let b = self.speed < 0.0 && frame > prev;
            Some((frame, self.looped && (a || b)))
        } else {
            None
        }
    }
}
