// Importa as dependências necessárias do ggez para criar a janela, renderizar gráficos e gerenciar eventos
use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use rand::Rng;

// Define as constantes que configuram o tamanho da grade, tamanho de cada célula e o intervalo entre atualizações
const TAMANHO_GRADE: (usize, usize) = (40, 30);  // (colunas, linhas)
const TAMANHO_CELULA: f32 = 20.0;
const INTERVALO_ATUALIZACAO: f32 = 0.15;  // Segundos entre cada atualização do estado

// Estrutura que representa o estado atual do jogo, incluindo a matriz de células e um acumulador de tempo
struct EstadoJogo {
    celulas: Vec<Vec<bool>>,
    acumulador_tempo: f32,
}

impl EstadoJogo {
    // Função de criação do estado inicial do jogo
    fn novo() -> Self {
        let mut rng = rand::thread_rng();
        // Inicializa a grade com todas as células mortas (false)
        let mut celulas = vec![vec![false; TAMANHO_GRADE.0]; TAMANHO_GRADE.1];
        
        // Preenche aleatoriamente 25% das células como vivas (true)
        for linha in &mut celulas {
            for celula in linha {
                *celula = rng.gen_bool(0.25);
            }
        }

        // Insere um padrão "glider" centralizado na grade, conhecido por se mover diagonalmente
        let centro = (TAMANHO_GRADE.0 / 2, TAMANHO_GRADE.1 / 2);
        celulas[centro.1][centro.0] = true;
        celulas[centro.1][centro.0 + 1] = true;
        celulas[centro.1][centro.0 + 2] = true;
        celulas[centro.1 - 1][centro.0 + 2] = true;
        celulas[centro.1 - 2][centro.0 + 1] = true;

        Self {
            celulas,
            acumulador_tempo: 0.0,
        }
    }

    // Função que conta os vizinhos vivos de uma célula na posição (x, y)
    // Percorre as 8 células ao redor (desconsiderando a própria célula) e soma as que estão vivas
    fn contar_vizinhos(&self, x: usize, y: usize) -> u8 {
        let mut contagem = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // Ignora a própria célula
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                // Verifica se as coordenadas vizinhas estão dentro dos limites da grade
                if nx >= 0 && nx < TAMANHO_GRADE.0 as isize &&
                   ny >= 0 && ny < TAMANHO_GRADE.1 as isize {
                    contagem += self.celulas[ny as usize][nx as usize] as u8;
                }
            }
        }
        contagem
    }

    // Atualiza o estado de todas as células da grade de acordo com as regras do Game of Life
    fn atualizar_celulas(&mut self) {
        // Cria uma cópia da grade para calcular as novas condições
        let mut novas_celulas = self.celulas.clone();

        // Itera por cada célula da grade
        for y in 0..TAMANHO_GRADE.1 {
            for x in 0..TAMANHO_GRADE.0 {
                let vizinhos = self.contar_vizinhos(x, y);
                
                // Aplica as regras:
                // - Célula viva com 2 ou 3 vizinhos permanece viva
                // - Célula morta com exatamente 3 vizinhos se torna viva
                // - Em qualquer outro caso, a célula fica ou se torna morta
                novas_celulas[y][x] = match (self.celulas[y][x], vizinhos) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        // Atualiza a grade com o novo estado calculado
        self.celulas = novas_celulas;
    }
}

// Implementa o trait EventHandler do ggez para gerenciar eventos de atualização e desenho
impl EventHandler for EstadoJogo {
    // Atualiza o estado do jogo a cada frame
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Obtém o tempo decorrido desde o último frame
        let delta = ctx.time.delta().as_secs_f32();
        self.acumulador_tempo += delta;

        // Se o acumulador atingir ou exceder o intervalo definido, atualiza as células
        if self.acumulador_tempo >= INTERVALO_ATUALIZACAO {
            self.atualizar_celulas();
            self.acumulador_tempo = 0.0;
        }
        Ok(())
    }

    // Desenha a grade e as células vivas na tela
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Cria um canvas e define a cor de fundo como preto
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let (largura_tela, altura_tela) = ctx.gfx.drawable_size();

        // Calcula as dimensões totais da grade e os offsets para centralizá-la na tela
        let largura_grade = TAMANHO_GRADE.0 as f32 * TAMANHO_CELULA;
        let altura_grade = TAMANHO_GRADE.1 as f32 * TAMANHO_CELULA;
        let offset_x = (largura_tela - largura_grade) / 2.0;
        let offset_y = (altura_tela - altura_grade) / 2.0;

        // Percorre cada célula da grade para desenhar as que estão vivas
        for y in 0..TAMANHO_GRADE.1 {
            for x in 0..TAMANHO_GRADE.0 {
                if self.celulas[y][x] {
                    // Define um retângulo representando a célula viva, com um pequeno espaçamento
                    let retangulo = graphics::Rect::new(
                        offset_x + x as f32 * TAMANHO_CELULA,
                        offset_y + y as f32 * TAMANHO_CELULA,
                        TAMANHO_CELULA - 1.0,  // Espaço entre células
                        TAMANHO_CELULA - 1.0,
                    );
                    
                    // Cria uma malha retangular preenchida com a cor verde para a célula
                    let malha = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        retangulo,
                        graphics::Color::GREEN,
                    )?;
                    
                    // Desenha a célula no canvas
                    canvas.draw(&malha, graphics::DrawParam::default());
                }
            }
        }

        // Finaliza a renderização do frame
        canvas.finish(ctx)?;
        Ok(())
    }
}

// Função principal que configura o contexto e inicia o loop do jogo
fn main() -> GameResult<()> {
    // Cria o contexto e o loop de eventos com as configurações da janela
    let (ctx, event_loop) = ggez::ContextBuilder::new("game_of_life", "Versão Final")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Game of Life, de Conway")
                .vsync(true)
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(800.0, 600.0)
                .resizable(false))
        .build()?;

    // Inicia o loop do jogo com o estado inicial definido pela função novo()
    event::run(ctx, event_loop, EstadoJogo::novo())
}
