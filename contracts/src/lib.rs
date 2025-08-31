#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U256}, 
    prelude::*, 
    msg,
    block,
    call::transfer_eth,
};


pub struct Song {
    Address artist;
    uint256 song_id;
    uint256 price_per_play;
    uint256 plays;
}

pub struct User {
    Address address;
    uint256 balance;
}

sol_storage! {
    #[entrypoint]
    pub struct MusicStreamingPlatform {
        /// Balance de cada artista
        // mapping(address => uint256) artist_balances;
        /// Número de reproducciones por canción
        // mapping(uint256 => uint256) song_plays;
        /// Precio por reproducción en wei
        // uint256 price_per_play;
        User listener;
        mapping(uint256 => Song) songs;
    }
}


#[public]
impl MusicStreamingPlatform {
    /// Obtener user
    pub fn get_user_address(&self) -> Address {
        self.listener.address.get()
    }

    /// encontrar cancion
    fn find_song(&self, song_id: U256) -> (bool, Song) {
        let current_song = self.songs.get(song_id);
        let exists = if current_song.song_id != U256::ZERO { true } else { false };
        return (exists, current_song);
    }

    /// Obtener precio por reproducción
    pub fn get_price_per_play(&self, song_id: U256) -> U256 {
        let (exists , current_song): (bool,Song) = self.find_song(song_id);
        if exists == true {
            return current_song.price_per_play.get();
        }else {
            return U256::from(0);
        }
    }

    /// Reproducir canción y pagar al artista
    #[payable]
    pub fn play_song(&mut self, song_id: U256, artist: Address) -> Result<(), Vec<u8>> {
        let price = self.get_price_per_play(song_id);
        self.withdraw(self.listener,price);
        Ok(())
    }

    /// Obtener balance de un artista
    pub fn get_artist_balance(&self, artist: User) -> U256 {
        artist.address.get()
    }
    
    /// Obtener reproducciones de una canción
    pub fn get_song_plays(&self, song_id: U256) -> U256 {
        let (exists , current_song): (bool,Song) = self.find_song(song_id);
        if exists == true {
            return current_song.plays.get();
        }else {
            return U256::from(0);
        }
    }

    
    /// Retirar fondos (solo el artista puede retirar sus fondos)
    pub fn withdraw(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        let artist_addr = msg::sender();
        let current_balance = self.artist_balances.get(artist_addr);
        
        if current_balance < amount {
            return Err(b"Insufficient balance".to_vec());
        }
        
        // Actualizar balance antes de transferir
        self.artist_balances.setter(artist_addr).set(current_balance - amount);
        
        // Transferir fondos
        // match transfer_eth(artist, amount) {
        //     Ok(_) => Ok(()),
        //     Err(_) => {
        //         // Revertir el cambio de balance si la transferencia falla
        //         self.artist_balances.insert(artist, balance);
        //         Err(b"Transfer failed".to_vec())
        //     }
        // }
    }


}