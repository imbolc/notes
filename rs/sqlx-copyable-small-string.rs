//! Small Postgres strings as copyable Rust array based types
//! ```cargo
//! edition = "2021"
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! sqlx = { version = "0.8", features = ["runtime-tokio-native-tls", "postgres"] }
//! ```
#[derive(Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(type_name = "varchar")]
pub struct CountryCode([u8; 2]);

impl TryFrom<&str> for CountryCode {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        for c in s.chars() {
            if !c.is_ascii() {
                return Err(format!("Non ASCII country code: {s}"));
            }
            if !c.is_uppercase() {
                return Err(format!("Non uppercase country code: {s}"));
            }
        }

        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return Err(format!("Non 2-digits country code: {s}"));
        }

        let mut arr = [0u8; 2];
        arr.copy_from_slice(bytes);
        Ok(Self(arr))
    }
}

impl std::fmt::Display for CountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(self.0.as_slice()).unwrap_or_default()
        )
    }
}

impl std::fmt::Debug for CountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CountryCode")
            .field(&self.to_string())
            .finish()
    }
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;
    let country_code: CountryCode = "US".try_into().unwrap();

    let serialized = sqlx::query_scalar!("SELECT $1::varchar", country_code as _)
        .fetch_one(&db)
        .await?
        .unwrap();
    assert_eq!(serialized, "US");

    let deserialized = sqlx::query_scalar!(
        r#"
        SELECT 'US'::varchar as "code!: CountryCode"
        "#
    )
    .fetch_one(&db)
    .await?;
    assert_eq!(deserialized, country_code);

    Ok(())
}
