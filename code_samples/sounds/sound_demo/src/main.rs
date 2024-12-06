use rodio::{source::{SineWave, Source}, OutputStream, Sink};
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use std::time::Duration;

// Estrutura para gerar ruído branco manualmente
struct RuidoBranco {
    duracao: Duration,
    restante: f32,
    taxa: u32,
    gerador: SmallRng, // Gerador de números aleatórios compatível com `Send`
}

impl RuidoBranco {
    fn new(duracao: Duration) -> Self {
        let taxa = 44100; // Taxa de amostragem padrão
        Self {
            duracao,
            restante: duracao.as_secs_f32() * taxa as f32,
            taxa,
            gerador: SmallRng::from_entropy(), // Inicializa o gerador
        }
    }
}

impl Iterator for RuidoBranco {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.restante > 0.0 {
            self.restante -= 1.0;
            Some(self.gerador.gen_range(-0.5..0.5)) // Gera valores menores para ajustar o volume
        } else {
            None
        }
    }
}

impl Source for RuidoBranco {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.taxa
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(self.duracao)
    }
}

// Função para reproduzir som de metralhadora
fn som_metralhadora() {
    let (_fluxo, manipulador_fluxo) = OutputStream::try_default().unwrap();
    let duracao_tiro = Duration::from_millis(40); // Cada tiro é curto (40ms)
    let pausa_entre_tiros = Duration::from_millis(60); // Pausa curta entre tiros

    for _ in 0..15 { // Simula 15 disparos
        let tanque = Sink::try_new(&manipulador_fluxo).unwrap();

        // Cria um disparo misturando frequências graves e ajustando o volume
        let onda_grave = SineWave::new(60.0).take_duration(duracao_tiro).amplify(13.0);
        let onda_aguda = SineWave::new(120.0).take_duration(duracao_tiro).amplify(12.0);
        let combinado = onda_grave.mix(onda_aguda);

        tanque.append(combinado);
        tanque.sleep_until_end();
        std::thread::sleep(pausa_entre_tiros);
    }

    println!("Som de metralhadora reproduzido!");
}

// Função para reproduzir som de explosão
fn som_explosao() {
    let (_fluxo, manipulador_fluxo) = OutputStream::try_default().unwrap();

    // Fase inicial: impacto grave
    let tanque = Sink::try_new(&manipulador_fluxo).unwrap();
    let onda_impacto = SineWave::new(50.0)
        .take_duration(Duration::from_millis(300))
        .amplify(5.0); // Volume ajustado
    tanque.append(onda_impacto);

    // Adiciona ruído branco para simular deslocamento de ar
    let ruido = RuidoBranco::new(Duration::from_millis(500)).amplify(3.0); // Volume ajustado
    tanque.append(ruido);

    tanque.sleep_until_end();

    // Decaimento: frequências aleatórias com volumes decrescentes
    for (frequencia, amplitude) in [(300.0, 4.0), (200.0, 3.0), (150.0, 2.0), (100.0, 1.0)] {
        let tanque = Sink::try_new(&manipulador_fluxo).unwrap();
        let onda_decay = SineWave::new(frequencia)
            .take_duration(Duration::from_millis(200))
            .amplify(amplitude);
        tanque.append(onda_decay);
        tanque.sleep_until_end();
    }

    println!("Som de explosão reproduzido!");
}

// Função principal
fn main() {
    println!("Reproduzindo som de metralhadora...");
    som_metralhadora();

    println!("Reproduzindo som de explosão...");
    som_explosao();
}
