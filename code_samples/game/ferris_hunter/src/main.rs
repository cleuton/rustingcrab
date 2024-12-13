/* Game template para usar ggez - Cleuton Sampaio*/

use std::{env, path};
use rand::{thread_rng, Rng};
use ggez::{Context, ContextBuilder, GameResult, glam::*};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::conf::{WindowSetup, WindowMode};
use ggez::winit::dpi::PhysicalPosition;
use rdev::{display_size};

// Constantes globais de configuração do Jogo: 
const TAMANHO_CENARIO: f32 = 300.0;
const LARGURA_CENA: f32 = 1024.0;
const ALTURA_SOLO: f32 = 568.0;
const ALTURA_CENARIO: f32 = ALTURA_SOLO - TAMANHO_CENARIO;
const TEMP_CRAB_LARGURA: f32 = 80.0;
const TEMP_CRAB_ALTURA: f32 = 55.0;
const VELOCIDADE_CENARIO: f32 = 50.0;
const SEGUNDOS_ENTRE_CENARIO: f32 = 5.0;

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

struct Cenario {
    // Aqui você define o estado de um elemento de cenário: Posições, velocidades, etc.
    imagem: graphics::Image,
    posicao: Vec2, // Canto superior esquerdo
    chegou_limite: bool,
}

impl Cenario {
    pub fn new(imagem: graphics::Image, posicao: Vec2) -> Cenario {
        Cenario {
            imagem: imagem,
            posicao: posicao,
            chegou_limite: false,
        }
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        // Atualize o estado do elemento de cenário
        self.posicao.x -= VELOCIDADE_CENARIO * dt.as_secs_f32();
        if self.posicao.x <= -300.0 {
            self.chegou_limite = true;
        }
    }
}

struct Jogo {
    // Aqui você define o estado do jogo: Posições, velocidades, etc.
    background: graphics::Image,
    imagens_cenario: Vec<graphics::Image>,
    crab1: graphics::Image,
    dt: std::time::Duration, // Intervalo de tempo entre frames
    cenarios: Vec<Cenario>,
    segundos_ultimo_cenario: f32,
    indice_ultimo_cenario: i32,
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
            imagens_cenario: carga_cenario,
            crab1: ferris1,
            dt: std::time::Duration::new(0, 0),
            cenarios: Vec::new(),
            segundos_ultimo_cenario: SEGUNDOS_ENTRE_CENARIO,
            indice_ultimo_cenario: -1,
        }
    }
}

impl EventHandler for Jogo {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Código para atualizar os gameobjects...
        self.dt = ctx.time.delta(); // Intervalo de tempo entre frames
        let segundos = 1 * self.dt; // 1 segundo vezes o intervalo de tempo

        let mut cenarios_a_remover = Vec::new();
        // Atualizando os elementos do cenário:
        for indice in 0..self.cenarios.len() {
            let cenario = &mut self.cenarios[indice];
            cenario.update(self.dt);
            if cenario.chegou_limite {
                // Marcar para remover
                cenarios_a_remover.push(indice);
            }
        }

        // Removendo elementos do cenário:
        for indice in cenarios_a_remover {
            self.cenarios.remove(indice);
        }

        // Adicionando elementos ao cenário:
        let mut rng = thread_rng();
        if self.segundos_ultimo_cenario > SEGUNDOS_ENTRE_CENARIO {
            let mut indice: usize;
            loop {
                indice = rng.gen_range(0..self.imagens_cenario.len());
                if indice != self.indice_ultimo_cenario as usize {
                    break;
                }
            }
            self.indice_ultimo_cenario = indice as i32;
            let posicao = Vec2::new(LARGURA_CENA, ALTURA_CENARIO);
            let cenario = Cenario::new(self.imagens_cenario[indice].clone(), posicao);
            self.cenarios.push(cenario);
            self.segundos_ultimo_cenario = 0.0;
        }
        self.segundos_ultimo_cenario += segundos.as_secs_f32();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        // Código para desenhar os gameobjects...

        // Desenhando o cenário de fundo:
        let onde = Vec2::new(0.0, 0.0); 
        canvas.draw(&self.background, graphics::DrawParam::default().dest(onde));

        // Desenhando os elementos do cenário ativos:
        for cenario in &mut self.cenarios {
            cenario.update(self.dt);
            if !cenario.chegou_limite {
                canvas.draw(&cenario.imagem, graphics::DrawParam::default().dest(cenario.posicao));
            }
        }

        canvas.draw(&self.crab1, graphics::DrawParam::default().dest(Vec2::new(200.0, ALTURA_SOLO - TEMP_CRAB_ALTURA)));
        canvas.finish(ctx)
    }
}