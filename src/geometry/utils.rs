use crate::settings::{DIMENSIONS, LATTICE_SIZE};

pub fn next_position(position: usize, dimension: usize) -> usize {
    // Convert to lattice position
    let mut lattice_position = position_to_lattice(position);

    // Add 1 to the lattice position in the dimension passed
    lattice_position[dimension] = (lattice_position[dimension] + 1) % LATTICE_SIZE;

    // Convert back to position
    lattice_to_position(lattice_position)
}

pub fn previous_position(position: usize, dimension: usize) -> usize {
    // Convert to lattice position
    let mut lattice_position = position_to_lattice(position);

    // Subtract 1 from the lattice position in the dimension passed
    lattice_position[dimension] = (lattice_position[dimension] + LATTICE_SIZE - 1) % LATTICE_SIZE;

    // Convert back to position
    lattice_to_position(lattice_position)
}

pub fn position_to_lattice(position: usize) -> [usize; DIMENSIONS] {
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

pub fn lattice_to_position(lattice_position: [usize; DIMENSIONS]) -> usize {
    // Initialise the position to 0
    let mut position = 0;

    // Iterate over the dimensions
    for i in 0..DIMENSIONS {
        // Compute the lattice position in the dimension passed
        position += lattice_position[i] * usize::pow(LATTICE_SIZE, i as u32);
    }
    position
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
        assert_eq!(next_position(31, 1), 19);
    }

    #[test]
    fn test_previous_position() {
        assert_eq!(previous_position(0, 0), 3);
        assert_eq!(previous_position(16, 2), 0);
        assert_eq!(previous_position(31, 1), 27);
    }

    #[test]
    fn test_position_to_lattice() {
        assert_eq!(position_to_lattice(12), [0, 3, 0]);
        assert_eq!(position_to_lattice(1), [1, 0, 0]);
        assert_eq!(position_to_lattice(32), [0, 0, 2]);
    }

    #[test]
    fn test_chessboard() {
        assert_eq!(chessboard([0, 0, 0]), true);
        assert_eq!(chessboard([0, 0, 1]), false);
        assert_eq!(chessboard([0, 1, 0]), false);
        assert_eq!(chessboard([0, 1, 1]), true);
    }
}
