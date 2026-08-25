use std::env;
use std::error::Error;

use road_name_normalizer::{normalize, Mode};

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "gofa_vietnam_real_road_names.csv".to_owned());
    let output_path = env::args()
        .nth(2)
        .unwrap_or_else(|| "road_name_normalization_stats.csv".to_owned());

    let mut reader = csv::Reader::from_path(input_path)?;
    let mut writer = csv::Writer::from_path(&output_path)?;
    writer.write_record([
        "input",
        "abbreviate_output",
        "remove_output",
        "abbreviate_changed",
        "remove_changed",
    ])?;

    let mut total = 0_u64;
    let mut abbreviate_changed = 0_u64;
    let mut remove_changed = 0_u64;

    for row in reader.records() {
        let row = row?;
        let input = row.get(0).ok_or("missing road_name value")?;
        let abbreviated = normalize(input, Mode::Abbreviate);
        let removed = normalize(input, Mode::Remove);
        let abbreviated_differs = abbreviated != input;
        let removed_differs = removed != input;

        total += 1;
        abbreviate_changed += u64::from(abbreviated_differs);
        remove_changed += u64::from(removed_differs);
        writer.write_record([
            input,
            &abbreviated,
            &removed,
            if abbreviated_differs { "true" } else { "false" },
            if removed_differs { "true" } else { "false" },
        ])?;
    }

    writer.flush()?;
    println!("output={output_path}");
    println!("total={total}");
    println!("abbreviate_changed={abbreviate_changed}");
    println!("remove_changed={remove_changed}");
    Ok(())
}
