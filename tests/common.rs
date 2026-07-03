#[allow(dead_code)]
pub mod util {
    use calamine::{Data, Reader, open_workbook};
    use csv::Writer;
    use rust_decimal::Decimal;
    use serde::{Deserialize, Serialize};
    use std::path::Path;
    use yeepay_core::YeepayConfig;

    pub fn string_to_decimal<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<Decimal>().map_err(serde::de::Error::custom)
    }

    pub fn config() -> YeepayConfig {
        dotenvy::dotenv().ok();

        let app_key = std::env::var("IS_APPKEY").unwrap_or_default();
        let merchant_no = std::env::var("IS_MERCHANT_NO").unwrap_or_default();
        let public_key = std::env::var("IS_PUBLIC_KEY").unwrap_or_default();
        let private_key = std::env::var("IS_PRIVATE_KEY").unwrap_or_default();
        YeepayConfig::new(app_key, merchant_no, public_key, private_key).with_debug()
    }

    pub trait ExcelReaderTrait {
        fn parse_row(row: &[Data]) -> Result<Option<Self>, Box<dyn std::error::Error>>
        where
            Self: Sized;

        fn read<P: AsRef<Path>, T>(path: P) -> Result<Vec<T>, Box<dyn std::error::Error>>
        where
            T: ExcelReaderTrait,
        {
            let mut workbook: calamine::Xlsx<_> =
                open_workbook(path).map_err(|_| "Invalid xlsx file")?;
            let mut result = Vec::new();

            if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
                for (i, row) in range.rows().enumerate() {
                    if i == 0 {
                        // Skip header row
                        continue;
                    }
                    let value = T::parse_row(&row)?;
                    if let Some(value) = value {
                        result.push(value);
                    }
                }
            }

            Ok(result)
        }
    }

    pub fn read_xlsx<T>(path: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: ExcelReaderTrait,
    {
        T::read(path)
    }

    pub fn read_csv<R>(path: &str) -> Result<Vec<R>, Box<dyn std::error::Error>>
    where
        R: for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        let mut rdr = csv::Reader::from_path(path)?;
        let mut result = vec![];
        for row in rdr.deserialize().into_iter() {
            if let Ok(row) = row {
                result.push(row)
            }
        }
        Ok(result)
    }

    pub fn write_csv<T>(path: &str, data: &[T]) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let mut wtr = Writer::from_path(path)?;
        for record in data {
            wtr.serialize(record)?;
        }
        wtr.flush()?;
        Ok(())
    }
}
