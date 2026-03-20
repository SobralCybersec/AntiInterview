use eframe::egui;
use tracing::{error, info};

pub struct VideoInfo {
    pub fps: f64,
}

pub fn load_embedded_frames(ctx: &egui::Context, gif_data: &[u8]) -> (Vec<egui::TextureHandle>, VideoInfo) {
    use image::AnimationDecoder;
    use std::io::Cursor;
    
    match image::codecs::gif::GifDecoder::new(Cursor::new(gif_data)) {
        Ok(decoder) => {
            let mut textures = Vec::new();
            let mut total_delay = 0u32;
            let frames = decoder.into_frames();
            
            for (i, frame_result) in frames.enumerate() {
                match frame_result {
                    Ok(frame) => {
                        let delay = frame.delay();
                        total_delay += delay.numer_denom_ms().0;
                        
                        let img = frame.into_buffer();
                        let size = [img.width() as usize, img.height() as usize];
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            size,
                            img.as_raw(),
                        );
                        
                        textures.push(ctx.load_texture(
                            format!("video_frame_{}", i),
                            color_image,
                            egui::TextureOptions::LINEAR,
                        ));
                    }
                    Err(_) => break,
                }
            }
            
            let frame_count = textures.len();
            let fps = if total_delay > 0 && frame_count > 0 {
                (frame_count as f64 * 1000.0) / total_delay as f64
            } else {
                10.0
            };
            
            info!("Embedded GIF loaded: {} frames at {:.2} FPS", frame_count, fps);
            (textures, VideoInfo { fps })
        }
        Err(e) => {
            error!("Failed to decode embedded GIF: {:?}", e);
            (Vec::new(), VideoInfo { fps: 10.0 })
        }
    }
}
