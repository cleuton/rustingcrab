use delaunator::{Point as PontoDelaunay, triangulate};
use std::collections::HashSet;

/// Ponto 2D com coordenadas X e Y (armazenado como inteiro para Eq + Hash)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Ponto {
    pub x: i64,
    pub y: i64,
}

impl Ponto {
    fn novo(x: f64, y: f64) -> Self {
        Self {
            x: (x * 1000.0).round() as i64,
            y: (y * 1000.0).round() as i64,
        }
    }

    /// Converte para f64 novamente
    fn para_f64(&self) -> (f64, f64) {
        ((self.x as f64) / 1000.0, (self.y as f64) / 1000.0)
    }
}

/// Círculo com centro e raio
#[derive(Debug, Clone, Copy)]
struct Circulo {
    pub centro: Ponto,
    pub raio: f64,
}

impl Circulo {
    fn novo(centro: Ponto, raio: f64) -> Self {
        Self { centro, raio }
    }

    /// Calcula os pontos de interseção com outro círculo
    fn intersecta(&self, outro: &Self) -> Vec<Ponto> {
        let (x1, y1) = self.centro.para_f64();
        let r1 = self.raio;
        let (x2, y2) = outro.centro.para_f64();
        let r2 = outro.raio;

        let dx = x2 - x1;
        let dy = y2 - y1;
        let d = ((dx * dx + dy * dy) as f64).sqrt();

        // Verifica se há interseção
        if d > r1 + r2 || d < (r1 - r2).abs() || d == 0.0 {
            return vec![];
        }

        // Fórmula de interseção
        let a = (r1 * r1 - r2 * r2 + d * d) / (2.0 * d);
        let h = (r1 * r1 - a * a).sqrt();

        let x0 = x1 + (a * dx) / d;
        let y0 = y1 + (a * dy) / d;

        let rx = (-h * dy) / d;
        let ry = (h * dx) / d;

        let mut pontos = vec![
            Ponto::novo(x0 + rx, y0 + ry),
            Ponto::novo(x0 - rx, y0 - ry),
        ];

        pontos.retain(|p| {
            let (x, y) = p.para_f64();
            x.is_finite() && y.is_finite()
        });

        pontos
    }
}

/// Construtor da mandala
struct ConstrutorMandala {
    centro: Ponto,
    num_camadas: usize,
    circulos_por_camada: usize,
    raio_base: f64,
    incremento_raio: f64,
    funcao_raio_circulo: Box<dyn Fn(usize) -> f64>,
}

impl ConstrutorMandala {
    fn novo(centro: Ponto) -> Self {
        Self {
            centro,
            num_camadas: 3,
            circulos_por_camada: 8,
            raio_base: 50.0,
            incremento_raio: 30.0,
            funcao_raio_circulo: Box::new(|_| 40.0),
        }
    }

    fn num_camadas(mut self, n: usize) -> Self {
        self.num_camadas = n;
        self
    }

    fn circulos_por_camada(mut self, n: usize) -> Self {
        self.circulos_por_camada = n;
        self
    }

    fn raio_base(mut self, r: f64) -> Self {
        self.raio_base = r;
        self
    }

    fn incremento_raio(mut self, inc: f64) -> Self {
        self.incremento_raio = inc;
        self
    }

    fn funcao_raio_circulo<F>(mut self, f: F) -> Self
    where
        F: 'static + Fn(usize) -> f64,
    {
        self.funcao_raio_circulo = Box::new(f);
        self
    }

