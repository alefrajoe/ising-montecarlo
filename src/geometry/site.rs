    use crate::field::ising::IsingField;
    use crate::field::initialisation::Initialisation;
    use crate::settings::{DIMENSIONS, Settings};
    use std::sync::{Arc, RwLock};
    use crate::field::schema::Field;
    use rand::Rng;
    use crate::geometry::utils::{lattice_position, chessboard};
    use uuid::Uuid;

    #[derive(Debug)]
    pub struct Site {
        pub id: Uuid,
        pub position: usize,
        pub field: IsingField,
        pub next: [Option<Arc<RwLock<Site>>>; DIMENSIONS],
        pub previous: [Option<Arc<RwLock<Site>>>; DIMENSIONS],
        pub lattice_position: [usize; DIMENSIONS],
        pub chessboard: bool,
    }


    impl Clone for Site {
        fn clone(&self) -> Self {
            Self {
                id: Uuid::new_v4(),
                position: self.position,
                field: self.field.clone(),
                next: self.next.clone(),
                previous: self.previous.clone(),
                lattice_position: self.lattice_position,
                chessboard: self.chessboard,
            }
        }
    }

    impl Site {
        pub fn new(position: usize, initialisation: Initialisation) -> Self {
            Self { 
                id: Uuid::new_v4(),
                position, 
                field: IsingField::new(initialisation), 
                next: [const { None }; DIMENSIONS], 
                previous: [const { None }; DIMENSIONS],
                lattice_position: lattice_position(position),
                chessboard: chessboard(lattice_position(position)),
            }
        }

        pub fn update_next(&mut self, dimension: usize, site: Option<Arc<RwLock<Site>>>) {
            self.next[dimension] = site;
        }

        pub fn update_previous(&mut self, dimension: usize, site: Option<Arc<RwLock<Site>>>) {
            self.previous[dimension] = site;
        }

        pub fn flip(&mut self) {
            self.field = match self.field {
                IsingField::Up => IsingField::Down,
                IsingField::Down => IsingField::Up,
            };
        }

        pub fn local_energy(&self) -> f64 {
            
            // Initialise the energy to zero
            let mut energy = 0.0;

            // Add the energy of the next site
            for next in self.next.iter().flatten() {
                let current_field = self.field;
                let next_field = next.read().unwrap().field;
                energy += current_field.interaction(&next_field);
            }

            // Add the energy of the previous site
            for previous in self.previous.iter().flatten() {
                let current_field = self.field;
                let previous_field = previous.read().unwrap().field;
                energy += current_field.interaction(&previous_field);
            }

            // Return the energy
            energy
        }

        pub fn montecarlo_single_site<R: Rng>(&mut self, settings: &Settings, rng: &mut R) -> bool {  

            // Compute the local energy BEFORE flipping (only read locks)
            let local_energy = self.local_energy();

            // Flip the site
            self.flip();

            // Compute the local energy after flipping
            let new_local_energy = self.local_energy();

            // Compute the energy ratio
            let energy_ratio = (- settings.beta * (new_local_energy - local_energy)).exp();

            // Compute the acceptance probability
            if energy_ratio > 1.0 {
                //println!("Accepting flip!");
                // Accept the flip
                true
            } else {

                // Sampling step
                let random_number = rng.random_range(0.0..=1.0);
                if random_number < energy_ratio {
                    //println!("Accepting flip!");
                    // Accept the flip
                    true
                } else {

                    // Flip the site back
                    self.flip();

                    // Reject the flip
                    false
                }
            }
        }
    }

    impl PartialEq for Site {
        fn eq(&self, other: &Self) -> bool {
            self.position == other.position &&
            self.field == other.field &&
            self.next.iter().flatten().map(|next| next.read().unwrap().id).collect::<Vec<Uuid>>() == other.next.iter().flatten().map(|next| next.read().unwrap().id).collect::<Vec<Uuid>>() &&
            self.previous.iter().flatten().map(|previous| previous.read().unwrap().id).collect::<Vec<Uuid>>() == other.previous.iter().flatten().map(|previous| previous.read().unwrap().id).collect::<Vec<Uuid>>()
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
        use crate::settings::{SettingsBuilder};
        use crate::geometry::lattice_geometry::boundary_conditions::BoundaryConditions;

        #[test]
        fn test_site_new() {
            let position = 0;
            let initialisation = Initialisation::Uniform;
            let site = Site::new(position, initialisation.clone());
            assert_eq!(site.position, position);
            assert_eq!(site.field, IsingField::new(initialisation));
        }

        #[test]
        fn test_site_flip() {
            let position = 0;
            let initialisation = Initialisation::Uniform;
            let mut site = Site::new(position, initialisation.clone());
            site.flip();
            assert_eq!(site.field, IsingField::Down);
        }

        #[test]
        fn test_site_local_energy() {
            let position = 0;
            let initialisation = Initialisation::Uniform;
            let site = Site::new(position, initialisation.clone());
            assert_eq!(site.local_energy(), 0.0);
        }

        #[test]
        fn test_site_montecarlo_single_site() {
            let settings = SettingsBuilder { beta: 1.0, boundary_conditions: BoundaryConditions::Periodic, site_initialisation: Initialisation::Uniform }.build();
            let mut site = Site::new(0, Initialisation::Uniform);
            let mut rng = rand::rng();
            site.montecarlo_single_site(&settings, &mut rng);
        }

    }   