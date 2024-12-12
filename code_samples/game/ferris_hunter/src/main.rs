/* Game template para usar ggez - Cleuton Sampaio*/

use std::{env, path};
use ggez::{Context, ContextBuilder, GameResult, glam::*};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::conf::{WindowSetup, WindowMode};
use ggez::winit::dpi::PhysicalPosition;
use rdev::{display_size};

// Constantes globais de configuração do Jogo: 
const TAMANHO_CENARIO: f32 = 300.0;
const ALTURA_CENA: f32 = 768.0;
const LARGURA_CENA: f32 = 1024.0;
const ALTURA_SOLO: f32 = 568.0;
const ALTURA_CENARIO: f32 = ALTURA_SOLO - TAMANHO_CENARIO;
const TEMP_CRAB_LARGURA: f32 = 80.0;
const TEMP_CRAB_ALTURA: f32 = 55.0;

fn main() {

    // Calcula tamanho do monitor:
    let (largura, altura) = display_size().unwrap();
    
    // Posição da janela do Game: 
    let pos_x = ((largura as f32) - 1024.0) / 2.0;
    let pos_y = ((altura as f32)  - 768.0) / 2.0;

    // Criar um contexto
    // Path de recursos (imagens etc):
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Path de recursos: {:?}", path);
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, loop_eventos) = ContextBuilder::new("ferris_hunter", "Cleuton Sampaio")
        .add_resource_path(resource_dir)
        .window_setup(WindowSetup::default().title("Ferrus Hunter!"))
        .window_mode(WindowMode::default().dimensions(1024.0, 768.0))
        .build()
        .expect("Não foi possível criar o contexto do jogo!");

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
    background: graphics::Image,
    cenario: Vec<graphics::Image>,
    crab1: graphics::Image,
}

impl Jogo {
    pub fn new(_ctx: &mut Context) -> Jogo {
        // Carregue / crie recursos como imagens aqui.
        let background = graphics::Image::from_path(_ctx, "/scene.png").unwrap();
        let mut carga_cenario = Vec::new();
        carga_cenario.push(graphics::Image::from_path(_ctx, "/arbusto.png").unwrap());   
        carga_cenario.push(graphics::Image::from_path(_ctx, "/poste.png").unwrap());   
        carga_cenario.push(graphics::Image::from_path(_ctx, "/arvore.png").unwrap());   
        let ferris1 =  graphics::Image::from_path(_ctx, "/crab1.png").unwrap();
        Jogo {
            // ...
            background: background,
            cenario: carga_cenario,
            crab1: ferris1,
        }
    }
}

impl EventHandler for Jogo {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Código para atualizar os gameobjects...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Código para desenhar os gameobjects...
        // Desenhando o cenário de fundo:
        let onde = Vec2::new(0.0, 0.0); 
        canvas.draw(&self.background, graphics::DrawParam::default().dest(onde));
        // Desenhando os elementos do cenário:
        canvas.draw(&self.cenario[0], graphics::DrawParam::default().dest(Vec2::new(300.0, ALTURA_CENARIO)));
        canvas.draw(&self.cenario[1], graphics::DrawParam::default().dest(Vec2::new(500.0, ALTURA_CENARIO)));
        canvas.draw(&self.cenario[2], graphics::DrawParam::default().dest(Vec2::new(800.0, ALTURA_CENARIO)));
        canvas.draw(&self.crab1, graphics::DrawParam::default().dest(Vec2::new(200.0, ALTURA_SOLO - TEMP_CRAB_ALTURA)));
        canvas.finish(ctx)
    }
}