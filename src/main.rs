use std::fs::File;
use std::io::{BufWriter, Write, Read};
use serde::{Deserialize, Serialize};
use chrono::{Utc, NaiveDate};

#[derive(Debug, Deserialize, Serialize)]
struct Empleado {
    nombre: String,
    fecha_ingreso: NaiveDate,
    dias_utilizados: u64,
}

fn read_employ(path: &str) -> Empleado {
    let mut file = File::open(path).expect(format!("No se pudo encontrar el archivo {path}.").as_str());
    let mut data = String::new();
    file.read_to_string(&mut data).expect(format!("No se pudo leer del archivo {path}").as_str());

    let employ: Empleado = serde_json::from_str(&data).expect(format!("{path} no tiene el formato esperado").as_str());

    return employ;
}

fn write_employ(path: &str, employ: &Empleado) {
    let file = File::create(path).expect(format!("No se pudo crear el archivo {path}.").as_str());
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, employ).expect(format!("Error al preparar el writer del archivo {path}").as_str());
    writer.flush().expect(format!("Error al escribir en el archivo {path}").as_str());
}

fn main() {
    let filename = "docs/data.json";
    let employ: Empleado = read_employ(filename);

    println!("{:?}", employ);

    let current_date = Utc::now().naive_utc().date();
    let diff = current_date.signed_duration_since(employ.fecha_ingreso);
    let days = diff.num_days();
    let vacation_days_total: f64 = days as f64*0.0411;
    let vacation_days_available: f64 = vacation_days_total - employ.dias_utilizados as f64;

    println!("Días de vacaciones disponibles a la fecha {}: {}", current_date, vacation_days_available);

    write_employ(filename, &employ)
}
