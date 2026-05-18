#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan rekap absensi
#[contracttype]
#[derive(Clone, Debug)]
pub struct AttendanceRecord {
    id: u64,
    nim: String,
    name: String,
}

// Storage key untuk data absensi (maksimal 9 karakter untuk symbol_short)
const ATTN_DATA: Symbol = symbol_short!("ATTN_DATA");

#[contract]
pub struct AttendanceContract;

#[contractimpl]
impl AttendanceContract {
    // Fungsi untuk melihat semua data absensi yang sudah masuk
    pub fn get_attendance(env: Env) -> Vec<AttendanceRecord> {
        // 1. ambil data absensi dari storage
        return env.storage().instance().get(&ATTN_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk melakukan absensi (mencatat kehadiran)
    pub fn record_attendance(env: Env, nim: String, name: String) -> String {
        // 1. ambil data absensi dari storage
        let mut records: Vec<AttendanceRecord> = env.storage().instance().get(&ATTN_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Buat object absensi baru
        let record = AttendanceRecord {
            id: env.prng().gen::<u64>(),
            nim: nim, // Menyimpan Nomor Induk Mahasiswa
            name: name, // Menyimpan Nama Mahasiswa
        };
        
        // 3. tambahkan data absensi baru ke daftar
        records.push_back(record);
        
        // 4. simpan daftar absensi yang sudah diperbarui ke storage
        env.storage().instance().set(&ATTN_DATA, &records);
        
        return String::from_str(&env, "Absensi berhasil dicatat di Blockchain!");
    }

    // Fungsi untuk membatalkan/menghapus data absensi berdasarkan id
    pub fn remove_attendance(env: Env, id: u64) -> String {
        // 1. ambil data absensi dari storage 
        let mut records: Vec<AttendanceRecord> = env.storage().instance().get(&ATTN_DATA).unwrap_or(Vec::new(&env));

        // 2. cari index absensi yang akan dihapus menggunakan perulangan
        for i in 0..records.len() {
            if records.get(i).unwrap().id == id {
                records.remove(i);

                env.storage().instance().set(&ATTN_DATA, &records);
                return String::from_str(&env, "Data absensi berhasil dihapus");
            }
        }

        return String::from_str(&env, "Data absensi tidak ditemukan")
    }
}

mod test;