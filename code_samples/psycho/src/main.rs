use ggez::event::{self, EventHandler};
use ggez::graphics::{Canvas, Color, DrawParam, Image, ImageFormat};
use ggez::{Context, ContextBuilder, GameResult};

// Estrutura de estado do jogo
type Pixels = Vec<u8>; // RGBA por pixel

struct Estado {
    largura: usize,
    altura: usize,
    malha_x: Vec<f32>,
    malha_y: Vec<f32>,
    buffer: Pixels,
}

impl Estado {
    // Inicializa o estado, gera a malha e aloca o buffer
    pub fn novo(_ctx: &mut Context, largura: usize, altura: usize) -> GameResult<Estado> {
        let mut malha_x = Vec::with_capacity(largura * altura);
        let mut malha_y = Vec::with_capacity(largura * altura);
        for j in 0..altura {
            for i in 0..largura {
                let x = -1.0 + 2.0 * (i as f32) / ((largura - 1) as f32);
                let y = -1.0 + 2.0 * (j as f32) / ((altura - 1) as f32);
                malha_x.push(x);
                malha_y.push(y);
            }
        }
        let buffer = vec![0; largura * altura * 4];
        Ok(Estado { largura, altura, malha_x, malha_y, buffer })
    }

    // Atualiza o buffer de pixels com o efeito caleidoscópio
    fn atualizar_buffer(&mut self, tempo: f32, segmentos: f32) {
        let ang_seg = std::f32::consts::PI * 2.0 / segmentos;
        for idx in 0..(self.largura * self.altura) {
            let x = self.malha_x[idx];
            let y = self.malha_y[idx];
            let r = (x * x + y * y).sqrt();
            let theta = y.atan2(x);
            let theta_mod = (theta % ang_seg).abs();
            let theta_dob = (theta_mod - ang_seg / 2.0).abs();

            let padrao_radial = (10.0 * r - tempo * 4.0).sin();
            let padrao_angular = (segmentos * theta_dob + tempo * 6.0).sin();
            let padrao = (padrao_radial + padrao_angular) * 0.5;
            let cor = ((padrao + 1.0) * 0.5 * 255.0) as u8;

            let r_c = cor;
            let g_c = (((10.0 * r - tempo * 4.0 + 2.0).sin() + 1.0) * 0.5 * 255.0) as u8;
            let b_c = (((10.0 * r - tempo * 4.0 + 4.0).sin() + 1.0) * 0.5 * 255.0) as u8;

            let base = idx * 4;
            self.buffer[base] = r_c;
            self.buffer[base + 1] = g_c;
            self.buffer[base + 2] = b_c;
            self.buffer[base + 3] = 255;
        }
    }
}

impl EventHandler for Estado {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let tempo = ctx.time.time_since_start().as_secs_f32();
        self.atualizar_buffer(tempo, 16.0);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Cria imagem a partir dos pixels RGBA
        let img = Image::from_pixels(
            ctx,
            &self.buffer,
            ImageFormat::Rgba8Unorm,
            self.largura as u32,
            self.altura as u32,
        );
        // Desenha usando Canvas
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&img, DrawParam::default());
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult<()> {
    // Monta contexto e event loop
    let (mut ctx, event_loop) = ContextBuilder::new(
        "kaleidoscopio_psicodelico",
        "Psycho",
    )
    .window_setup(ggez::conf::WindowSetup::default().title("Caleidoscópio Psicodélico - Túnel"))
    .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
    .build()?;

    // Cria estado
    let estado = Estado::novo(&mut ctx, 800, 600)?;
    // Roda
    event::run(ctx, event_loop, estado)
}