/* Game template para usar ggez - Cleuton Sampaio*/

use std::{env, path};
use std::time::Instant;
use rand::{thread_rng, Rng};
use ggez::{Context, ContextBuilder, GameResult, glam::*};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::conf::{WindowSetup, WindowMode};
use ggez::winit::dpi::PhysicalPosition;
use ggez::input::keyboard::{KeyCode, KeyInput};
use rdev::{display_size};

// Constantes globais de configuração do Jogo: 
const TAMANHO_CENARIO: f32 = 300.0;
const LARGURA_CENA: f32 = 1024.0;
const ALTURA_SOLO: f32 = 568.0;
const ALTURA_CENARIO: f32 = ALTURA_SOLO - TAMANHO_CENARIO;
const TEMP_CRAB_ALTURA: f32 = 55.0;
const VELOCIDADE_CENARIO: f32 = 50.0;
const SEGUNDOS_ENTRE_CENARIO: f32 = 5.0;
const SEGUNDOS_PARA_VIRAR: f32 = 1.0;
const VELOCIDADE_PULO_GAMEOBJECTS: f32 = 300.0;
const VELOCIDADE_NPC: f32 = 200.0;
const TEMPO_MINIMO_LANCAMENTO_NPC: f64 = 3.0;
const TIPOS_NPC: i32 = 3;

// Funções auxiliares

fn intercecao_retangulos(canto_superior_esquerdo1: Vec2, 
                         canto_inferior_direito1: Vec2,
                         canto_superior_esquerdo2: Vec2,
                         canto_inferior_direito2: Vec2) -> bool {
    // Verifica se dois retângulos se interceptam
    if canto_superior_esquerdo1.x < canto_inferior_direito2.x &&
        canto_inferior_direito1.x > canto_superior_esquerdo2.x &&
        canto_superior_esquerdo1.y < canto_inferior_direito2.y &&
        canto_inferior_direito1.y > canto_superior_esquerdo2.y {
            return true;
        }
    false
}

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

trait GameObject {
    fn update(&mut self, dt: std::time::Duration);
    fn colidiu(&self, outro: &PropriedadesComuns) -> bool;
    fn desenhar(&self, canvas: &mut graphics::Canvas);
    fn obter_propriedades(&mut self) -> &mut PropriedadesComuns;
}

#[derive(Clone)]
struct PropriedadesComuns {
    imagem1: graphics::Image,
    imagem2: graphics::Image,
    posicao: Vec2,
    largura: f32,
    altura: f32,
    segundos_para_virar: f32,
    velocidade: f32,
    invertido: bool,
    saiu_de_cena: bool,
    pulando: bool,
    caindo: bool,
    limite_pulo: f32,
    posicao_vertical_original: f32,
    recuando: bool,
    acelerando: bool,
}

impl PropriedadesComuns {
    fn new(_ctx: &mut Context,
        im1: &str, 
        im2: &str,
        posicao: Vec2,
        ) -> PropriedadesComuns {
            let i1 = graphics::Image::from_path(_ctx, im1).unwrap();
            let i2 = graphics::Image::from_path(_ctx, im2).unwrap();
            let largura = i1.width() as f32; // extrai largura antes de mover i1
            let altura = i1.height() as f32; // extrai altura antes de mover i1
            PropriedadesComuns {
                imagem1: i1,
                imagem2: i2,
                posicao: posicao,
                largura: largura,
                altura: altura,
                segundos_para_virar: SEGUNDOS_PARA_VIRAR,
                velocidade: VELOCIDADE_NPC,
                invertido: false,
                saiu_de_cena: false,
                pulando: false,
                caindo: false,
                limite_pulo: 0.0,
                posicao_vertical_original: posicao.y,
                recuando: false,
                acelerando: false,
            }
    }
}

impl GameObject for PropriedadesComuns {
    fn update(&mut self, dt: std::time::Duration) {
        // Atualize o estado do gameobject
        self.posicao.x -= self.velocidade * dt.as_secs_f32();
        self.segundos_para_virar = self.segundos_para_virar + dt.as_secs_f32();
        if self.posicao.x <= -1.0 * self.largura {
            self.saiu_de_cena = true;
        } else {
            if self.segundos_para_virar >= SEGUNDOS_PARA_VIRAR  {
                self.invertido = !self.invertido;
                self.segundos_para_virar = 0.0;
            }            
        }
    }

