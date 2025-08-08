// cargo run -- --file "./texto.txt" --lpm 60 --zoom 36

use clap::Parser;
use ggez::{
    event::{self, EventHandler},
    graphics::{self, Canvas, Color, DrawParam, Drawable, Mesh, PxScale, Rect, Text, TextFragment},
    Context, ContextBuilder, GameResult,
};
use ggez::input::mouse::MouseButton;
use regex::Regex;

/// CLI
#[derive(Parser, Debug)]
struct Args {
    /// Caminho do arquivo .txt
    #[arg(short, long)]
    file: String,
    /// Linhas por minuto (controle “linha a linha”)
    #[arg(short, long, default_value_t = 60.0)]
    lpm: f32,
    /// Tamanho da fonte (px)
    #[arg(short = 'z', long, default_value_t = 36.0)]
    zoom: f32,
}

#[derive(Clone, Copy)]
enum ButtonKind {
    PausePlay,
    ZoomIn,
    ZoomOut,
    Faster,
    Slower,
    Restart,
}

struct Button {
    rect: Rect,
    label: &'static str,
    kind: ButtonKind,
}

struct ParagraphLayout {
    lines: Vec<String>,
}

struct Teleprompter {
    // Entrada
    raw_text: String,

    // Layout fixo (sem reflow durante rolagem)
    paragraphs: Vec<ParagraphLayout>,
    stream_lines: Vec<String>, // lead-in + linhas + gaps

    // Parâmetros visuais
    font_size: f32,
    line_px: f32,

    // Rolagem
    paused: bool,
    current_line: f32,   // índice “contínuo” da linha corrente (float)
    lines_per_sec: f32,  // lpm/60

    // Separação
    para_gap_lines: usize,
    lead_in_lines: usize,

    // UI
    buttons: Vec<Button>,

    // Relayout apenas quando necessário
    need_relayout: bool,
    last_layout_width: f32,
}

impl Teleprompter {
    fn new(ctx: &mut Context, raw_text: String, font_size: f32, lpm: f32) -> GameResult<Self> {
        let mut s = Self {
            raw_text,
            paragraphs: vec![],
            stream_lines: vec![],
            font_size,
            line_px: font_size * 1.20,
            paused: true, // começa pausado
            current_line: 0.0,
            lines_per_sec: (lpm.max(1.0)) / 60.0,
            para_gap_lines: 2, // 2 linhas em branco entre parágrafos
            lead_in_lines: 6,  // tela “vazia” por 6 linhas
            buttons: vec![],
            need_relayout: true,
            last_layout_width: 0.0,
        };
        s.layout_buttons(ctx);
        s.relayout(ctx)?;
        // Começa “antes” do começo real para a tela vir vazia.
        s.current_line = -(s.lead_in_lines as f32);
        Ok(s)
    }

    fn layout_buttons(&mut self, ctx: &mut Context) {
        let (w, _) = ctx.gfx.drawable_size();
        let mut x = 12.0;
        let y = 12.0;
        let bw = 135.0;
        let bh = 36.0;
        let gap = 8.0;

        let items = [
            (ButtonKind::PausePlay, "Pausar/Retomar"),
            (ButtonKind::ZoomIn, "Zoom +"),
            (ButtonKind::ZoomOut, "Zoom -"),
            (ButtonKind::Faster, "Velocidade +"),
            (ButtonKind::Slower, "Velocidade -"),
            (ButtonKind::Restart, "Reiniciar"),
        ];

        self.buttons.clear();
        for (kind, label) in items {
            if x + bw > w - 12.0 { break; }
            self.buttons.push(Button { rect: Rect::new(x, y, bw, bh), label, kind });
            x += bw + gap;
        }
    }

