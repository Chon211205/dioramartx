use std::ffi::CString;
use std::slice;

use raylib::ffi;

use crate::core::vec3::Vec3;

pub struct TextureMap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec3>,
}

impl TextureMap {
    pub fn from_file(
        path: &str,
    ) -> Self {
        let c_path =
            CString::new(path)
                .expect(
                    "Ruta invalida para textura",
                );

        unsafe {
            let image =
                ffi::LoadImage(
                    c_path.as_ptr(),
                );

            if image.data.is_null()
                || image.width <= 0
                || image.height <= 0
            {
                panic!(
                    "No se pudo cargar la textura: {}",
                    path
                );
            }

            let width =
                image.width as usize;

            let height =
                image.height as usize;

            let colors_ptr =
                ffi::LoadImageColors(
                    image,
                );

            if colors_ptr.is_null() {
                ffi::UnloadImage(image);

                panic!(
                    "No se pudieron leer los colores: {}",
                    path
                );
            }

            let count =
                width * height;

            let colors =
                slice::from_raw_parts(
                    colors_ptr,
                    count,
                );

            let mut data =
                Vec::with_capacity(count);

            for color in colors {
                data.push(
                    Vec3::new(
                        color.r as f32
                            / 255.0,
                        color.g as f32
                            / 255.0,
                        color.b as f32
                            / 255.0,
                    ),
                );
            }

            ffi::UnloadImageColors(
                colors_ptr,
            );

            ffi::UnloadImage(image);

            Self {
                width,
                height,
                data,
            }
        }
    }

    pub fn sample(
        &self,
        u: f32,
        v: f32,
    ) -> Vec3 {
        let u =
            u.rem_euclid(1.0);

        let v =
            v.rem_euclid(1.0);

        let x =
            (
                u
                    * self.width as f32
            ) as usize;

        let y =
            (
                (1.0 - v)
                    * self.height as f32
            ) as usize;

        let x =
            x.min(
                self.width - 1,
            );

        let y =
            y.min(
                self.height - 1,
            );

        self.data[
            y * self.width + x
        ]
    }

    pub fn sample_scalar(
        &self,
        u: f32,
        v: f32,
    ) -> f32 {
        let color =
            self.sample(u, v);

        (
            color.x
                + color.y
                + color.z
        ) / 3.0
    }
}