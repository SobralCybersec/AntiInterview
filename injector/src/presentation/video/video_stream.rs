use eframe::egui;

pub struct VideoStream {
    frames: Vec<egui::TextureHandle>,
    current_frame: usize,
    last_update: std::time::Instant,
    frame_delay: std::time::Duration,
    looping: bool,
}

impl VideoStream {
    pub fn new(frames: Vec<egui::TextureHandle>, fps: u32) -> Self {
        Self {
            frames,
            current_frame: 0,
            last_update: std::time::Instant::now(),
            frame_delay: std::time::Duration::from_secs_f64(1.0 / fps as f64),
            looping: true,
        }
    }

    pub fn current_frame(&mut self) -> Option<&egui::TextureHandle> {
        if self.frames.is_empty() {
            return None;
        }

        let now = std::time::Instant::now();
        if now.duration_since(self.last_update) >= self.frame_delay {
            self.current_frame = if self.looping {
                (self.current_frame + 1) % self.frames.len()
            } else {
                (self.current_frame + 1).min(self.frames.len() - 1)
            };
            self.last_update = now;
        }

        self.frames.get(self.current_frame)
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    pub fn time_until_next_frame(&self) -> std::time::Duration {
        self.frame_delay.saturating_sub(self.last_update.elapsed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repaint_delay_does_not_exceed_frame_delay() {
        let stream = VideoStream::new(Vec::new(), 10);

        assert!(stream.time_until_next_frame() <= std::time::Duration::from_millis(100));
    }
}
