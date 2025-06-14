use crate::field::ising::IsingField;
use crate::field::initialisation::Initialisation;
use crate::settings::DIMENSIONS;
use std::sync::{Arc, RwLock};
use crate::field::schema::Field;

#[derive(Debug)]
pub struct Site {
    pub position: usize,
    pub field: RwLock<IsingField>,
    pub next: [Option<Arc<Site>>; DIMENSIONS],
    pub previous: [Option<Arc<Site>>; DIMENSIONS],
}


impl Clone for Site {
    fn clone(&self) -> Self {
        Self {
            position: self.position,
            field: RwLock::new(*self.field.read().unwrap()),
            next: self.next.clone(),
            previous: self.previous.clone(),
        }
    }
}

impl Site {
    pub fn new(position: usize, initialisation: Initialisation) -> Self {
        Self { 
            position, 
            field: RwLock::new(IsingField::new(initialisation)), 
            next: [const { None }; DIMENSIONS], 
            previous: [const { None }; DIMENSIONS] 
        }
    }

    pub fn update_next(&mut self, dimension: usize, site: Option<Arc<Site>>) {
        self.next[dimension] = site;
    }

    pub fn update_previous(&mut self, dimension: usize, site: Option<Arc<Site>>) {
        self.previous[dimension] = site;
    }

    pub fn flip(&self) {
        let flipped = match *self.field.read().unwrap() {
            IsingField::Up => IsingField::Down,
            IsingField::Down => IsingField::Up,
        };
        *self.field.write().unwrap() = flipped;
    }

    pub fn local_energy(&self) -> f64 {
        
        // Initialise the energy to zero
        let mut energy = 0.0;

        // Add the energy of the next site
        for next in self.next.iter() {
            if let Some(next_site) = next {
                let current_field = *self.field.read().unwrap();
                let next_field = *next_site.field.read().unwrap();
                energy += current_field.interaction(&next_field);
            }
        }

        // Add the energy of the previous site
        for previous in self.previous.iter() {
            if let Some(previous_site) = previous {
                let current_field = *self.field.read().unwrap();
                let previous_field = *previous_site.field.read().unwrap();
                energy += current_field.interaction(&previous_field);
            }
        }

        // Return the energy
        energy
    }
}

impl PartialEq for Site {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position &&
        *self.field.read().unwrap() == *other.field.read().unwrap() &&
        self.next == other.next &&
        self.previous == other.previous
    }
}

impl PartialEq<Site> for Arc<Site> {
    fn eq(&self, other: &Site) -> bool {
        self.position == other.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_new() {
        let position = 0;
        let initialisation = Initialisation::Uniform;
        let site = Site::new(position, initialisation.clone());
        assert_eq!(site.position, position);
        assert_eq!(*site.field.read().unwrap(), IsingField::new(initialisation));
        assert_eq!(site.next, [const { None }; DIMENSIONS]);
        assert_eq!(site.previous, [const { None }; DIMENSIONS]);
    }

    #[test]
    fn test_site_flip() {
        let position = 0;
        let initialisation = Initialisation::Uniform;
        let site = Site::new(position, initialisation.clone());
        site.flip();
        assert_eq!(*site.field.read().unwrap(), IsingField::Down);
    }

    #[test]
    fn test_site_local_energy() {
        let position = 0;
        let initialisation = Initialisation::Uniform;
        let site = Site::new(position, initialisation.clone());
        assert_eq!(site.local_energy(), 0.0);
    }
}   