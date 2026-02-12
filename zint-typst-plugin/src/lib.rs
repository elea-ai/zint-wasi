use wasm_minimal_protocol::*;
use zint_wasm_rs::{options::Options, symbol::Symbol};

initiate_protocol!();

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("provided invalid options: {0}")]
    BadOptions(
        #[from]
        #[source]
        ciborium::de::Error<std::io::Error>,
    ),
    #[error(transparent)]
    ZintEncoding(#[from] zint_wasm_rs::error::Error),
}
type Result<T> = std::result::Result<T, crate::Error>;

#[wasm_func]
pub fn gen_with_options(options: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    let options: Options = ciborium::from_reader(options)?;
    let symbol = Symbol::new(&options);

    if 1 == options.input_mode.unwrap().as_i32() {
        std::str::from_utf8(data).map_err(|_| {
            Error::ZintEncoding(zint_wasm_rs::error::Error::InvalidInput("data is not valid UTF-8".into()))
        })?;
    }

    let length = i32::try_from(data.len()).map_err(|_| {
        Error::ZintEncoding(zint_wasm_rs::error::Error::InvalidInput("data too long".into()))
    })?;

    let svg = symbol.encode_svg(data, length, 0)?;
    Ok(svg.into_bytes())
}
