use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

#[warn(unused)]
pub fn read_csv<R>(path: &str) -> Result<Vec<R>, Box<dyn std::error::Error>>
where
    R: for<'de> Deserialize<'de> + std::fmt::Debug,
{
    let mut rdr = Reader::from_path(path)?;
    let mut result = vec![];
    for row in rdr.deserialize().into_iter() {
        if let Ok(row) = row {
            result.push(row)
        }
    }
    Ok(result)
}

#[warn(unused)]
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
