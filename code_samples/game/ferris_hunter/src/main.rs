/* Game template para usar ggez - Cleuton Sampaio*/

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::conf::{WindowSetup, WindowMode};
use ggez::winit::dpi::PhysicalPosition;
use rdev::{display_size};

fn main() {

    // Calcula tamanho do monitor:
    let (largura, altura) = display_size().unwrap();
    
    // Posição da janela do Game: 
    let pos_x = ((largura as f32) - 1024.0) / 2.0;
    let pos_y = ((altura as f32)  - 768.0) / 2.0;

    // Criar um contexto
    let (mut ctx, loop_eventos) = ContextBuilder::new("ferris_hunter", "Cleuton Sampaio")
        .window_setup(WindowSetup::default().title("Ferrus Hunter!"))
        .window_mode(WindowMode::default().dimensions(1024.0, 768.0))
        .build()
        .expect("aieee, could not create ggez context!");

    // Posiciona a janela do jogo:no centro do monitor
    let context = &ctx.gfx;
    let pos = PhysicalPosition::new(pos_x as i32, pos_y as i32);
    context.set_window_position(pos).unwrap();

    // Cria uma instância do jogo
    let jogo = Jogo::new(&mut ctx);

    // Inicia o loop de eventos do jogo
    event::run(ctx, loop_eventos, jogo);
}

struct Jogo {
    // Aqui você define o estado do jogo: Posições, velocidades, etc.
}

impl Jogo {
    pub fn new(_ctx: &mut Context) -> Jogo {
        // Carregue / crie recursos como imagens aqui.
        Jogo {
            // ...
        }
    }
}

impl EventHandler for Jogo {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Código para atualizar os gameobjects...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Código para desenhar os gameobjects...
        canvas.finish(ctx)
    }
}