    /// Pré-quebra em linhas fixas e monta o “stream” (lead-in + linhas + gaps).
    /// Só roda no início, zoom ou resize (quando `need_relayout = true`).
    fn relayout(&mut self, ctx: &mut Context) -> GameResult {
        if !self.need_relayout { return Ok(()); }
        self.need_relayout = false;

        // 1) separa parágrafos por linha em branco
        let re_split = Regex::new(r"\r?\n\s*\r?\n+").unwrap();
        // 2) remove numeração existente "1) " no início do parágrafo
        let re_leading = Regex::new(r"^\s*\d+\)\s*").unwrap();

        let blocks: Vec<String> = re_split
            .split(&self.raw_text.replace('\r', ""))
            .map(|p| re_leading.replace(p.trim(), "").to_string())
            .filter(|p| !p.is_empty())
            .collect();

        // 3) wrap por largura
        let (w, _) = ctx.gfx.drawable_size();
        let max_w = (w * 0.90).max(200.0);
        self.last_layout_width = w;

        let mut paragraphs = Vec::new();
        for (i, p) in blocks.iter().enumerate() {
            let numbered = format!("{}) {}", i + 1, p.replace('\n', " "));
            let lines = wrap_text(ctx, &numbered, self.font_size, max_w);
            paragraphs.push(ParagraphLayout { lines });
        }
        self.paragraphs = paragraphs;

        // 4) stream: lead-in + texto + gaps
        let mut stream = Vec::new();
        for _ in 0..self.lead_in_lines { stream.push(String::new()); }
        for para in &self.paragraphs {
            for l in &para.lines { stream.push(l.clone()); }
            for _ in 0..self.para_gap_lines { stream.push(String::new()); }
        }
        self.stream_lines = stream;

        Ok(())
    }
}

/// Quebra por largura (fixa por zoom/largura)
fn wrap_text(ctx: &mut Context, s: &str, font_px: f32, max_w: f32) -> Vec<String> {
    let mut out = Vec::new();
    let mut curr = String::new();
    let scale = PxScale::from(font_px);

    for word in s.split_whitespace() {
        let tent = if curr.is_empty() { word.to_string() } else { format!("{} {}", curr, word) };
        let t = Text::new(TextFragment::new(&tent).scale(scale));
        if let Some(rect) = t.dimensions(&ctx.gfx) {
            if rect.w <= max_w {
                curr = tent;
            } else {
                if !curr.is_empty() { out.push(curr); }
                curr = word.to_string();
            }
        } else {
            curr = tent;
        }
    }
    if !curr.is_empty() { out.push(curr); }
    out
}

impl EventHandler for Teleprompter {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Relayout apenas quando marcado (zoom/resize)
        self.relayout(ctx)?;

        if !self.paused {
            let dt = ctx.time.delta().as_secs_f32();
            self.current_line += self.lines_per_sec * dt; // avança frações de linha, suave
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(10, 10, 10));

