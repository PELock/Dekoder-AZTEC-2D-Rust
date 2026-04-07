/******************************************************************************
 *
 * Dekoder kodow AZTEC 2D z dowodow rejestracyjnych interfejs Web API
 *
 * Wersja         : AZTecDecoder v2.0
 * Jezyk          : Rust
 * Zaleznosci     : reqwest, serde_json, thiserror, tokio
 * Autor          : Bartosz Wójcik (support@pelock.com)
 * Strona domowa  : https://www.dekoderaztec.pl | https://www.pelock.com
 *
 *****************************************************************************/

//! # aztec-decoder
//!
//! Biblioteka programistyczna pozwalająca na dekodowanie danych z dowodów
//! rejestracyjnych pojazdów samochodowych zapisanych w formie kodu AZTEC 2D.
//!
//! ## Szybki start
//!
//! ```rust,no_run
//! use aztec_decoder::AZTecDecoder;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let decoder = AZTecDecoder::new("ABCD-ABCD-ABCD-ABCD");
//!
//!     let result = decoder.decode_image_from_file("zdjecie-dowodu.jpg").await?;
//!
//!     if result["Status"] == true {
//!         println!("{}", serde_json::to_string_pretty(&result)?);
//!     }
//!
//!     Ok(())
//! }
//! ```

use std::path::Path;

use reqwest::multipart;
use serde_json::Value;
use thiserror::Error;

const API_URL: &str = "https://www.pelock.com/api/aztec-decoder/v1";

/// Błędy zwracane przez [`AZTecDecoder`].
#[derive(Debug, Error)]
pub enum AZTecError {
    /// Klucz API jest pusty.
    #[error("brak klucza API")]
    EmptyApiKey,

    /// Nie udało się odczytać pliku.
    #[error("błąd odczytu pliku: {0}")]
    FileRead(#[from] std::io::Error),

    /// Błąd komunikacji z serwerem Web API.
    #[error("błąd żądania HTTP: {0}")]
    Request(#[from] reqwest::Error),

    /// Serwer zwrócił odpowiedź, której nie można sparsować jako JSON.
    #[error("nieprawidłowa odpowiedź JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
}

/// Klient dekodera kodów AZTEC 2D z dowodów rejestracyjnych (Web API).
///
/// Komunikuje się z usługą Web API pod adresem
/// `https://www.pelock.com/api/aztec-decoder/v1`.
///
/// # Przykład
///
/// ```rust,no_run
/// use aztec_decoder::AZTecDecoder;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let decoder = AZTecDecoder::new("ABCD-ABCD-ABCD-ABCD");
/// let result = decoder.decode_text("ggMAANtYAAJD...").await?;
/// println!("{}", serde_json::to_string_pretty(&result)?);
/// # Ok(())
/// # }
/// ```
pub struct AZTecDecoder {
    api_key: String,
    client: reqwest::Client,
}

impl AZTecDecoder {
    /// Tworzy nową instancję dekodera.
    ///
    /// # Argumenty
    ///
    /// * `api_key` – klucz do usługi Web API
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: reqwest::Client::new(),
        }
    }

    /// Dekoduje zaszyfrowaną wartość tekstową do wyjściowej struktury JSON.
    ///
    /// Wysyła polecenie `decode-text` wraz z podanym tekstem (np. odczytanym
    /// skanerem w formacie Base64) do serwera Web API.
    ///
    /// # Argumenty
    ///
    /// * `text` – odczytana wartość kodu AZTEC 2D w formie ASCII
    ///
    /// # Błędy
    ///
    /// Zwraca [`AZTecError`] w przypadku pustego klucza API, błędu sieciowego
    /// lub nieprawidłowej odpowiedzi JSON.
    pub async fn decode_text(&self, text: &str) -> Result<Value, AZTecError> {
        let form = multipart::Form::new()
            .text("key", self.api_key.clone())
            .text("command", "decode-text")
            .text("text", text.to_owned());

        self.post_request(form).await
    }

    /// Dekoduje zaszyfrowaną wartość tekstową ze wskazanego pliku do
    /// wyjściowej struktury JSON.
    ///
    /// Odczytuje zawartość pliku jako UTF-8 i przekazuje ją do
    /// [`decode_text`](Self::decode_text).
    ///
    /// # Argumenty
    ///
    /// * `path` – ścieżka do pliku z odczytaną wartością kodu AZTEC 2D
    ///
    /// # Błędy
    ///
    /// Zwraca [`AZTecError::FileRead`] jeśli plik nie istnieje lub nie można
    /// go odczytać, oraz pozostałe warianty [`AZTecError`] w przypadku błędów
    /// komunikacji z API.
    pub async fn decode_text_from_file(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<Value, AZTecError> {
        let data = tokio::fs::read_to_string(path).await?;
        self.decode_text(&data).await
    }

    /// Dekoduje zaszyfrowaną wartość zakodowaną w obrazku PNG lub JPG/JPEG
    /// do wyjściowej struktury JSON.
    ///
    /// Wysyła plik graficzny jako formularz multipart z poleceniem
    /// `decode-image` do serwera Web API.
    ///
    /// # Argumenty
    ///
    /// * `path` – ścieżka do obrazka z kodem AZTEC 2D
    ///
    /// # Błędy
    ///
    /// Zwraca [`AZTecError::FileRead`] jeśli plik nie istnieje lub nie można
    /// go odczytać, oraz pozostałe warianty [`AZTecError`] w przypadku błędów
    /// komunikacji z API.
    pub async fn decode_image_from_file(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<Value, AZTecError> {
        if self.api_key.is_empty() {
            return Err(AZTecError::EmptyApiKey);
        }

        let path = path.as_ref();
        let file_bytes = tokio::fs::read(path).await?;

        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();

        let file_part = multipart::Part::bytes(file_bytes).file_name(file_name);

        let form = multipart::Form::new()
            .text("key", self.api_key.clone())
            .text("command", "decode-image")
            .part("image", file_part);

        self.post_request(form).await
    }

    /// Wysyła żądanie POST (multipart) do serwera Web API i zwraca
    /// odpowiedź jako [`serde_json::Value`].
    async fn post_request(&self, form: multipart::Form) -> Result<Value, AZTecError> {
        if self.api_key.is_empty() {
            return Err(AZTecError::EmptyApiKey);
        }

        let response = self
            .client
            .post(API_URL)
            .multipart(form)
            .send()
            .await?;

        let text = response.text().await?;
        let json: Value = serde_json::from_str(&text)?;

        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_decoder_stores_api_key() {
        let decoder = AZTecDecoder::new("test-key");
        assert_eq!(decoder.api_key, "test-key");
    }

    #[tokio::test]
    async fn empty_api_key_returns_error() {
        let decoder = AZTecDecoder::new("");
        let result = decoder.decode_text("test").await;
        assert!(matches!(result, Err(AZTecError::EmptyApiKey)));
    }

    #[tokio::test]
    async fn missing_file_returns_error() {
        let decoder = AZTecDecoder::new("test-key");
        let result = decoder
            .decode_text_from_file("nieistniejacy-plik.txt")
            .await;
        assert!(matches!(result, Err(AZTecError::FileRead(_))));
    }

    #[tokio::test]
    async fn missing_image_returns_error() {
        let decoder = AZTecDecoder::new("test-key");
        let result = decoder
            .decode_image_from_file("nieistniejacy-obrazek.png")
            .await;
        assert!(matches!(result, Err(AZTecError::FileRead(_))));
    }
}