    /// Constrói a mandala: gera triângulos a partir das interseções
    fn construir(&self) -> Vec<Triangulo> {
        let mut todos_pontos = HashSet::new();

        let mut raio_atual = self.raio_base;

        // Gera as camadas
        let mut camadas = Vec::new();
        for indice_camada in 0..self.num_camadas {
            let mut circulos_camada = Vec::new();
            let raio_circulo = (self.funcao_raio_circulo)(indice_camada);

            for i in 0..self.circulos_por_camada {
                let angulo = 2.0 * std::f64::consts::PI * (i as f64) / (self.circulos_por_camada as f64);
                let x = self.centro.para_f64().0 + raio_atual * angulo.cos();
                let y = self.centro.para_f64().1 + raio_atual * angulo.sin();
                circulos_camada.push(Circulo::novo(Ponto::novo(x, y), raio_circulo));
            }

            camadas.push(circulos_camada);
            raio_atual += self.incremento_raio;
        }

        // Calcula interseções entre camadas adjacentes
        let mut total_intersecoes = 0;
        for indice_camada in 0..(self.num_camadas - 1) {
            for circulo_a in &camadas[indice_camada] {
                for circulo_b in &camadas[indice_camada + 1] {
                    let intersecoes = circulo_a.intersecta(circulo_b);
                    total_intersecoes += intersecoes.len();
                    for pt in intersecoes {
                        todos_pontos.insert(pt);
                    }
                }
            }
        }

        eprintln!("Pontos únicos gerados: {}", todos_pontos.len());
        eprintln!("Total de interseções calculadas: {}", total_intersecoes);

        if todos_pontos.is_empty() {
            eprintln!("Nenhum ponto de interseção encontrado. Ajuste os raios ou espaçamento.");
            return vec![];
        }

        // Converte para f64
        let pontos_f64: Vec<(f64, f64)> = todos_pontos
            .into_iter()
            .map(|pt| pt.para_f64())
            .collect();

        if pontos_f64.len() < 3 {
            eprintln!("Menos de 3 pontos — não é possível triangulação.");
            return vec![];
        }

        // Prepara para triangulação
        let pontos_delaunay: Vec<PontoDelaunay> = pontos_f64
            .iter()
            .map(|&(x, y)| PontoDelaunay { x, y })
            .collect();

        let triangulacao = triangulate(&pontos_delaunay);

        // Monta triângulos
        let mut triangulos = Vec::new();
        for window in triangulacao.triangles.chunks(3) {
            if window.len() == 3 {
                triangulos.push(Triangulo {
                    a: Ponto::novo(pontos_f64[window[0]].0, pontos_f64[window[0]].1),
                    b: Ponto::novo(pontos_f64[window[1]].0, pontos_f64[window[1]].1),
                    c: Ponto::novo(pontos_f64[window[2]].0, pontos_f64[window[2]].1),
                });
            }
        }

        triangulos
    }
}

/// Triângulo formado por três pontos
#[derive(Debug, Clone)]
struct Triangulo {
    pub a: Ponto,
    pub b: Ponto,
    pub c: Ponto,
}

// Função principal — entrada do programa
fn main() {
    let centro = Ponto::novo(400.0, 400.0);

    let mandala = ConstrutorMandala::novo(centro)
        .num_camadas(5)
        .circulos_por_camada(16)
        .raio_base(80.0)
        .incremento_raio(40.0)
        .funcao_raio_circulo(|_| 50.0)
        .construir();

    println!("Gerados {} triângulos", mandala.len());

    for (i, tri) in mandala.iter().take(5).enumerate() {
        let (ax, ay) = tri.a.para_f64();
        let (bx, by) = tri.b.para_f64();
        let (cx, cy) = tri.c.para_f64();
        println!(
            "Triângulo {}: ({:.1},{:.1}), ({:.1},{:.1}), ({:.1},{:.1})",
            i + 1,
            ax, ay,
            bx, by,
            cx, cy
        );
    }

    salvar_como_svg(&mandala, 800.0, 800.0, "mandala.svg");
}

/// Salva os triângulos como um arquivo SVG visível
fn salvar_como_svg(triangulos: &[Triangulo], largura: f64, altura: f64, nome_arquivo: &str) {
    use svg::node::element::path::Data;
    use svg::node::element::{Circle, Path, Rectangle, SVG};

    let mut documento = SVG::new()
        .set("width", largura)
        .set("height", altura)
        .set("viewBox", format!("0 0 {} {}", largura, altura))
        .add(Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "black"));

    for tri in triangulos {
        let (ax, ay) = tri.a.para_f64();
        let (bx, by) = tri.b.para_f64();
        let (cx, cy) = tri.c.para_f64();

        if ![ax, ay, bx, by, cx, cy].iter().all(|&v| v.is_finite()) {
            continue;
        }

        let dados = Data::new()
            .move_to((ax, ay))
            .line_to((bx, by))
            .line_to((cx, cy))
            .close();

        let caminho = Path::new()
            .set("fill", "none")
            .set("stroke", "white")
            .set("stroke-width", 1.0)
            .set("d", dados);

        documento = documento.add(caminho);
    }

    // Marca o centro
    documento = documento.add(
        Circle::new()
            .set("cx", 400)
            .set("cy", 400)
            .set("r", 3)
            .set("fill", "lime"),
    );

    svg::save(nome_arquivo, &documento).unwrap();
    eprintln!("Arquivo SVG salvo: {}", nome_arquivo);
}