use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::Bound;              // traz o tipo Bound<'py, T>
use std::time::{SystemTime, UNIX_EPOCH};

const EPOCA: u64          = 1_577_836_800_000;
const BITS_ID: u8         = 10;
const BITS_SEQ: u8        = 12;
const MAX_ID: u64         = (1 << BITS_ID) - 1;
const MAX_SEQ: u64        = (1 << BITS_SEQ) - 1;
const SHIFT_ID: u8        = BITS_SEQ;
const SHIFT_TIMESTAMP: u8 = BITS_SEQ + BITS_ID;

/// Gerador de IDs Snowflake de 64 bits
#[pyclass]
pub struct GeradorSnowflake {
    id_trab: u64,
    seq:     u64,
    last_ts: u64,
}

#[pymethods]
impl GeradorSnowflake {
    #[new]
    fn novo(id_trab: u64) -> Self {
        assert!(id_trab <= MAX_ID, "id_trab cabe em {} bits", BITS_ID);
        GeradorSnowflake { id_trab, seq: 0, last_ts: 0 }
    }

    fn proximo_id(&mut self) -> PyResult<u64> {
        let mut ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        if ts < self.last_ts {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("clock regressou {} ms", self.last_ts - ts),
            ));
        }
        if ts == self.last_ts {
            self.seq = (self.seq + 1) & MAX_SEQ;
            if self.seq == 0 {
                while ts <= self.last_ts {
                    ts = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;
                }
            }
        } else {
            self.seq = 0;
        }
        self.last_ts = ts;
        Ok(((ts - EPOCA) << SHIFT_TIMESTAMP)
           | (self.id_trab << SHIFT_ID)
           | self.seq)
    }
}

/// perceba que aqui só existe um parâmetro: o módulo em si, que vira
/// um Bound<'_, PyModule>, e é nele que o add_class está disponível :contentReference[oaicite:0]{index=0}
#[pymodule]
fn gerador_snowflake(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GeradorSnowflake>()?;
    Ok(())
}