    fn colidiu(&self, outro: &PropriedadesComuns) -> bool {
        // Verifique se houve colisão
        let canto_inferior_direito1 = self.posicao + Vec2::new(self.largura, self.altura);
        let canto_inferior_direito2 = outro.posicao + Vec2::new(outro.largura, outro.altura);   
        if intercecao_retangulos(self.posicao, canto_inferior_direito1, outro.posicao, canto_inferior_direito2) {
            return true;
        }
        false
    }

    fn desenhar(&self, canvas: &mut graphics::Canvas) {
        if self.invertido {
            canvas.draw(&self.imagem2, graphics::DrawParam::default().dest(self.posicao));
        } else {
            canvas.draw(&self.imagem1, graphics::DrawParam::default().dest(self.posicao));
        }
    }
    
    fn obter_propriedades(&mut self) -> &mut PropriedadesComuns {
        self
    }
}

struct Ferris {
    // Aqui você define o estado do Ferris: Posições, velocidades, etc.
    propriedades: PropriedadesComuns,
}

impl Ferris {
    pub fn new(propriedades: PropriedadesComuns) -> Ferris {
        Ferris {
            propriedades: propriedades.clone(),
        }
    }
}

impl GameObject for Ferris {
    fn update(&mut self, dt: std::time::Duration) {
        // O Ferris é um player portanto não usa a impl das propriedades comuns
        // Ele pode pular (SETA PARA CIMA) ou recuar (SETA PARA ESQUERDA).
        let velocidade_queda = VELOCIDADE_PULO_GAMEOBJECTS * 1.5 as f32;
        if self.propriedades.pulando {
            if self.propriedades.caindo {
                self.propriedades.posicao.y = self.propriedades.posicao.y + velocidade_queda * dt.as_secs_f32();
                if self.propriedades.posicao.y >= self.propriedades.posicao_vertical_original {
                    self.propriedades.posicao.y = self.propriedades.posicao_vertical_original;
                    self.propriedades.pulando = false;
                    self.propriedades.caindo = false;
                }
            } else {
                self.propriedades.posicao.y = self.propriedades.posicao.y - VELOCIDADE_PULO_GAMEOBJECTS * dt.as_secs_f32();
                if self.propriedades.posicao.y <= self.propriedades.limite_pulo {
                    self.propriedades.caindo = true;
                }
            }
        } else if self.propriedades.recuando {
            if self.propriedades.acelerando {
                self.propriedades.posicao.x = self.propriedades.posicao.x + self.propriedades.velocidade * dt.as_secs_f32();
                if self.propriedades.posicao.x >= 200.0 {
                    self.propriedades.posicao.x = 200.0;
                    self.propriedades.recuando = false;
                    self.propriedades.acelerando = false;
                }
            } else {
                self.propriedades.posicao.x = self.propriedades.posicao.x - self.propriedades.velocidade * dt.as_secs_f32();
                if self.propriedades.posicao.x <=  0.0 {
                    self.propriedades.acelerando = true;
                }
            }
        }

        if !self.propriedades.pulando && !self.propriedades.recuando {
            self.propriedades.segundos_para_virar = self.propriedades.segundos_para_virar + dt.as_secs_f32();
            if self.propriedades.segundos_para_virar >= SEGUNDOS_PARA_VIRAR  {
                self.propriedades.invertido = !self.propriedades.invertido;
                self.propriedades.segundos_para_virar = 0.0;
            }            
        }
    }

    fn colidiu(&self, outro: &PropriedadesComuns) -> bool {
        // Verifique se houve colisão
        self.propriedades.colidiu(outro)
    }

    fn desenhar(&self, canvas: &mut graphics::Canvas) {
        self.propriedades.desenhar(canvas);
    }

    fn obter_propriedades(&mut self) -> &mut PropriedadesComuns {
        &mut self.propriedades
    }
}   

struct Cobra {
    propriedades: PropriedadesComuns,
}

impl Cobra {
    pub fn new(propriedades: PropriedadesComuns) -> Cobra {
        Cobra {
            propriedades: propriedades.clone(),
        }
    }
}

impl GameObject for Cobra {
    fn update(&mut self, dt: std::time::Duration) {
        // Atualize o estado do gameobject
        self.propriedades.update(dt);
    }

    fn colidiu(&self, outro: &PropriedadesComuns) -> bool {
        // Verifique se houve colisão
        self.propriedades.colidiu(outro)
    }

    fn desenhar(&self, canvas: &mut graphics::Canvas) {
        self.propriedades.desenhar(canvas);
    }

    fn obter_propriedades(&mut self) -> &mut PropriedadesComuns {
        &mut self.propriedades
    }
}

