/*
 * Copyright 2025 Cleuton Sampaio de Melo Junior
 *
 * Licenciado sob a Licença Apache, Versão 2.0 (a "Licença");
 * você não pode usar este arquivo exceto em conformidade com a Licença.
 * Você pode obter uma cópia da Licença em
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * A menos que exigido por lei aplicável ou acordado por escrito, software
 * distribuído sob a Licença é distribuído "COMO ESTÁ",
 * SEM GARANTIAS OU CONDIÇÕES DE QUALQUER TIPO, expressas ou implícitas.
 * Consulte a Licença para o idioma específico que rege as permissões e
 * limitações sob a Licença.
 */

// Importações principais
use ggez::{event, Context, GameResult, glam::*};
use ggez::graphics::{self, Color, DrawMode, Mesh, PxScale, Text, TextFragment};
use ggez::mint;
use nalgebra as na;
use std::time::Instant;

// Estrutura principal do estado do jogo
struct Estado {
    octogono: Mesh,        // Malha gráfica do octógono
    rotacao_octogono: f32, // Ângulo atual de rotação do octógono
    bola: Bola,            // Objeto da bola com física
}

// Estrutura que representa a bola com propriedades físicas
struct Bola {
    posicao: na::Point2<f32>,  // Posição 2D (x,y)
    velocidade: na::Vector2<f32>, // Vetor velocidade
    raio: f32,                // Raio para colisão e desenho
    quica: bool,              // Estado de permissão para quique
    ultimo_quique: Instant,   // Tempo do último quique
}

impl Bola {
    /// Construtor da bola
    /// - x, y: Posição inicial
    /// - raio: Tamanho da bola
    fn new(x: f32, y: f32, raio: f32) -> Self {
        Bola {
            posicao: na::Point2::new(x, y),
            velocidade: na::Vector2::new(0.0, 0.0),
            raio,
            quica: true,
            ultimo_quique: Instant::now(),
        }
    }

    /// Atualiza a física da bola
    /// - dt: Delta time (variação de tempo desde o último frame)
    fn update(&mut self, dt: f32) {
        // Lógica de quique com intervalo mínimo de 0.5 segundos
        if self.quica {
            let now = Instant::now();
            if now.duration_since(self.ultimo_quique).as_secs_f32() > 0.5 {
                self.velocidade.y = -200.0; // Aplica impulso vertical
                self.ultimo_quique = now;
                self.quica = false;
            }
        }

        // Atualização da posição pela velocidade
        self.posicao += self.velocidade * dt;
        // Aplicação da gravidade (980 pixels/s²)
        self.velocidade.y += 980.0 * dt;
    }

    /// Desenha a bola e o texto na tela
    fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        // Criação do texto de propaganda
        let texto = Text::new(TextFragment {
            text: "Veja mais em rustingcrab.com!".to_string(),
            color: Some(Color::new(1.0, 1.0, 0.0, 1.0)), // Amarelo
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(50.0)), // Tamanho da fonte
            ..Default::default()
        });        
        // Posicionamento do texto no canto inferior esquerdo
        canvas.draw(&texto, graphics::DrawParam::from(Vec2::new(20.0, 100.0)));

        // Conversão de coordenadas para o formato mint
        let posicao = mint::Point2 { x: self.posicao.x, y: self.posicao.y };
        // Criação do círculo representando a bola
        let circulo = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            posicao,
            self.raio,
            0.1, // Suavização
            Color::new(0.0, 0.0, 1.0, 1.0), // Azul
        )?;
        // Desenho do círculo
        canvas.draw(&circulo, graphics::DrawParam::default());
        Ok(())
    }

    /// Verifica e resolve colisões com as bordas do octógono
    /// - octogono_points: Lista de pontos transformados do octógono
    fn verificar_colisao_ajustar(&mut self, octogono_points: &[mint::Point2<f32>]) {
        // Itera por todas as arestas do octógono
        for i in 0..octogono_points.len() {
            // Converte pontos para coordenadas nalgebra
            let p1 = na::Point2::new(octogono_points[i].x, octogono_points[i].y);
            let p2 = na::Point2::new(
                octogono_points[(i + 1) % octogono_points.len()].x,
                octogono_points[(i + 1) % octogono_points.len()].y
            );
            
            // Cálculo vetorial para detecção de colisão
            let vetor = p2 - p1; // Vetor da aresta
            let bola_p1 = self.posicao - p1; // Vetor da bola para o ponto inicial da aresta
            // Projeção do vetor bola-p1 na aresta
            let projecao = bola_p1.dot(&vetor) / vetor.dot(&vetor);

            // Determina o ponto mais próximo na aresta
            let mais_proximo = if projecao < 0.0 {
                p1 // Antes do início da aresta
            } else if projecao > 1.0 {
                p2 // Depois do final da aresta
            } else {
                p1 + vetor * projecao // Ponto ao longo da aresta
            };

            // Cálculo da distância entre bola e ponto mais próximo
            let distancia = (self.posicao - mais_proximo).norm();
            if distancia < self.raio {
                // Vetor normal da colisão
                let normal = (self.posicao - mais_proximo).normalize();
                // Corrige posição para fora da colisão
                self.posicao = mais_proximo + normal * self.raio;
                // Reflexão do vetor velocidade
                self.velocidade = self.velocidade - 2.0 * self.velocidade.dot(&normal) * normal;
                self.quica = true; // Libera novo quique
            }
        }
    }
}

