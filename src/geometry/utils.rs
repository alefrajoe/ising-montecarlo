use crate::settings::{DIMENSIONS, LATTICE_SIZE};

pub fn next_position(position: usize, dimension: usize) -> usize {

    // Take the position passed
    let mut position = position;

    // Add the lattice size to the position in the dimension passed
    position += usize::pow(LATTICE_SIZE, dimension as u32);
    
    // Return the position modulo the lattice size
    position % usize::pow(LATTICE_SIZE, DIMENSIONS as u32)
}

pub fn previous_position(position: usize, dimension: usize) -> usize {

    // Take the position passed
    let mut position = position;

    // Subtract the lattice size from the position in the dimension passed
    position += usize::pow(LATTICE_SIZE, DIMENSIONS as u32);

    // Subtract the lattice size from the position in the dimension passed
    position -= usize::pow(LATTICE_SIZE, dimension as u32);

    // Return the position modulo the lattice size
    position % usize::pow(LATTICE_SIZE, DIMENSIONS as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_position() {
        assert_eq!(next_position(0, 0), 1);
        assert_eq!(next_position(16, 2), 32);
        assert_eq!(next_position(31, 1), 35);
    }

    #[test]
    fn test_previous_position() {
        assert_eq!(previous_position(0, 0), 63);
        assert_eq!(previous_position(16, 2), 0);
        assert_eq!(previous_position(31, 1), 27);
    }
}