#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U256}, 
    prelude::*, 
    msg,
    block,
    call::transfer_eth,
};
use alloc::{string::String, vec::Vec};

sol_storage! {
    #[entrypoint]
    pub struct MusicStreamingPlatform {
        // Owner del contrato
        address owner;
        
        // Precio por reproducción en wei (usando ETH)
        uint256 price_per_play;
        
        // Datos de artistas
        mapping(address => Artist) artists;
        mapping(address => bool) is_artist_registered;
        
        // Datos de canciones
        mapping(uint256 => Song) songs;
        uint256 next_song_id;
        
        // Balances de artistas (en wei)
        mapping(address => uint256) artist_balances;
        
        // Reproducciones por canción
        mapping(uint256 => uint256) song_plays;
        
        // Lista de canciones por artista (simplificada)
        mapping(address => uint256) artist_song_count;
        
        // Total de canciones
        uint256 total_songs;
    }
}

// Structs para datos
sol_storage! {
    pub struct Artist {
        address wallet;
        string name;
        string metadata_uri;
        uint256 total_plays;
        uint256 total_earnings;
        bool verified;
        uint256 songs_count;
    }
    
    pub struct Song {
        uint256 id;
        address artist;
        string title;
        string ipfs_hash;
        uint256 plays;
        bool active;
        uint256 created_at;
    }
}

#[public]
impl MusicStreamingPlatform {
    
    /// Inicializar la plataforma
    pub fn initialize(&mut self, price_per_play: U256) -> Result<(), Vec<u8>> {
        if !self.owner.get().is_zero() {
            return Err(b"Already initialized".to_vec());
        }
        
        self.owner.set(msg::sender());
        self.price_per_play.set(price_per_play);
        self.next_song_id.set(U256::from(1)); // Empezar desde 1
        self.total_songs.set(U256::from(0));
        
        Ok(())
    }

    /// Verificar si está inicializado
    pub fn is_initialized(&self) -> bool {
        !self.owner.get().is_zero()
    }

    /// Obtener configuración básica
    pub fn get_config(&self) -> (Address, U256, U256) {
        (self.owner.get(), self.price_per_play.get(), self.total_songs.get())
    }

    // =================== GESTIÓN DE ARTISTAS ===================
    
    /// Registrar artista
    pub fn register_artist(&mut self, name: String, metadata_uri: String) -> Result<(), Vec<u8>> {
        let artist_addr = msg::sender();
        
        if self.is_artist_registered.get(artist_addr) {
            return Err(b"Artist already registered".to_vec());
        }
        
        // Crear y configurar artista
        let mut artist_storage = self.artists.setter(artist_addr);
        artist_storage.wallet.set(artist_addr);
        artist_storage.name.set_str(&name);
        artist_storage.metadata_uri.set_str(&metadata_uri);
        artist_storage.verified.set(false);
        artist_storage.songs_count.set(U256::from(0));
        artist_storage.total_plays.set(U256::from(0));
        artist_storage.total_earnings.set(U256::from(0));
        
        self.is_artist_registered.setter(artist_addr).set(true);
        
        Ok(())
    }
    
    /// Obtener información de artista
    pub fn get_artist(&self, artist_addr: Address) -> Result<(String, String, U256, U256, bool), Vec<u8>> {
        if !self.is_artist_registered.get(artist_addr) {
            return Err(b"Artist not found".to_vec());
        }
        
        let artist = self.artists.get(artist_addr);
        Ok((
            artist.name.get_string(),
            artist.metadata_uri.get_string(),
            artist.total_plays.get(),
            artist.total_earnings.get(),
            artist.verified.get()
        ))
    }
    
    /// Verificar si una dirección es artista registrado
    pub fn is_artist(&self, artist_addr: Address) -> bool {
        self.is_artist_registered.get(artist_addr)
    }

    // =================== GESTIÓN DE CANCIONES ===================
    
    /// Subir canción (solo artistas registrados)
    pub fn upload_song(&mut self, title: String, ipfs_hash: String) -> Result<U256, Vec<u8>> {
        let artist_addr = msg::sender();
        
        if !self.is_artist_registered.get(artist_addr) {
            return Err(b"Artist not registered".to_vec());
        }
        
        let song_id = self.next_song_id.get();
        
        // Crear y configurar canción
        let mut song_storage = self.songs.setter(song_id);
        song_storage.id.set(song_id);
        song_storage.artist.set(artist_addr);
        song_storage.title.set_str(&title);
        song_storage.ipfs_hash.set_str(&ipfs_hash);
        song_storage.plays.set(U256::from(0));
        song_storage.active.set(true);
        song_storage.created_at.set(U256::from(block::timestamp()));
        
        // Actualizar contador de canciones del artista
        let mut artist_storage = self.artists.setter(artist_addr);
        let current_songs = artist_storage.songs_count.get();
        artist_storage.songs_count.set(current_songs + U256::from(1));
        
        // Incrementar contadores
        self.next_song_id.set(song_id + U256::from(1));
        let total = self.total_songs.get();
        self.total_songs.set(total + U256::from(1));
        
        // Incrementar contador por artista
        let artist_count = self.artist_song_count.get(artist_addr);
        self.artist_song_count.setter(artist_addr).set(artist_count + U256::from(1));
        
        Ok(song_id)
    }
    
    /// Obtener información de canción
    pub fn get_song(&self, song_id: U256) -> Result<(Address, String, String, U256, bool), Vec<u8>> {
        let song = self.songs.get(song_id);
        
        if song.artist.get().is_zero() {
            return Err(b"Song not found".to_vec());
        }
        
        Ok((
            song.artist.get(),
            song.title.get_string(),
            song.ipfs_hash.get_string(),
            song.plays.get(),
            song.active.get()
        ))
    }
    
