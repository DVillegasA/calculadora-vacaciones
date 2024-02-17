use std::fs::File;
use std::io;
use std::io::{BufWriter, Write, Read};
use serde::{Deserialize, Serialize};
use chrono::{Utc, NaiveDate};

#[derive(Deserialize, Serialize)]
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

fn calculate_extra_days(current_days: f64, filename: &str, employ: &mut Empleado) {
    println!("Favor ingrese la cantidad de días de vacaciones que se va a tomar");
    let mut vacation_days = String::new();
    io::stdin().read_line(&mut vacation_days).expect("Failed to read line");

    let vacation_days: u64 = match vacation_days.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Valor entregado es invalido, favor ingresar un número entero.");
            return;
        },
    };

    let vacation_days_available: f64 = current_days - vacation_days as f64;
    println!("Nuevo total de días de vacaciones disponibles {:.1}", vacation_days_available);
    println!("¿Desea modificar el archivo {filename} con la cantidad de días de vacaciones a tomar? [S/N]");
    
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read line");

    let option: char = match option.trim().parse() {
        Ok(ch) => ch,
        Err(_) => {
            println!("Valor entregado es invalido, favor ingresar S para si o N para no.");
            return;
        },
    };

    match option {
        's' => {
            employ.dias_utilizados += vacation_days;
            write_employ(filename, employ);
            println!("Archivo {filename} modificado correctamente!");
            return;
        },
        'n' => return,
        _ => println!("Valor entregado es invalido, favor ingresar S para si o N para no."),
    }
}

fn main() {
    let filename = "docs/data.json";
    let mut employ: Empleado = read_employ(filename);

    let current_date = Utc::now().naive_utc().date();
    let diff = current_date.signed_duration_since(employ.fecha_ingreso);
    let days = diff.num_days();
    let vacation_days_total: f64 = days as f64*0.0411;
    let vacation_days_available: f64 = vacation_days_total - employ.dias_utilizados as f64;

    println!("Días de vacaciones de {} disponibles a la fecha {}: {:.1}", employ.nombre, current_date, vacation_days_available);
    println!("¿Desea calcular cuantos días le van a quedar después de tomarse vacaciones? [S/N]");
    
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read line");

    let option: char = match option.trim().parse() {
        Ok(ch) => ch,
        Err(_) => {
            println!("Valor entregado es invalido, favor ingresar S para si o N para no.");
            return;
        },
    };

    match option {
        's' => calculate_extra_days(vacation_days_available, filename, &mut employ),
        'n' => return,
        _ => println!("Valor entregado es invalido, favor ingresar S para si o N para no."),
    }

    write_employ(filename, &employ)
}
