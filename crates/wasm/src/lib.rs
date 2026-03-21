use wasm_bindgen::prelude::*;
use pc_proof_core::{parser, tableau, sequent};

/// Parse a formula and return its string representation (for debugging)
#[wasm_bindgen]
pub fn parse(input: &str) -> Result<String, JsError> {
    let expr = parser::parse(input).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(expr.to_string())
}

/// Check whether `input` is a tautology using the tableau method.
/// Returns JSON: TautologyOutcome
#[wasm_bindgen]
pub fn tableau_tautology(input: &str) -> Result<JsValue, JsError> {
    let expr = parser::parse(input).map_err(|e| JsError::new(&e.to_string()))?;
    let outcome = tableau::check_tautology(&expr);
    serde_wasm_bindgen::to_value(&outcome).map_err(|e| JsError::new(&e.to_string()))
}

/// Check whether `input` is satisfiable using the tableau method.
/// Returns JSON: SatOutcome
#[wasm_bindgen]
pub fn tableau_satisfiable(input: &str) -> Result<JsValue, JsError> {
    let expr = parser::parse(input).map_err(|e| JsError::new(&e.to_string()))?;
    let outcome = tableau::check_satisfiable(&expr);
    serde_wasm_bindgen::to_value(&outcome).map_err(|e| JsError::new(&e.to_string()))
}

/// Check whether `input` is a tautology using the sequent calculus.
/// Returns JSON: TautologyOutcome
#[wasm_bindgen]
pub fn sequent_tautology(input: &str) -> Result<JsValue, JsError> {
    let expr = parser::parse(input).map_err(|e| JsError::new(&e.to_string()))?;
    let outcome = sequent::check_tautology(&expr);
    serde_wasm_bindgen::to_value(&outcome).map_err(|e| JsError::new(&e.to_string()))
}