    /// Obtener número de canciones de un artista
    pub fn get_artist_song_count(&self, artist_addr: Address) -> U256 {
        self.artist_song_count.get(artist_addr)
    }
    
    /// Obtener total de canciones
    pub fn get_total_songs(&self) -> U256 {
        self.total_songs.get()
    }

    /// Obtener canciones por rango (para paginación)
    pub fn get_songs_in_range(&self, start: U256, end: U256) -> Vec<U256> {
        let mut songs = Vec::new();
        let total = self.total_songs.get();
        let max_end = if end > total { total } else { end };
        
        let mut current = start;
        while current < max_end && songs.len() < 50 { // Limitar a 50 por llamada
            let song_id = current + U256::from(1); // Los IDs empiezan desde 1
            if !self.songs.get(song_id).artist.get().is_zero() {
                songs.push(song_id);
            }
            current += U256::from(1);
        }
        
        songs
    }

    // =================== REPRODUCCIÓN Y PAGOS ===================
    
    /// Reproducir canción con micropago en ETH
    #[payable]
    pub fn play_song(&mut self, song_id: U256) -> Result<(), Vec<u8>> {
        let _listener = msg::sender();
        let price = self.price_per_play.get();
        
        // Verificar pago
        if msg::value() < price {
            return Err(b"Insufficient payment".to_vec());
        }
        
        // Verificar que la canción existe y está activa
        let song = self.songs.get(song_id);
        if song.artist.get().is_zero() {
            return Err(b"Song not found".to_vec());
        }
        
        if !song.active.get() {
            return Err(b"Song is not active".to_vec());
        }
        
        let artist_addr = song.artist.get();
        
        // Actualizar contador de reproducciones de la canción
        let mut song_storage = self.songs.setter(song_id);
        let current_plays = song_storage.plays.get();
        song_storage.plays.set(current_plays + U256::from(1));
        
        // Actualizar estadísticas del artista
        let mut artist_storage = self.artists.setter(artist_addr);
        let artist_plays = artist_storage.total_plays.get();
        let artist_earnings = artist_storage.total_earnings.get();
        artist_storage.total_plays.set(artist_plays + U256::from(1));
        artist_storage.total_earnings.set(artist_earnings + price);
        
        // Agregar al balance del artista
        let current_balance = self.artist_balances.get(artist_addr);
        self.artist_balances.setter(artist_addr).set(current_balance + price);
        
        // Actualizar reproducciones globales
        let global_plays = self.song_plays.get(song_id);
        self.song_plays.setter(song_id).set(global_plays + U256::from(1));
        
        Ok(())
    }
    
    /// Obtener balance de artista
    pub fn get_artist_balance(&self, artist_addr: Address) -> U256 {
        self.artist_balances.get(artist_addr)
    }
    
    /// Obtener reproducciones de una canción
    pub fn get_song_plays(&self, song_id: U256) -> U256 {
        self.song_plays.get(song_id)
    }

    // =================== RETIRO DE FONDOS ===================
    
    /// Retirar fondos (solo el artista puede retirar sus fondos)
    pub fn withdraw(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        let artist_addr = msg::sender();
        let current_balance = self.artist_balances.get(artist_addr);
        
        if current_balance < amount {
            return Err(b"Insufficient balance".to_vec());
        }
        
        // Actualizar balance antes de transferir
        self.artist_balances.setter(artist_addr).set(current_balance - amount);
        
        // Transferir ETH al artista
        match transfer_eth(artist_addr, amount) {
            Ok(_) => Ok(()),
            Err(_) => {
                // Revertir balance si falla la transferencia
                self.artist_balances.setter(artist_addr).set(current_balance);
                Err(b"ETH transfer failed".to_vec())
            }
        }
    }

    // =================== FUNCIONES DE UTILIDAD ===================
    
    /// Solo owner: cambiar precio por reproducción
    pub fn set_price_per_play(&mut self, new_price: U256) -> Result<(), Vec<u8>> {
        if msg::sender() != self.owner.get() {
            return Err(b"Only owner".to_vec());
        }
        
        self.price_per_play.set(new_price);
        Ok(())
    }
    
    /// Solo owner: pausar/activar canción
    pub fn toggle_song_status(&mut self, song_id: U256) -> Result<(), Vec<u8>> {
        if msg::sender() != self.owner.get() {
            return Err(b"Only owner".to_vec());
        }
        
        let mut song_storage = self.songs.setter(song_id);
        if song_storage.artist.get().is_zero() {
            return Err(b"Song not found".to_vec());
        }
        
        let current_status = song_storage.active.get();
        song_storage.active.set(!current_status);
        
        Ok(())
    }

    /// Obtener información básica de múltiples canciones
    pub fn get_multiple_songs(&self, song_ids: Vec<U256>) -> Vec<(U256, Address, String, U256, bool)> {
        let mut results = Vec::new();
        
        for song_id in song_ids.iter().take(20) { // Limitar a 20 por gas
            let song = self.songs.get(*song_id);
            if !song.artist.get().is_zero() {
                results.push((
                    *song_id,
                    song.artist.get(),
                    song.title.get_string(),
                    song.plays.get(),
                    song.active.get()
                ));
            }
        }
        
        results
    }

    /// Helper: obtener precio por reproducción
    pub fn get_price_per_play(&self) -> U256 {
        self.price_per_play.get()
    }

    /// Helper: obtener owner
    pub fn get_owner(&self) -> Address {
        self.owner.get()
    }
}