// Import necessary modules
use chrono::{NaiveDateTime, NaiveDate};
use std::collections::HashMap;

// Define the Pegawai struct
struct Pegawai {
    id_pegawai: i32,
    nama: String,
}

// Define the Parkir struct
struct Parkir {
    id_parkir: i32,
    masuk: NaiveDateTime,
    keluar: NaiveDateTime,
    harga_perjam: f64,
    total_harga: f64,
    nopol: String,
    type_parkir: String,
}

// Define the Tiket struct that holds collections of Pegawai and Parkir
struct Tiket {
    pegawai: Vec<Pegawai>,
    parkir: Vec<Parkir>,
}

impl Tiket {
    // Implement methods for Tiket struct
    fn new() -> Self {
        Self {
            pegawai: Vec::new(),
            parkir: Vec::new(),
        }
    }

    fn add_pegawai(&mut self, id_pegawai: i32, nama: String) {
        self.pegawai.push(Pegawai { id_pegawai, nama });
    }

    fn add_parkir(
        &mut self,
        id_parkir: i32,
        masuk: NaiveDateTime,
        keluar: NaiveDateTime,
        harga_perjam: f64,
        total_harga: f64,
        nopol: String,
        type_parkir: String,
    ) {
        self.parkir.push(Parkir {
            id_parkir,
            masuk,
            keluar,
            harga_perjam,
            total_harga,
            nopol,
            type_parkir,
        });
    }

    fn get_pegawai(&self) -> &Vec<Pegawai> {
        &self.pegawai
    }

    fn get_parkir(&self) -> &Vec<Parkir> {
        &self.parkir
    }
}

fn main() {
    // Create a new Tiket instance
    let mut tiket = Tiket::new();

    // Add some Pegawai and Parkir data
    tiket.add_pegawai(1, "John Doe".to_string());
    tiket.add_pegawai(2, "Jane Smith".to_string());

    let masuk1 = NaiveDate::from_ymd(2023, 7, 23).and_hms(10, 0, 0);
    let keluar1 = NaiveDate::from_ymd(2023, 7, 23).and_hms(14, 30, 0);
    tiket.add_parkir(1, masuk1, keluar1, 10.0, 40.0, "ABC123".to_string(), "Car".to_string());

    let masuk2 = NaiveDate::from_ymd(2023, 7, 23).and_hms(12, 0, 0);
    let keluar2 = NaiveDate::from_ymd(2023, 7, 23).and_hms(16, 45, 0);
    tiket.add_parkir(2, masuk2, keluar2, 8.0, 34.0, "XYZ789".to_string(), "Motorcycle".to_string());

    // Retrieve Pegawai and Parkir data
    let pegawai_data = tiket.get_pegawai();
    let parkir_data = tiket.get_parkir();

    // Display the data (just for demonstration purposes)
    for pegawai in pegawai_data {
        println!("ID Pegawai: {}, Nama: {}", pegawai.id_pegawai, pegawai.nama);
    }

    for parkir in parkir_data {
        println!(
            "ID Parkir: {}, Masuk: {}, Keluar: {}, Harga Perjam: {}, Total Harga: {}, NoPol: {}, Type Parkir: {}",
            parkir.id_parkir,
            parkir.masuk,
            parkir.keluar,
            parkir.harga_perjam,
            parkir.total_harga,
            parkir.nopol,
            parkir.type_parkir
        );
    }
}
