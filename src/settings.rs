#[cfg(not(test))]
pub const DIMENSIONS: usize = 2;
#[cfg(test)]
pub const DIMENSIONS: usize = 3;  //---TESTS---

#[cfg(not(test))]
pub const LATTICE_SIZE: usize = 32;
#[cfg(test)]
pub const LATTICE_SIZE: usize = 4;  //---TESTS---