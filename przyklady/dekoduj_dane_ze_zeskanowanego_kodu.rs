//!
//! Dekoduj dane z odczytanego ciągu znaków (np. ze skanera ręcznego).
//!
//! Uruchomienie:
//!
//! ```bash
//! cargo run --example dekoduj_dane_ze_zeskanowanego_kodu
//! ```

use aztec_decoder::AZTecDecoder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // inicjalizuj dekoder (używamy naszego klucza licencyjnego do inicjalizacji)
    let decoder = AZTecDecoder::new("ABCD-ABCD-ABCD-ABCD");

    // dekoduj dane z odczytanego już ciągu znaków (np. wykorzystując skaner ręczny)
    // odczytane dane są w formacie Base64
    //
    // zakodowane dane z dowodu rejestracyjnego
    let sz_value = "ggMAANtYAAJD...";

    let result = decoder.decode_text(sz_value).await?;

    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}