        // Botões
        for b in &self.buttons {
            let mesh = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                b.rect,
                Color::from_rgb(40, 40, 40),
            )?;
            canvas.draw(&mesh, DrawParam::default());
            let label = Text::new(TextFragment::new(b.label).scale(PxScale::from(16.0)));
            canvas.draw(&label, DrawParam::default().dest([b.rect.x + 8.0, b.rect.y + 8.0]));
        }

        // HUD
        let status = format!(
            "{} | Zoom: {:.0}px | Vel: {:.0} lpm",
            if self.paused { "PAUSADO" } else { "RODANDO" },
            self.font_size,
            self.lines_per_sec * 60.0
        );
        let status_text = Text::new(TextFragment::new(status).scale(PxScale::from(14.0)));
        canvas.draw(&status_text, DrawParam::default().dest([12.0, 56.0]));

        // Texto: do rodapé para cima (auto-fit: deixa chegar até o topo)
        let (w, h) = ctx.gfx.drawable_size();
        let margin_x = w * 0.05;
        let scale = PxScale::from(self.font_size);
        let y_bottom = h - 20.0;

        let total = self.stream_lines.len() as isize;
        let k = self.current_line.floor() as isize; // última linha “atingida”
        let f = self.current_line - (k as f32);    // 0..1 fração até a próxima

        // calcula automaticamente quantas linhas cabem na janela
        let hud_reserved_px = 80.0; // aprox. altura da barra/labels
        let max_fit = (((h - hud_reserved_px) / self.line_px).ceil() as isize + 2).max(1);

        // mostra desde (k - max_fit + 1) até (k + 1): isso permite a linha chegar no topo
        let start_idx = (k - max_fit + 1).max(0);
        let end_idx = (k + 1).min(total - 1);

        for i in start_idx..=end_idx {
            let dist_lines = (k - i) as f32 + f; // distância do rodapé em múltiplos de line_px
            let y = y_bottom - dist_lines * self.line_px;

            if let Some(line) = self.stream_lines.get(i as usize) {
                if !line.is_empty() && y > -self.line_px && y < (h + self.line_px) {
                    let t = Text::new(TextFragment::new(line).scale(scale));
                    canvas.draw(&t, DrawParam::default().dest([margin_x, y]));
                }
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeat: bool,
    ) -> GameResult {
        use ggez::input::keyboard::KeyCode as K;
        if let Some(code) = input.keycode {
            match code {
                // PLAY/PAUSE: se estava no lead-in, pula direto para a 1ª linha útil
                K::Space => {
                    if self.paused && self.current_line < 0.0 {
                        self.current_line = self.lead_in_lines as f32;
                    }
                    self.paused = !self.paused;
                }
                K::Equals | K::Plus => {
                    self.font_size = (self.font_size + 2.0).min(160.0);
                    self.line_px = self.font_size * 1.20;
                    self.need_relayout = true;
                }
                K::Minus => {
                    self.font_size = (self.font_size - 2.0).max(12.0);
                    self.line_px = self.font_size * 1.20;
                    self.need_relayout = true;
                }
                K::Up => {
                    self.lines_per_sec = (self.lines_per_sec * 60.0 + 5.0).min(400.0) / 60.0;
                }
                K::Down => {
                    self.lines_per_sec = (self.lines_per_sec * 60.0 - 5.0).max(5.0) / 60.0;
                }
                // REINICIAR: pronto para começar na 1ª linha útil (sem esperar lead-in)
                K::R => {
                    self.current_line = self.lead_in_lines as f32;
                    self.paused = true;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _btn: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        let mut clicked: Option<ButtonKind> = None;
        for b in &self.buttons {
            if b.rect.contains([x, y]) { clicked = Some(b.kind); break; }
        }
        match clicked {
            // PLAY/PAUSE no botão: pular lead-in ao iniciar
            Some(ButtonKind::PausePlay) => {
                if self.paused && self.current_line < 0.0 {
                    self.current_line = self.lead_in_lines as f32;
                }
                self.paused = !self.paused;
            }
            Some(ButtonKind::ZoomIn) => {
                self.font_size = (self.font_size + 2.0).min(160.0);
                self.line_px = self.font_size * 1.20;
                self.need_relayout = true;
            }
            Some(ButtonKind::ZoomOut) => {
                self.font_size = (self.font_size - 2.0).max(12.0);
                self.line_px = self.font_size * 1.20;
                self.need_relayout = true;
            }
            Some(ButtonKind::Faster) => {
                self.lines_per_sec = (self.lines_per_sec * 60.0 + 5.0).min(400.0) / 60.0;
            }
            Some(ButtonKind::Slower) => {
                self.lines_per_sec = (self.lines_per_sec * 60.0 - 5.0).max(5.0) / 60.0;
            }
            // Reiniciar: pronto para começar na 1ª linha útil
            Some(ButtonKind::Restart) => {
                self.current_line = self.lead_in_lines as f32;
                self.paused = true;
            }
            None => {}
        }
        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, _w: f32, _h: f32) -> GameResult {
        self.need_relayout = true;
        Ok(())
    }
}

/// Lê arquivo local
fn read_content(path: &str) -> GameResult<String> {
    Ok(std::fs::read_to_string(path)?)
}

fn center_window(ctx: &mut Context) {
    use ggez::winit::dpi::PhysicalPosition;
    let win = ctx.gfx.window();
    if let Some(monitor) = win.current_monitor() {
        let size = monitor.size();
        let ws = win.outer_size();
        let x = ((size.width as i32 - ws.width as i32) / 2).max(0);
        let y = ((size.height as i32 - ws.height as i32) / 2).max(0);
        win.set_outer_position(PhysicalPosition::new(x, y));
    }
}

fn main() -> GameResult {
    let args = Args::parse();
    let raw = read_content(&args.file).expect("Falha ao ler conteúdo");

    let (mut ctx, event_loop) = ContextBuilder::new("teleprompter", "you")
        .window_mode(ggez::conf::WindowMode::default().dimensions(1000.0, 700.0))
        .window_setup(ggez::conf::WindowSetup::default().title("Teleprompter"))
        .build()?;

    center_window(&mut ctx);

    let state = Teleprompter::new(&mut ctx, raw, args.zoom, args.lpm)?;
    event::run(ctx, event_loop, state)
}
