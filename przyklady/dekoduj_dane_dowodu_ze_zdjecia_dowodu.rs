//!
//! Dekoduj dane dowodu rejestracyjnego bezpośrednio ze zdjęcia dowodu.
//!
//! Uruchomienie:
//!
//! ```bash
//! cargo run --example dekoduj_dane_dowodu_ze_zdjecia_dowodu
//! ```

use aztec_decoder::AZTecDecoder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // inicjalizuj dekoder (używamy naszego klucza licencyjnego do inicjalizacji)
    let decoder = AZTecDecoder::new("ABCD-ABCD-ABCD-ABCD");

    // dekoduj dane bezpośrednio z pliku graficznego,
    // zwróć wynik jako strukturę JSON
    let result = decoder
        .decode_image_from_file(r"C:\zdjecie-dowodu.jpg")
        .await?;

    // czy udało się zdekodować dane?
    if result["Status"] == true {
        // wyświetl rozkodowane dane (są zapisane jako struktura JSON)
        println!("{}", serde_json::to_string_pretty(&result)?);
    }

    Ok(())
}
