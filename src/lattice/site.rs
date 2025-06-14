use crate::field::ising::IsingField;
use crate::field::initialisation::Initialisation;
use crate::settings::DIMENSIONS;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Site {
    position: usize,
    field: RwLock<IsingField>,
    next: [Option<Arc<Site>>; DIMENSIONS],
    previous: [Option<Arc<Site>>; DIMENSIONS],
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
}

impl PartialEq for Site {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position &&
        *self.field.read().unwrap() == *other.field.read().unwrap() &&
        self.next == other.next &&
        self.previous == other.previous
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
}   