use memmap2::{MmapMut, MmapOptions};
use std::fs::{File, OpenOptions};
use std::io::{Result, Seek, SeekFrom, Write};

/// Tamanho da página em bytes
const PAGE_SIZE: usize = 4096;

/// Gerenciador de páginas
struct PageManager {
    mmap: MmapMut,
    total_pages: usize,
}

impl PageManager {
    /// Cria ou abre um arquivo e mapeia em memória
    fn new(file_path: &str, file_size: usize) -> Result<Self> {
        // Abre ou cria o arquivo
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        // Garante que o arquivo tenha o tamanho desejado
        file.set_len(file_size as u64)?;

        // Cria o mapeamento de memória
        let mmap = unsafe { MmapOptions::new().map_mut(&file)? };
        let total_pages = file_size / PAGE_SIZE;

        Ok(Self { mmap, total_pages })
    }

    /// Retorna uma referência a uma página específica (como slice mutável)
    fn get_page_mut(&mut self, page_number: usize) -> Option<&mut [u8]> {
        if page_number >= self.total_pages {
            return None;
        }
        let start = page_number * PAGE_SIZE;
        let end = start + PAGE_SIZE;
        Some(&mut self.mmap[start..end])
    }

    /// Exemplo de escrita em uma página: escreve um array de bytes na página informada
    fn write_to_page(&mut self, page_number: usize, data: &[u8]) -> Result<()> {
        let page = self
            .get_page_mut(page_number)
            .expect("Número de página inválido");
        let len = data.len().min(PAGE_SIZE);
        page[..len].copy_from_slice(&data[..len]);
        Ok(())
    }

    /// Sincroniza (flush) o mapeamento para o disco
    fn flush(&mut self) -> Result<()> {
        self.mmap.flush()
    }
}

fn main() -> Result<()> {
    // Exemplo: cria um arquivo de 1 MB e divide em páginas de 4KB.
    let file_size = 1024 * 1024; // 1 MB
    let mut page_manager = PageManager::new("database_pages.dat", file_size)?;

    // Suponha que você queira escrever na página 10
    let data = "Exemplo de dados na página 10".as_bytes();
    page_manager.write_to_page(10, data)?;

    // Leitura simples: obtém a página 10 e imprime os primeiros bytes
    if let Some(page) = page_manager.get_page_mut(10) {
        println!(
            "Dados na página 10: {}",
            String::from_utf8_lossy(&page[..data.len()])
        );
    }

    // Sincroniza as alterações para o disco
    page_manager.flush()?;

    Ok(())
}
