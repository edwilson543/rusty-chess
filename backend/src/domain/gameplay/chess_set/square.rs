use core::array;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Rank {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
}

impl Rank {
    pub fn iter() -> array::IntoIter<Self, 8> {
        [
            Rank::ONE,
            Rank::TWO,
            Rank::THREE,
            Rank::FOUR,
            Rank::FIVE,
            Rank::SIX,
            Rank::SEVEN,
            Rank::EIGHT,
        ]
        .into_iter()
    }

    pub fn index(&self) -> i8 {
        match self {
            Rank::ONE => 1,
            Rank::TWO => 2,
            Rank::THREE => 3,
            Rank::FOUR => 4,
            Rank::FIVE => 5,
            Rank::SIX => 6,
            Rank::SEVEN => 7,
            Rank::EIGHT => 8,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn iter() -> array::IntoIter<Self, 8> {
        [
            File::A,
            File::B,
            File::C,
            File::D,
            File::E,
            File::F,
            File::G,
            File::H,
        ]
        .into_iter()
    }

    pub fn index(&self) -> i8 {
        match self {
            File::A => 1,
            File::B => 2,
            File::C => 3,
            File::D => 4,
            File::E => 5,
            File::F => 6,
            File::G => 7,
            File::H => 8,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Square {
    rank: Rank,
    file: File,
}

impl Square {
    pub fn new(rank: Rank, file: File) -> Self {
        Self {
            rank: rank,
            file: file,
        }
    }

    pub fn get_rank(&self) -> &Rank {
        &self.rank
    }

    pub fn get_file(&self) -> &File {
        &self.file
    }
}

// Trait implementations.

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.file, self.rank)
    }
}
