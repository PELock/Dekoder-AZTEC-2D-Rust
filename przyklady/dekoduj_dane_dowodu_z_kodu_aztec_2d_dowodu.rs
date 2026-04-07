//!
//! Dekoduj dane dowodu rejestracyjnego z kodu AZTEC 2D dowodu.
//!
//! Uruchomienie:
//!
//! ```bash
//! cargo run --example dekoduj_dane_dowodu_z_kodu_aztec_2d_dowodu
//! ```

use aztec_decoder::AZTecDecoder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // inicjalizuj dekoder (używamy naszego klucza licencyjnego do inicjalizacji)
    let decoder = AZTecDecoder::new("ABCD-ABCD-ABCD-ABCD");

    // dekoduj dane bezpośrednio z pliku graficznego ze zdjęciem kodu AZTEC 2D
    // i zwróć wynik jako strukturę JSON
    let result = decoder
        .decode_image_from_file(r"C:\zdjecie-kodu-aztec-2d.png")
        .await?;

    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}
