use calamine::{Data, Reader, Xlsx, open_workbook};
use std::path::Path;

pub struct ExcelReader;

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

#[warn(unused)]
pub fn read_xlsx<T>(path: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: ExcelReaderTrait,
{
    T::read(path)
}