// Implementação do loop principal do jogo
impl event::EventHandler<ggez::GameError> for Estado {
    /// Atualiza a lógica do jogo
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        // Atualiza rotação do octógono (1 radiano/segundo)
        self.rotacao_octogono += 1.0 * dt;
        self.bola.update(dt);

        // Transformação dos pontos do octógono considerando rotação e posição
        let rotacao = self.rotacao_octogono;
        let cos_theta = rotacao.cos();
        let sen_theta = rotacao.sin();
        let centro_x = 400.0; // Centro da tela X
        let centro_y = 300.0; // Centro da tela Y

        // Pontos originais do octógono não transformado
        let original_points = [
            na::Point2::new(0.0, -100.0),
            na::Point2::new(-70.0, -70.0),
            na::Point2::new(-100.0, 0.0),
            na::Point2::new(-70.0, 70.0),
            na::Point2::new(0.0, 100.0),
            na::Point2::new(70.0, 70.0),
            na::Point2::new(100.0, 0.0),
            na::Point2::new(70.0, -70.0),
        ];

        // Aplica transformação de rotação e translação
        let mut pontos_transformados = Vec::new();
        for point in &original_points {
            // Rotação usando matriz de rotação 2D
            let rot_x = point.x * cos_theta - point.y * sen_theta;
            let rot_y = point.x * sen_theta + point.y * cos_theta;
            // Translação para posição central
            pontos_transformados.push(mint::Point2 {
                x: rot_x + centro_x,
                y: rot_y + centro_y,
            });
        }

        // Verifica colisões com os pontos transformados
        self.bola.verificar_colisao_ajustar(&pontos_transformados);

        Ok(())
    }

    /// Renderiza todos os elementos gráficos
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::new(0.1, 0.1, 0.1, 1.0));

        // Desenha o octógono com rotação
        let transform = graphics::DrawParam::new()
            .rotation(self.rotacao_octogono)
            .dest(mint::Point2 { x: 400.0, y: 300.0 });
        canvas.draw(&self.octogono, transform);

        // Desenha a bola
        self.bola.draw(ctx, &mut canvas)?;

        // Finaliza renderização
        canvas.finish(ctx)?;
        Ok(())
    }
}

/// Cria a malha do octógono
fn criar_octogono(ctx: &mut Context) -> GameResult<Mesh> {
    // Pontos relativos do octógono (não transformados)
    let pontos = [
        mint::Point2 { x: 0.0, y: -100.0 },
        mint::Point2 { x: -70.0, y: -70.0 },
        mint::Point2 { x: -100.0, y: 0.0 },
        mint::Point2 { x: -70.0, y: 70.0 },
        mint::Point2 { x: 0.0, y: 100.0 },
        mint::Point2 { x: 70.0, y: 70.0 },
        mint::Point2 { x: 100.0, y: 0.0 },
        mint::Point2 { x: 70.0, y: -70.0 },
    ];

    // Constrói a malha gráfica
    let mut builder = graphics::MeshBuilder::new();
    builder.polygon(
        DrawMode::fill(),
        &pontos,
        Color::new(1.0, 1.0, 1.0, 1.0), // Branco
    )?;
    let dados_malha = builder.build();
    Ok(Mesh::from_data(ctx, dados_malha))
}

/// Função principal
fn main() -> GameResult {
    // Configuração inicial da janela
    let cb = ggez::ContextBuilder::new("game_physics_demo", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Se uma IA faz, eu também faço"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));

    // Inicialização do contexto do jogo
    let (mut ctx, event_loop) = cb.build()?;
    let octogono = criar_octogono(&mut ctx)?;
    let bola = Bola::new(400.0, 300.0, 20.0); // Bola centralizada

    // Cria estado inicial
    let state = Estado {
        octogono,
        rotacao_octogono: 0.0,
        bola,
    };

    // Inicia loop principal do jogo
    event::run(ctx, event_loop, state)
}