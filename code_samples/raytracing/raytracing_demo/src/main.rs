use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, Copy)]
struct Vetor3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vetor3D {
    fn novo(x: f64, y: f64, z: f64) -> Self {
        Vetor3D { x, y, z }
    }

    fn somar(self, outro: Vetor3D) -> Vetor3D {
        Vetor3D::novo(self.x + outro.x, self.y + outro.y, self.z + outro.z)
    }

    fn produto_escalar(self, outro: Vetor3D) -> f64 {
        self.x * outro.x + self.y * outro.y + self.z * outro.z
    }

    fn normalizar(self) -> Vetor3D {
        let magnitude = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vetor3D::novo(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }

    fn subtrair(self, outro: Vetor3D) -> Vetor3D {
        Vetor3D::novo(self.x - outro.x, self.y - outro.y, self.z - outro.z)
    }

    fn escalar(self, escalar: f64) -> Vetor3D {
        Vetor3D::novo(self.x * escalar, self.y * escalar, self.z * escalar)
    }
}

#[derive(Debug, Clone, Copy)]
struct Raio {
    origem: Vetor3D,
    direcao: Vetor3D,
}

impl Raio {
    fn novo(origem: Vetor3D, direcao: Vetor3D) -> Self {
        Raio { origem, direcao }
    }
}

#[derive(Debug)]
struct Esfera {
    centro: Vetor3D,
    raio: f64,
    cor: (u8, u8, u8),
}

impl Esfera {
    fn interseccao(&self, raio: &Raio) -> Option<f64> {
        let oc = raio.origem.subtrair(self.centro);
        let a = raio.direcao.produto_escalar(raio.direcao);
        let b = 2.0 * oc.produto_escalar(raio.direcao);
        let c = oc.produto_escalar(oc) - self.raio * self.raio;
        let discriminante = b * b - 4.0 * a * c;

        if discriminante < 0.0 {
            None
        } else {
            Some((-b - discriminante.sqrt()) / (2.0 * a))
        }
    }
}

fn calcular_sombra(esfera: &Esfera, ponto: Vetor3D, luz: Vetor3D) -> (u8, u8, u8) {
    let normal = ponto.subtrair(esfera.centro).normalizar();
    let direcao_luz = luz.subtrair(ponto).normalizar();
    let intensidade = normal.produto_escalar(direcao_luz).max(0.0); // Sombreamento Lambertiano
    let (r, g, b) = esfera.cor;
    (
        (r as f64 * intensidade) as u8,
        (g as f64 * intensidade) as u8,
        (b as f64 * intensidade) as u8,
    )
}

fn main() {
    let largura = 800;
    let altura = 800;

    let camera = Vetor3D::novo(0.0, 0.0, -5.0);
    let tamanho_projecao = 2.0;
    let tamanho_canvas = 2.0;

    let esferas = vec![
        Esfera {
            centro: Vetor3D::novo(0.0, 0.0, 2.0),
            raio: 1.0,
            cor: (255, 0, 0),
        },
        Esfera {
            centro: Vetor3D::novo(2.0, 0.0, 4.0),
            raio: 1.0,
            cor: (0, 0, 255),
        },
    ];

    let luz = Vetor3D::novo(-5.0, 5.0, -10.0);

    let mut svg_conteudo = String::new();
    svg_conteudo.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}" style="background-color: black;">"#,
        largura, altura, largura, altura
    ));

    for y in 0..altura {
        for x in 0..largura {
            let canvas_x = (x as f64 / largura as f64) * tamanho_canvas - tamanho_canvas / 2.0;
            let canvas_y = -(y as f64 / altura as f64) * tamanho_canvas + tamanho_canvas / 2.0;

            let direcao = Vetor3D::novo(canvas_x, canvas_y, tamanho_projecao).normalizar();
            let raio = Raio::novo(camera, direcao);

            for esfera in &esferas {
                if let Some(distancia) = esfera.interseccao(&raio) {
                    let ponto_intersecao = raio.origem.somar(raio.direcao.escalar(distancia));
                    let cor = calcular_sombra(esfera, ponto_intersecao, luz);
                    svg_conteudo.push_str(&format!(
                        r#"<rect x="{}" y="{}" width="1" height="1" fill="rgb({}, {}, {})"/>"#,
                        x, y, cor.0, cor.1, cor.2
                    ));
                    break;
                }
            }
        }
    }

    svg_conteudo.push_str("</svg>");

    let mut arquivo = File::create("cena.svg").expect("Erro ao criar arquivo SVG");
    arquivo
        .write_all(svg_conteudo.as_bytes())
        .expect("Erro ao escrever no arquivo SVG");

    println!("Renderização concluída! O arquivo 'cena.svg' foi gerado.");
}
