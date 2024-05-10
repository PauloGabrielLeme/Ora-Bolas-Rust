use std::fs::File;
use std::io::{BufRead, BufReader};
use plotters::*;

fn ler_arquivo(lista_t: &mut Vec<f64>,lista_x: &mut Vec<f64>,lista_y: &mut Vec<f64>,) -> Result<(), std::io::Error> {
    let file = File::open("dados.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines().skip(1) {
        let line = line?;
        let columns: Vec<_> = line.trim().split_whitespace().collect();

        let t_value = columns[0].replace(',', ".").parse::<f64>().unwrap_or(0.0);
        let x_value = columns[1].replace(',', ".").parse::<f64>().unwrap_or(0.0);
        let y_value = columns[2].replace(',', ".").parse::<f64>().unwrap_or(0.0);

        lista_t.push(t_value);
        lista_x.push(x_value);
        lista_y.push(y_value);
    }

    Ok(())
}
