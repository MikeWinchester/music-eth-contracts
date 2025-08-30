// contracts/src/lib.rs - Versión actualizada para stylus-sdk 0.6.0
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U256}, 
    prelude::*, 
    msg,
    call::transfer_eth,
};

// Definir el almacenamiento usando la nueva sintaxis sol_storage!
sol_storage! {
    #[entrypoint]
    pub struct MusicStreamingPlatform {
        address owner;
        /// Balance de cada artista
        mapping(address => uint256) artist_balances;
        /// Número de reproducciones por canción
        mapping(uint256 => uint256) song_plays;
        /// Precio por reproducción en wei
        uint256 price_per_play;
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
        
        Ok(())
    }

    /// Verificar si está inicializado
    pub fn is_initialized(&self) -> bool {
        !self.owner.get().is_zero()
    }

    /// Obtener owner
    pub fn get_owner(&self) -> Address {
        self.owner.get()
    }

    /// Obtener precio por reproducción
    pub fn get_price_per_play(&self) -> U256 {
        self.price_per_play.get()
    }

    /// Reproducir canción y pagar al artista
    #[payable]
    pub fn play_song(&mut self, song_id: U256, artist: Address) -> Result<(), Vec<u8>> {
        let price = self.price_per_play.get();
        
        // Verificar que se envió el pago correcto
        if msg::value() < price {
            return Err(b"Insufficient payment".to_vec());
        }

        // Incrementar contador de reproducciones
        let current_plays = self.song_plays.get(song_id);
        self.song_plays.insert(song_id, current_plays + U256::from(1));

        // Agregar pago al balance del artista
        let current_balance = self.artist_balances.get(artist);
        self.artist_balances.insert(artist, current_balance + price);

        Ok(())
    }

    /// Obtener balance de un artista
    pub fn get_artist_balance(&self, artist: Address) -> U256 {
        self.artist_balances.get(artist)
    }

    /// Obtener reproducciones de una canción
    pub fn get_song_plays(&self, song_id: U256) -> U256 {
        self.song_plays.get(song_id)
    }

    /// Retirar fondos (solo el artista)
    pub fn withdraw(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        let artist = msg::sender();
        let balance = self.artist_balances.get(artist);

        if balance < amount {
            return Err(b"Insufficient balance".to_vec());
        }

        // Actualizar balance
        self.artist_balances.insert(artist, balance - amount);
        
        // Transferir fondos
        match transfer_eth(artist, amount) {
            Ok(_) => Ok(()),
            Err(_) => {
                // Revertir el cambio de balance si la transferencia falla
                self.artist_balances.insert(artist, balance);
                Err(b"Transfer failed".to_vec())
            }
        }
    }

    /// Cambiar precio por reproducción (solo owner)
    pub fn set_price_per_play(&mut self, new_price: U256) -> Result<(), Vec<u8>> {
        if msg::sender() != self.owner.get() {
            return Err(b"Only owner".to_vec());
        }
        
        self.price_per_play.set(new_price);
        Ok(())
    }
}