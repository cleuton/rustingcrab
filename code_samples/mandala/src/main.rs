use image::{GrayImage, Luma, imageops};
use imageproc::{
    drawing::draw_line_segment_mut,
    geometric_transformations::{rotate_about_center, Interpolation},
};
use rand::Rng;

/// 1) Gera um “rabisco” aleatório dentro de um quadrado d×d
fn gerar_rabisco(d: u32) -> GrayImage {
    let mut img = GrayImage::from_pixel(d, d, Luma([255]));
    let mut rng = rand::thread_rng();

    // desenha círculos, retângulos e linhas
    for _ in 0..(d / 4) {
        match rng.gen_range(0..3) {
            0 => {
                // círculo simples
                let x = rng.gen_range(0..d) as i32;
                let y = rng.gen_range(0..d) as i32;
                let r = rng.gen_range(5..(d / 8).max(6)) as i32;
                for dx in -r..=r {
                    for dy in -r..=r {
                        if dx*dx + dy*dy <= r*r {
                            let nx = x + dx; let ny = y + dy;
                            if (0..d as i32).contains(&nx) && (0..d as i32).contains(&ny) {
                                img.put_pixel(nx as u32, ny as u32, Luma([0]));
                            }
                        }
                    }
                }
            }
            1 => {
                // linha aleatória
                let x0 = rng.gen_range(0.0..d as f32);
                let y0 = rng.gen_range(0.0..d as f32);
                let x1 = rng.gen_range(0.0..d as f32);
                let y1 = rng.gen_range(0.0..d as f32);
                draw_line_segment_mut(&mut img, (x0, y0), (x1, y1), Luma([0]));
            }
            2 => {
                // retângulo
                let x0 = rng.gen_range(0..d);
                let y0 = rng.gen_range(0..d);
                let w  = rng.gen_range(5..(d/4).max(6));
                let h  = rng.gen_range(5..(d/4).max(6));
                for xx in x0..(x0+w).min(d) {
                    for yy in y0..(y0+h).min(d) {
                        img.put_pixel(xx, yy, Luma([0]));
                    }
                }
            }
            _ => {}
        }
    }

    img
}

/// 2+3) A partir do “rabisco”, constrói um setor 2d×2d pelo espelho diagonal e vertical
fn gerar_setor(d: u32) -> GrayImage {
    let base = gerar_rabisco(d);

    // espelha sobre a diagonal: rota 90° e depois flip horizontal
    let diag = {
        let rot = rotate_about_center(
            &base,
            std::f32::consts::PI / 2.0,
            Interpolation::Nearest,
            Luma([255]),
        );
        imageops::flip_horizontal(&rot)
    };

    // monta um bloco 2d×2d
    let mut quad = GrayImage::from_pixel(2*d, 2*d, Luma([255]));
    imageops::replace(&mut quad, &base, 0, 0);
    imageops::replace(&mut quad, &diag, d as i64, 0);
    imageops::replace(&mut quad, &imageops::flip_vertical(&diag), 0, d as i64);
    imageops::replace(&mut quad, &imageops::flip_vertical(&base),   d as i64, d as i64);

    quad
}

/// 4+5) Replica o setor por rotação e aplica máscara circular
fn montar_mandala(setor: &GrayImage, setores: usize) -> GrayImage {
    let size = setor.width();
    let mut mandala = GrayImage::from_pixel(size, size, Luma([255]));
    let centro = size as f32 / 2.0;
    let passo = 2.0 * std::f32::consts::PI / setores as f32;

    for i in 0..setores {
        let ang = i as f32 * passo;
        let rot = rotate_about_center(setor, ang, Interpolation::Nearest, Luma([255]));
        imageops::overlay(&mut mandala, &rot, 0, 0);
    }

    // máscara circular para limpar excesso
    for x in 0..size {
        for y in 0..size {
            let dx = x as f32 - centro;
            let dy = y as f32 - centro;
            if dx*dx + dy*dy > centro*centro {
                mandala.put_pixel(x, y, Luma([255]));
            }
        }
    }

    mandala
}

fn main() {
    let d = 200;        // tamanho do rabisco
    let setores = 8;    // quantas fatias na mandala

    let setor   = gerar_setor(d);
    let mandala = montar_mandala(&setor, setores);
    mandala.save("mandala.png").unwrap();
    println!("Mandala gerada em mandala.png");
}
