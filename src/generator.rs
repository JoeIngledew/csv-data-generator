use crate::errors::RegexError;
use crate::models::{CsvOutput, FieldType, InputDefItem};
use fake::faker::company::raw::Bs;
use fake::{
    Fake, Faker, faker::chrono::raw::Date, faker::name::en::Name, locales::EN, uuid::UUIDv4,
};
use rand::{Rng, SeedableRng};
use rand_regex::Regex;
use rand_xorshift::XorShiftRng;

fn gen_pattern_string(pattern: &str, rng: &mut XorShiftRng) -> Result<String, RegexError> {
    let generator = Regex::compile(pattern, 10).map_err(|e| RegexError(e))?;
    let result: String = rng.sample(&generator);
    Ok(result)
}

fn generate_inner(
    count: u32,
    defs: &[InputDefItem],
) -> Result<Vec<(String, Vec<String>)>, RegexError> {
    let mut generated: Vec<(String, Vec<String>)> = Vec::new();
    let mut rng = XorShiftRng::from_os_rng();
    let mut sortable_defs = defs.to_vec();
    sortable_defs.sort_by_key(|d| d.order.unwrap_or(usize::MAX));
    //sortable_defs.reverse();
    for def in sortable_defs {
        let mut values = Vec::new();
        for _ in 0..count {
            let value = match def.field_type {
                FieldType::String => Faker.fake::<String>(),
                FieldType::Number => Faker.fake::<f64>().to_string(),
                FieldType::Boolean => Faker.fake::<bool>().to_string(),
                FieldType::Date => Date(EN).fake::<String>(),
                FieldType::Uuid => UUIDv4.fake::<String>(),
                FieldType::PatternString(ref pattern) => gen_pattern_string(pattern, &mut rng)?,
                FieldType::Name => Name().fake::<String>(),
                FieldType::CompanyBrand => Bs(EN).fake::<String>(),
                FieldType::Selection(ref selection) => {
                    let index = rng.random_range(0..selection.len());
                    selection[index].clone()
                }
                FieldType::RangeInteger(ref range) => {
                    let value = rng.random_range(range.min..range.max);
                    value.to_string()
                }
            };
            values.push(value);
        }
        generated.push((def.field_name.clone(), values));
    }
    Ok(generated)
}

pub fn generate(count: u32, defs: &[InputDefItem]) -> Result<CsvOutput, RegexError> {
    let generated = generate_inner(count, defs)?;
    let header: Vec<String> = generated.iter().map(|(name, _)| name.clone()).collect();
    //let header = generated.keys().cloned().collect();
    dbg!(&header);
    let rows: Vec<Vec<String>> = generated.iter().map(|(_, vs)| vs).cloned().collect();
    let mut csv_rows: Vec<Vec<String>> = Vec::new();
    for i in 0..count {
        let mut csv_row: Vec<String> = Vec::new();
        for row in rows.iter() {
            let item = row[i as usize].clone();
            csv_row.push(item);
        }
        csv_rows.push(csv_row);
    }
    Ok(CsvOutput {
        header,
        rows: csv_rows,
    })
}
