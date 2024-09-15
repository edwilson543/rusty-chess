use core::array;
use std::cmp;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Square {
    rank: Rank,
    file: File,
}

// Implementations.

impl Rank {
    pub fn iter() -> array::IntoIter<Self, 8> {
        [
            Rank::One,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
        ]
        .into_iter()
    }

    pub fn index(&self) -> i8 {
        match self {
            Rank::One => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
        }
    }

    pub fn from_index(index: i8) -> Self {
        match index {
            1 => Rank::One,
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            _ => panic!("'{}' is not a valid rank!", index),
        }
    }
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

    pub fn from_index(index: i8) -> Self {
        match index {
            1 => File::A,
            2 => File::B,
            3 => File::C,
            4 => File::D,
            5 => File::E,
            6 => File::F,
            7 => File::G,
            8 => File::H,
            _ => panic!("'{}' is not a valid file!", index),
        }
    }

    pub fn from_letter(letter: char) -> Self {
        match letter {
            'A' => File::A,
            'B' => File::B,
            'C' => File::C,
            'D' => File::D,
            'E' => File::E,
            'F' => File::F,
            'G' => File::G,
            'H' => File::H,
            _ => panic!("'{}' is not a valid file!", letter),
        }
    }
}

impl Square {
    pub fn new(rank: Rank, file: File) -> Self {
        Self {
            rank: rank,
            file: file,
        }
    }

    pub fn from_indexes(rank_index: i8, file_index: i8) -> Self {
        Self {
            rank: Rank::from_index(rank_index),
            file: File::from_index(file_index),
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

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.index().cmp(&other.index())
    }
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.index().cmp(&other.index())
    }
}

impl Ord for Square {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.get_rank().cmp(&other.get_rank()) {
            cmp::Ordering::Equal => self.get_file().cmp(&other.get_file()),
            result => result,
        }
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.index().partial_cmp(&other.index())
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.index().partial_cmp(&other.index())
    }
}

impl PartialOrd for Square {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self.get_rank().partial_cmp(&other.get_rank()) {
            Some(cmp::Ordering::Equal) => self.get_file().partial_cmp(&other.get_file()),
            result => result,
        }
    }
}
