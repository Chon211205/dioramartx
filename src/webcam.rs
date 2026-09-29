use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;

use crate::core::vec3::Vec3;

const FALLBACK_COLOR: Vec3 = Vec3 { x: 0.10, y: 0.12, z: 0.15 };

pub struct WebcamFrame {
    width: usize,
    height: usize,
    data: Vec<Vec3>,
}

impl WebcamFrame {
    fn fallback() -> Self {
        Self { width: 1, height: 1, data: vec![FALLBACK_COLOR] }
    }

    fn sample(&self, u: f32, v: f32) -> Vec3 {
        if self.width == 0 || self.height == 0 || self.data.is_empty() {
            return FALLBACK_COLOR;
        }

        // Center-crop the camera image to the mirror's portrait aspect ratio.
        let source_aspect = self.width as f32 / self.height as f32;
        let target_aspect = 9.0 / 16.0;
        let (crop_u, crop_v) = if source_aspect > target_aspect {
            let visible = target_aspect / source_aspect;
            (0.5 + (u.clamp(0.0, 1.0) - 0.5) * visible, v.clamp(0.0, 1.0))
        } else {
            let visible = source_aspect / target_aspect;
            (u.clamp(0.0, 1.0), 0.5 + (v.clamp(0.0, 1.0) - 0.5) * visible)
        };

        // Horizontal flip gives the familiar front-camera mirror behavior.
        let x = ((1.0 - crop_u) * (self.width.saturating_sub(1)) as f32) as usize;
        let y = ((1.0 - crop_v) * (self.height.saturating_sub(1)) as f32) as usize;
        self.data[y * self.width + x]
    }
}

fn shared_frame() -> &'static Arc<RwLock<WebcamFrame>> {
    static FRAME: OnceLock<Arc<RwLock<WebcamFrame>>> = OnceLock::new();
    FRAME.get_or_init(|| Arc::new(RwLock::new(WebcamFrame::fallback())))
}

pub fn sample_webcam(u: f32, v: f32) -> Vec3 {
    shared_frame()
        .read()
        .map(|frame| frame.sample(u, v))
        .unwrap_or(FALLBACK_COLOR)
}

pub struct WebcamCapture {
    stop: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}

impl WebcamCapture {
    pub fn start() -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop);
        let frame = Arc::clone(shared_frame());
        let worker = thread::spawn(move || {
            let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
            let mut camera = match Camera::new(CameraIndex::Index(0), format) {
                Ok(camera) => camera,
                Err(error) => {
                    eprintln!("Warning: no se pudo abrir la webcam: {error}");
                    return;
                }
            };

            if let Err(error) = camera.open_stream() {
                eprintln!("Warning: no se pudo iniciar la webcam: {error}");
                return;
            }

            while !worker_stop.load(Ordering::Relaxed) {
                match camera.frame().and_then(|buffer| buffer.decode_image::<RgbFormat>()) {
                    Ok(image) => {
                        let width = image.width() as usize;
                        let height = image.height() as usize;
                        let data = image
                            .as_raw()
                            .chunks_exact(3)
                            .map(|pixel| Vec3::new(
                                pixel[0] as f32 / 255.0,
                                pixel[1] as f32 / 255.0,
                                pixel[2] as f32 / 255.0,
                            ))
                            .collect();
                        if let Ok(mut destination) = frame.write() {
                            *destination = WebcamFrame { width, height, data };
                        }
                    }
                    Err(error) => {
                        eprintln!("Warning: error capturando webcam: {error}");
                        thread::sleep(Duration::from_millis(100));
                    }
                }
                thread::sleep(Duration::from_millis(33));
            }
            let _ = camera.stop_stream();
        });

        Self { stop, worker: Some(worker) }
    }
}

impl Drop for WebcamCapture {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
        if let Ok(mut frame) = shared_frame().write() {
            *frame = WebcamFrame::fallback();
        }
    }
}
