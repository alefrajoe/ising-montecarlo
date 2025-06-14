use crate::lattice::site::Site;
use crate::field::initialisation::Initialisation;
use crate::settings::{DIMENSIONS, LATTICE_SIZE};

pub struct Lattice {
    sites: [Site; usize::pow(LATTICE_SIZE, DIMENSIONS as u32)],
}

impl Lattice {
    pub fn new(initialisation: Initialisation) -> Self {
        let sites = std::array::from_fn(|i| Site::new(i, initialisation.clone()));
        Self { sites }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::ising::IsingField;
    
    #[test]
    fn test_lattice_new() {
        let lattice = Lattice::new(Initialisation::Uniform);
        assert_eq!(lattice.sites.len(), usize::pow(LATTICE_SIZE, DIMENSIONS as u32));

        for (i, site) in lattice.sites.iter().enumerate() {
            assert_eq!(site.position, i);
            assert_eq!(*site.field.read().unwrap(), IsingField::Up);
        }
    }
}