use std::collections::HashSet;

const EMPTY_CHAIN: u8 = 0xff;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Channels {
    pu1: SongChannel,
    pu2: SongChannel,
    wav: SongChannel,
    noi: SongChannel,
}

impl Channels {
    /// Returns an immutable version of all channels.
    pub fn get(&self) -> [&SongChannel; 4] {
        [
            self.get_pu1(),
            self.get_pu2(),
            self.get_wav(),
            self.get_noi(),
        ]
    }

    /// Immutably returns pulse channel 1.
    pub fn get_pu1(&self) -> &SongChannel {
        &self.pu1
    }

    /// Immutably returns pulse channel 2.
    pub fn get_pu2(&self) -> &SongChannel {
        &self.pu2
    }

    /// Immutably returns wav.
    pub fn get_wav(&self) -> &SongChannel {
        &self.wav
    }

    /// Immutably returns noi.
    pub fn get_noi(&self) -> &SongChannel {
        &self.noi
    }

    /// Returns a mutable version of a channel based on its index.
    pub fn get_mut(&mut self, index: usize) -> &mut SongChannel {
        match index {
            0 => &mut self.pu1,
            1 => &mut self.pu2,
            2 => &mut self.wav,
            3 => &mut self.noi,
            _ => panic!("Invalid SongChannel Id: Expected 0-3, got {}", index),
        }
    }
}

/// Stores information about each Channel column on the Song screen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SongChannel {
    /// The chains on the channel.
    chains: [u8; 256],
    /// A hashset indicating where the bookmarks are.
    /// Limited to 16 at a time.
    bookmarks: HashSet<u8>,
}

impl SongChannel {
    pub fn new() -> SongChannel {
        SongChannel {
            chains: [0xff; 256],
            bookmarks: HashSet::new(),
        }
    }

    /// Gets the value of the chain as a given point.
    pub fn get_chain(&self, index: u8) -> Option<u8> {
        let chain = self.chains[index as usize];
        if chain == EMPTY_CHAIN {
            None
        } else {
            Some(chain)
        }
    }

    /// Sets a chain at a given point.
    /// Returns the old value of the chain.
    pub fn set_chain(&mut self, index: u8, chain: u8) -> Option<u8> {
        assert!(
            chain <= 0x7f || chain == EMPTY_CHAIN,
            "Chain should have an index between 0x00 - 0x7f or a value of {:02x}: Got {:#02x}",
            EMPTY_CHAIN,
            chain
        );

        let old_value = self.get_chain(index);
        self.chains[index as usize] = chain;
        old_value
    }

    /// Clears the chain at the given point.
    /// Returns the old value of the chain.
    pub fn clear_chain(&mut self, index: u8) -> Option<u8> {
        self.set_chain(index, EMPTY_CHAIN)
    }

    /// Checks if an index is bookmarked.
    /// NOTE: The index 0xff can not be bookmarked.
    pub fn is_bookmarked(&self, index: u8) -> bool {
        self.bookmarks.contains(&index)
    }

    /// Sets an index to be bookmarked.
    /// Returns if it was previously bookmarked or not.
    pub fn set_bookmark(&mut self, index: u8) -> bool {
        !self.bookmarks.insert(index)
    }

    /// Clears a bookmark from an index.
    /// Returns if it was previously bookmarked or not.
    pub fn clear_bookmark(&mut self, index: u8) -> bool {
        self.bookmarks.remove(&index)
    }

    /// Toggles a bookmark at an index.
    /// Returns if it was previously bookmarked or not.
    pub fn toggle_bookmark(&mut self, index: u8) -> bool {
        // If it failed to insert because it already exists,
        // clear the bookmark and return false.
        // Else return true.
        if !self.set_bookmark(index) {
            self.clear_bookmark(index)
        } else {
            false
        }
    }
}

impl Default for SongChannel {
    fn default() -> Self {
        SongChannel::new()
    }
}
