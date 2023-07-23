use chrono::{Utc, NaiveDateTime, Duration};
use std::collections::HashMap;

// Rest of the code remains the same

impl Tiket {
    // Rest of the implementation remains the same
    
    fn add_parkir_auto(
        &mut self,
        id_parkir: i32,
        harga_perjam: f64,
        nopol: String,
        type_parkir: String,
    ) {
        let masuk = Utc::now().naive_utc(); // Get the current date and time
        let keluar = masuk + Duration::hours(2); // Example: Set the "keluar" date 2 hours after "masuk"

        // You can adjust the "keluar" date based on your requirements.
        
        // Calculate the total_harga (example: assuming 1 hour minimum charge)
        let total_harga = harga_perjam * 1.0;

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
}

fn main() {
    // Create a new Tiket instance
    let mut tiket = Tiket::new();

    // Add some Pegawai and Parkir data
    tiket.add_pegawai(1, "John Doe".to_string());
    tiket.add_pegawai(2, "Jane Smith".to_string());

    tiket.add_parkir_auto(1, 10.0, "ABC123".to_string(), "Car".to_string());
    tiket.add_parkir_auto(2, 8.0, "XYZ789".to_string(), "Motorcycle".to_string());

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
