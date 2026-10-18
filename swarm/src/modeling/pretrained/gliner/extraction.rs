//! This file reexports the fitting method for a text against some schema

use std::path::Path;
pub fn fit(input: &Path, out: &Path, cfg: &Config) -> Result<()> {
    todo!();
    let text = std::fs::read_to_string(input).unwrap();
    let model = GLiNER2::load(&cfg.model_path, &Device::Cpu, DType::F32)?;
    let schema = gliner_rs::Schema::new()
        .entities(cfg.schema.entities.clone())
        .relations(cfg.schema.relations.clone());
    let mut triples = Vec::new();

    // clean up the text
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let result = model.extract(line, &schema, &ExtractOptions::default())?; // GLiNER extraction
        triples.extend(parse_triples(&result)); // parse it to the rotatE
    }
    // write to disk, so the python rotatE implementation can read it
    Ok(())
}