struct Gopher {
    propriedades: PropriedadesComuns,
}

impl Gopher {
    pub fn new(propriedades: PropriedadesComuns) -> Gopher {
        Gopher {
            propriedades: propriedades.clone(),
        }
    }
}

impl GameObject for Gopher {
    fn update(&mut self, dt: std::time::Duration) {
        // O Gopher é mais lento, mas pode pular
        self.propriedades.posicao.x -= (self.propriedades.velocidade * 0.6) * dt.as_secs_f32();
        if self.propriedades.posicao.x <= -1.0 * self.propriedades.largura {
            self.propriedades.saiu_de_cena = true;
            return;
        } 
        // O Gopher pode pular aleatoriamente
        if self.propriedades.pulando {
            if self.propriedades.caindo {
                self.propriedades.posicao.y = self.propriedades.posicao.y + VELOCIDADE_PULO_GAMEOBJECTS * dt.as_secs_f32();
                if self.propriedades.posicao.y >= self.propriedades.posicao_vertical_original {
                    self.propriedades.posicao.y = self.propriedades.posicao_vertical_original;
                    self.propriedades.pulando = false;
                    self.propriedades.caindo = false;
                }
            } else {
                self.propriedades.posicao.y = self.propriedades.posicao.y - VELOCIDADE_PULO_GAMEOBJECTS * dt.as_secs_f32();
                if self.propriedades.posicao.y <= self.propriedades.limite_pulo {
                    self.propriedades.caindo = true;
                }
            }
        } 

        if !self.propriedades.pulando {
            self.propriedades.segundos_para_virar = self.propriedades.segundos_para_virar + dt.as_secs_f32();
            if self.propriedades.segundos_para_virar >= SEGUNDOS_PARA_VIRAR  {
                self.propriedades.invertido = !self.propriedades.invertido;
                self.propriedades.segundos_para_virar = 0.0;
            }            

            // Pular aleatoriamente
            let mut rng = thread_rng();
            let pular = rng.gen_range(0..80);
            if pular == 1 {
                self.propriedades.pulando = true;
                self.propriedades.caindo = false;
            }
        } 
    }

    fn colidiu(&self, outro: &PropriedadesComuns) -> bool {
        // Verifique se houve colisão
        self.propriedades.colidiu(outro)
    }

    fn desenhar(&self, canvas: &mut graphics::Canvas) {
        self.propriedades.desenhar(canvas);
    }

    fn obter_propriedades(&mut self) -> &mut PropriedadesComuns {
        &mut self.propriedades
    }
}

struct Xicara {
    propriedades: PropriedadesComuns,
}

impl Xicara {
    pub fn new(propriedades: PropriedadesComuns) -> Xicara {
        let mut rng = thread_rng();
        let altura = rng.gen_range(80..(ALTURA_SOLO-80.0) as i32) as f32;
        let mut xicara = Xicara {
            propriedades: propriedades.clone(),
        };
        xicara.propriedades.posicao.y = altura;
        let redutor_velocidade = rng.gen_range(1..5) as f32;
        xicara.propriedades.velocidade = VELOCIDADE_NPC / redutor_velocidade;
        xicara.propriedades.posicao_vertical_original = altura;
        xicara
    }
}

impl GameObject for Xicara  {
    fn update(&mut self, dt: std::time::Duration) {
        self.propriedades.update(dt);
    }

    fn colidiu(&self, outro: &PropriedadesComuns) -> bool {
        self.propriedades.colidiu(outro)
    }

    fn desenhar(&self, canvas: &mut graphics::Canvas) {
        self.propriedades.desenhar(canvas);
    }

    fn obter_propriedades(&mut self) -> &mut PropriedadesComuns {
        &mut self.propriedades
    }
}

struct Jogo {
    // Aqui você define o estado do jogo: Posições, velocidades, etc.
    background: graphics::Image,
    imagens_cenario: Vec<graphics::Image>,
    dt: std::time::Duration, // Intervalo de tempo entre frames
    cenarios: Vec<Cenario>,
    segundos_ultimo_cenario: f32,
    indice_ultimo_cenario: i32,
    player: Ferris,
    npcs: Vec<Box<dyn GameObject>>,
    ultima_vez_npc_lancado: Instant,
    terminou: bool,
}

