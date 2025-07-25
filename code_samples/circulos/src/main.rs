/*
Copyright 2025 Cleuton Sampaio de Melo Junior

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use ggez::event;
use ggez::graphics::{self, Drawable};
use ggez::{Context, ContextBuilder, GameResult};
use glam::*;
use winit::dpi::LogicalPosition;

const LINHAS: usize = 10;
const COLUNAS: usize = 10;
const RAIO: f32 = 30.0;
const ESPACAMENTO: f32 = 45.0;
const QUADROS: u32 = 60;
const QUADROS_POR_SEGUNDO: f32 = 15.0;

struct CirculosAnimation {
    centros: Vec<Vec2>,
    fases: Vec<f32>,
    indice_quadro: u32,
    tempo_acumulado: f32,
    intervalo_quadro: f32,
}

impl CirculosAnimation {
    fn new() -> Self {
        // Calcula passo de fase
        let passo_fase = 2.0 * std::f32::consts::PI / (LINHAS + COLUNAS) as f32;
        
        // Centraliza a grade na janela (800x800)
        let window_width = 800.0;
        let window_height = 800.0;
        let total_width = (COLUNAS - 1) as f32 * ESPACAMENTO;
        let total_height = (LINHAS - 1) as f32 * ESPACAMENTO;
        let offset_x = (window_width - total_width) / 2.0;
        let offset_y = (window_height - total_height) / 2.0;
        
        // Pré-calcula centros
        let mut centros = Vec::new();
        for j in 0..LINHAS {
            for i in 0..COLUNAS {
                let cx = i as f32 * ESPACAMENTO + offset_x;
                let cy = j as f32 * ESPACAMENTO + offset_y;
                centros.push(Vec2::new(cx, cy));
            }
        }
        
        // Pré-calcula fases
        let mut fases = Vec::new();
        for j in 0..LINHAS {
            for i in 0..COLUNAS {
                let fase = (i + j) as f32 * passo_fase;
                fases.push(fase);
            }
        }
        
        Self {
            centros,
            fases,
            indice_quadro: 0,
            tempo_acumulado: 0.0,
            intervalo_quadro: 1.0 / QUADROS_POR_SEGUNDO,
        }
    }
}

impl event::EventHandler for CirculosAnimation {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Atualiza o tempo
        let delta_time = ctx.time.delta().as_secs_f32();
        self.tempo_acumulado += delta_time;
        
        // Avança quadros com base no tempo
        while self.tempo_acumulado >= self.intervalo_quadro {
            self.indice_quadro = (self.indice_quadro + 1) % QUADROS;
            self.tempo_acumulado -= self.intervalo_quadro;
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        
        // Desenha todos os círculos estáticos em preto
        let circle_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            Vec2::ZERO,
            RAIO,
            0.1,
            graphics::Color::BLACK,
        )?;
        
        for &centro in &self.centros {
            let dest = graphics::DrawParam::new()
                .dest(centro)
                .scale(Vec2::new(1.0, 1.0));
            circle_mesh.draw(&mut canvas, dest);
        }
        
        // Calcula posições dos pontos móveis
        let angulo_base = self.indice_quadro as f32 * (2.0 * std::f32::consts::PI / QUADROS as f32);
        
        let mut pontos_posicoes = Vec::new();
        for (i, &centro) in self.centros.iter().enumerate() {
            let angulo = angulo_base + self.fases[i];
            let offset = Vec2::new(RAIO * angulo.cos(), RAIO * angulo.sin());
            pontos_posicoes.push(centro + offset);
        }
        
        // Desenha pontos móveis em azul
        let ponto_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::ZERO,
            6.0,
            0.1,
            graphics::Color::BLUE,
        )?;
        
        for &posicao in &pontos_posicoes {
            let dest = graphics::DrawParam::new()
                .dest(posicao)
                .scale(Vec2::new(1.0, 1.0));
            ponto_mesh.draw(&mut canvas, dest);
        }
        
        // Finaliza o frame
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn centralizar_janela(ctx: &Context) -> GameResult {
    // Tenta centralizar a janela no monitor
    let window = ctx.gfx.window();
    
    if let Some(monitor) = window.current_monitor() {
        let monitor_size = monitor.size();
        let window_size = window.inner_size();
        
        let x = (monitor_size.width - window_size.width) / 2;
        let y = (monitor_size.height - window_size.height) / 2;
        
        window.set_outer_position(LogicalPosition::new(x as f64, y as f64));
    }
    
    Ok(())
}

fn main() -> GameResult {
    // Cria o contexto do ggez
    let (ctx, event_loop) = ContextBuilder::new("circulos_animation", "autor")
        .window_setup(ggez::conf::WindowSetup::default().title("Efeito de onda - Cleuton"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 800.0))
        .build()?;
    
    // Tenta centralizar a janela
    let _ = centralizar_janela(&ctx);
    
    // Cria a instância do jogo
    let game = CirculosAnimation::new();
    
    // Roda o loop de eventos
    event::run(ctx, event_loop, game)
}