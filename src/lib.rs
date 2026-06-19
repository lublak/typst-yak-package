use ciborium_io::Write as _;
use jaq_all::{data, fmts, load};
use wasm_minimal_protocol::*;

initiate_protocol!();

#[wasm_func]
fn jq(data: &[u8], filter: &[u8]) -> Result<Vec<u8>, String> {
    let filter = std::str::from_utf8(filter)
        .map_err(|e| format!("failed to parse filter: {}", e.to_string()))?;

    let filter = data::compile(&filter).map_err(|frs| {
        frs.iter()
            .map(|fr| format!("{}", load::FileReportsDisp::new(fr)))
            .collect::<Vec<String>>()
            .join("\n")
    })?;
    let input = Box::new(fmts::read::cbor::parse_many(data));
    let runner = Default::default();
    let vars = Default::default();

    let mut data = Vec::new();
    let mut size = 0;

    data::run(
        &runner,
        &filter,
        vars,
        input,
        |e| e,
        |v| {
            let v = jaq_all::jaq_core::unwrap_valr(v).map_err(|e| e.to_string())?;
            size = size + 1;
            fmts::write::cbor::write(&mut data, &v).map_err(|e| e.to_string())
        },
    )?;

    let mut result = Vec::new();
    let mut encoder = ciborium_ll::Encoder::from(&mut result);
    encoder
        .push(ciborium_ll::Header::Array(Some(size)))
        .map_err(|e| e.to_string())?;
    encoder.write_all(&data).map_err(|e| e.to_string())?;
    encoder.flush().map_err(|e| e.to_string())?;

    Ok(result)
}
