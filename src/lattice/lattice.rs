use std::sync::Arc;
use crate::lattice::site::Site;
use crate::field::initialisation::Initialisation;
use crate::settings::{DIMENSIONS, LATTICE_SIZE};
use crate::lattice::utils::{next_position, previous_position};

pub struct Lattice {
    sites: [Site; usize::pow(LATTICE_SIZE, DIMENSIONS as u32)],
}

impl Lattice {
    pub fn new(initialisation: Initialisation) -> Self {

        // Create all sites
        let mut sites = std::array::from_fn(|i| Site::new(i, initialisation.clone()));
        
        // First create all Arc references
        let site_refs: Vec<Arc<Site>> = sites.iter()
            .map(|site| Arc::new(site.clone()))
            .collect();

        // Then update the links
        for (i, site) in sites.iter_mut().enumerate() {
            for d in 0..DIMENSIONS {
                let next_pos = next_position(i, d);
                let prev_pos = previous_position(i, d);
                site.update_next(d, site_refs[next_pos].clone());
                site.update_previous(d, site_refs[prev_pos].clone());
            }
        }

        Self { sites: sites }
    }

    pub fn get(&self, position: usize) -> &Site {
        &self.sites[position]
    }

    pub fn get_mut(&mut self, position: usize) -> &mut Site {
        &mut self.sites[position]
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

    #[test]
    fn test_lattice_get() {
        let lattice = Lattice::new(Initialisation::Uniform);
        for i in 0..lattice.sites.len() {
            let site = lattice.get(i);
            assert_eq!(site.position, i);
            assert_eq!(*site.field.read().unwrap(), IsingField::Up);
        }
    }

    #[test]
    fn test_lattice_get_mut() {
        let mut lattice = Lattice::new(Initialisation::Uniform);
        for i in 0..lattice.sites.len() {
            let site = lattice.get_mut(i);
            site.flip();
        }
    }

    #[test]
    fn test_lattice_geometry() {
        let lattice = Lattice::new(Initialisation::Uniform);
        assert_eq!(lattice.get(0).next[0].as_ref().unwrap().position, 1);
        assert_eq!(lattice.get(63).next[0].as_ref().unwrap().position, 0);
        assert_eq!(lattice.get(63).next[0].as_ref().unwrap(), lattice.get(0));
    }
}