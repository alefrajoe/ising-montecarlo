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

pub fn lattice_position(position: usize) -> [usize; DIMENSIONS] {
    let mut lattice_position = [0; DIMENSIONS];
    let mut position = position;
    
    for i in 0..DIMENSIONS {

        // Compute the lattice position in the dimension passed
        lattice_position[i] = position % LATTICE_SIZE;

        // Update the position
        position = position / LATTICE_SIZE;
    }
    lattice_position
}

pub fn chessboard(lattice_position: [usize; DIMENSIONS]) -> bool {
    lattice_position.iter().sum::<usize>() % 2 == 0
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

    #[test]
    fn test_lattice_position() {
        assert_eq!(lattice_position(12), [0, 3, 0]);
        assert_eq!(lattice_position(1), [1, 0, 0]);
        assert_eq!(lattice_position(32), [0, 0, 2]);
    }

    #[test]
    fn test_chessboard() {
        assert_eq!(chessboard([0, 0, 0]), true);
        assert_eq!(chessboard([0, 0, 1]), false);
        assert_eq!(chessboard([0, 1, 0]), false);
        assert_eq!(chessboard([0, 1, 1]), true);
    }
}