impl Jogo {
    pub fn new(_ctx: &mut Context) -> Jogo {

        // Carregue / crie recursos aqui.
        let background = graphics::Image::from_path(_ctx, "/scene.png").unwrap();
        let mut carga_cenario = Vec::new();
        carga_cenario.push(graphics::Image::from_path(_ctx, "/arbusto.png").unwrap());   
        carga_cenario.push(graphics::Image::from_path(_ctx, "/poste.png").unwrap());   
        carga_cenario.push(graphics::Image::from_path(_ctx, "/arvore.png").unwrap());   
        let propriedades_ferris = PropriedadesComuns::new(_ctx, "/crab1.png", 
                                                            "/crab2.png", 
                                                            Vec2::new(200.0, ALTURA_SOLO - TEMP_CRAB_ALTURA));
        let player = Ferris::new(propriedades_ferris);
        
        Jogo {
            // ...
            background: background,
            imagens_cenario: carga_cenario,
            dt: std::time::Duration::new(0, 0),
            cenarios: Vec::new(),
            segundos_ultimo_cenario: SEGUNDOS_ENTRE_CENARIO,
            indice_ultimo_cenario: -1,
            player: player,
            npcs: Vec::new(),
            ultima_vez_npc_lancado: Instant::now(),
            terminou: false,
        }
    }    
}

impl EventHandler for Jogo {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        // terminou o jogo?
        if self.terminou {
            return Ok(());
        }

        // Código para atualizar os gameobjects...
        self.dt = ctx.time.delta(); // Intervalo de tempo entre frames
        let segundos = 1 * self.dt; // 1 segundo vezes o intervalo de tempo
        let mut rng = thread_rng();

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

        // Atualizando o player:
        self.player.update(self.dt);

        // Atualizando NPCs:
        let mut npcs_a_remover: Vec<i32> = Vec::new();
        for i in 0..self.npcs.len() {
            let npc = &mut self.npcs[i];
            npc.update(self.dt);
            if npc.colidiu(&self.player.propriedades) {
                self.terminou = true;
                return Ok(());
            }

            if npc.obter_propriedades().saiu_de_cena {
                npcs_a_remover.push(i as i32);
            }
        }

        // Devemos lançar novos NPCs?
        let tempo_decorrido_ultimo_npc = self.ultima_vez_npc_lancado.elapsed().as_secs_f64();
        if tempo_decorrido_ultimo_npc >= TEMPO_MINIMO_LANCAMENTO_NPC {
            if rng.gen_range(0..2) == 1 {
                let tipo = rng.gen_range(0..TIPOS_NPC);
                match tipo {
                    0 => {
                        let propriedades_cobra = PropriedadesComuns::new(ctx, "/cobra1.png", "/cobra2.png", Vec2::new(1024.0, ALTURA_SOLO - 31.0));
                        let cobra = Cobra::new(propriedades_cobra);
                        self.npcs.push(Box::new(cobra));
                    },
                    1 => {
                        let propriedades_xicara = PropriedadesComuns::new(ctx, "/xicara1.png", "/xicara2.png", Vec2::new(1024.0, ALTURA_SOLO - 60.0));
                        let xicara = Xicara::new(propriedades_xicara);
                        self.npcs.push(Box::new(xicara));
                    },
                    2 => {
                        let mut propriedades_gopher = PropriedadesComuns::new(ctx, "/gopher1.png", "/gopher2.png", Vec2::new(1024.0, ALTURA_SOLO - 49.0));
                        propriedades_gopher.limite_pulo = propriedades_gopher.posicao.y - 200.0;
                        let gopher = Gopher::new(propriedades_gopher);
                        self.npcs.push(Box::new(gopher));
                    },
                    _ => (),
                }
            }
            self.ultima_vez_npc_lancado = Instant::now();
        }

        // Atualizando segundos do último cenário:
        self.segundos_ultimo_cenario += segundos.as_secs_f32();

        // Removendo NPCs:
        for i in npcs_a_remover {
            self.npcs.remove(i as usize);
        }

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

        // Desenhando o player:
        self.player.desenhar(&mut canvas);

        // Desenhando NPCs:
        for npc in &mut self.npcs {
            npc.desenhar(&mut canvas);
        }

        if self.terminou {

        }

        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool)  -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => {
                if !self.player.propriedades.pulando {
                    self.player.propriedades.pulando = true;
                    self.player.propriedades.caindo = false;
                }
            },
            Some(KeyCode::Left) => {
                if !self.player.propriedades.recuando && !self.player.propriedades.pulando {
                    self.player.propriedades.recuando = true;
                    self.player.propriedades.acelerando = false;
                }
            },
            _ => (),
        }
        Ok(())
    }
}