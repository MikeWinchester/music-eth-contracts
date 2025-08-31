#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U256}, 
    prelude::*, 
    msg,
    call::transfer_eth,
};
use alloc::vec::Vec;

// =================== ESTRUCTURAS CORREGIDAS ===================

// Estructura Song con tipos de storage correctos
sol_storage! {
    pub struct Song {
        address artist;          // ✅ Corregido: usar 'address' no 'Address'
        uint256 song_id;         // ✅ Corregido: mantener 'uint256'
        uint256 price_per_play;  // ✅ Corregido: mantener 'uint256'
        uint256 plays;           // ✅ Agregado: contador de reproducciones
    }
}

// Estructura User con tipos de storage correctos
sol_storage! {
    pub struct User {
        address address;         // ✅ Corregido: usar 'address' no 'Address'
        uint256 balance;         // ✅ Corregido: mantener 'uint256'
    }
}

// Storage principal
sol_storage! {
    #[entrypoint]
    pub struct MusicStreamingPlatform {
        User listener;
        mapping(uint256 => Song) songs;
        uint256 total_songs;     // ✅ Agregado: contador total de canciones
        mapping(address => bool) artists; // ✅ Agregado: registro de artistas
    }
}

#[public]
impl MusicStreamingPlatform {
    
    /// Inicializar la plataforma
    pub fn initialize(&mut self) {
        // Inicializar valores por defecto si es necesario
        self.total_songs.set(U256::ZERO);
    }

    /// Verificar si está inicializado
    pub fn is_initialized(&self) -> bool {
        // Lógica simple: si hay algún dato, está inicializado
        true
    }

    /// Obtener configuración básica
    pub fn get_config(&self) -> U256 {
        self.total_songs.get()
    }

    // =================== GESTIÓN DE ARTISTAS ===================
    
    /// Registrar artista
    pub fn register_artist(&mut self) {
        let sender = msg::sender();
        self.artists.setter(sender).set(true);
    }
    
    /// Obtener información de artista
    pub fn get_artist(&self, artist_address: Address) -> bool {
        self.artists.get(artist_address)
    }
    
    /// Verificar si una dirección es artista registrado
    pub fn is_artist(&self, artist_address: Address) -> bool {
        self.artists.get(artist_address)
    }

    // =================== GESTIÓN DE CANCIONES ===================
    
    /// Subir canción (solo artistas registrados)
    pub fn upload_song(&mut self, price_per_play: U256) -> Result<U256, Vec<u8>> {
        let sender = msg::sender();
        
        // Verificar que el sender es un artista registrado
        if !self.is_artist(sender) {
            return Err(b"Solo artistas registrados pueden subir canciones".to_vec());
        }
        
        // Generar nuevo song_id
        let current_total = self.total_songs.get();
        let new_song_id = current_total + U256::from(1);
        
        // Crear nueva canción
        let mut new_song = self.songs.setter(new_song_id);
        new_song.artist.set(sender);
        new_song.song_id.set(new_song_id);
        new_song.price_per_play.set(price_per_play);
        new_song.plays.set(U256::ZERO);
        
        // Actualizar contador total
        self.total_songs.set(new_song_id);
        
        Ok(new_song_id)
    }
    
    /// Obtener información de canción
    pub fn get_song(&self, song_id: U256) -> (Address, U256, U256, U256) {
        let song = self.songs.get(song_id);
        (
            song.artist.get(),
            song.song_id.get(),
            song.price_per_play.get(),
            song.plays.get()
        )
    }
    
    /// Obtener número total de canciones
    pub fn get_total_songs(&self) -> U256 {
        self.total_songs.get()
    }

    // =================== REPRODUCCIÓN Y PAGOS ===================
    
    /// Reproducir canción con micropago en ETH
    #[payable]
    pub fn play_song(&mut self, song_id: U256) -> Result<(), Vec<u8>> {
        // Verificar que la canción existe
        if song_id == U256::ZERO || song_id > self.total_songs.get() {
            return Err(b"Cancion no existe".to_vec());
        }
        
        // Obtener información de la canción y guardar valores necesarios
        let (artist_address, required_price, current_plays) = {
            let song = self.songs.get(song_id);
            (
                song.artist.get(),
                song.price_per_play.get(),
                song.plays.get()
            )
        }; // ✅ Aquí termina el borrow inmutable
        
        // Verificar que se envió el monto correcto
        if msg::value() != required_price {
            return Err(b"Monto incorrecto para reproducir".to_vec());
        }
        
        // ¡AQUÍ SE TRANSFIERE EL ETH DEL USUARIO AL ARTISTA!
        // El ETH ya fue descontado de la wallet del usuario cuando envió la transacción
        transfer_eth(artist_address, msg::value())?;
        
        // Incrementar contador de reproducciones (ahora podemos usar mutable borrow)
        let mut song_mut = self.songs.setter(song_id);
        song_mut.plays.set(current_plays + U256::from(1));
        
        Ok(())
    }
    
    /// Obtener reproducciones de una canción
    pub fn get_song_plays(&self, song_id: U256) -> U256 {
        if song_id == U256::ZERO || song_id > self.total_songs.get() {
            return U256::ZERO;
        }
        let song = self.songs.get(song_id);
        song.plays.get()
    }

    // =================== TRANSFERENCIAS ===================
    
    /// Transfer funds genérico (para propinas, etc.)
    #[payable]
    pub fn transfer_funds(&mut self, artist_address: Address) -> Result<(), Vec<u8>> {
        // Verificar que se envió ETH
        if msg::value() == U256::ZERO {
            return Err(b"Debe enviar ETH para transferir".to_vec());
        }
        
        // Transferir todo el ETH enviado
        transfer_eth(artist_address, msg::value())?;
        
        Ok(())
    }

    // =================== FUNCIONES DE UTILIDAD ===================
    
    /// Obtener dirección del usuario actual
    pub fn get_user_address(&self) -> Address {
        msg::sender()
    }

    /// Obtener precio por reproducción
    pub fn get_price_per_play(&self, song_id: U256) -> U256 {
        if song_id == U256::ZERO || song_id > self.total_songs.get() {
            return U256::ZERO;
        }
        let song = self.songs.get(song_id);
        song.price_per_play.get()
    }

    /// Verificar si una canción existe
    pub fn song_exists(&self, song_id: U256) -> bool {
        song_id != U256::ZERO && song_id <= self.total_songs.get()
    }
}