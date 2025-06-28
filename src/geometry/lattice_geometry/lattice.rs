use std::sync::Arc;
use crate::geometry::site::Site;
use crate::settings::{Settings, DIMENSIONS, LATTICE_SIZE};
use crate::geometry::utils::{next_position, previous_position};
use crate::geometry::lattice_geometry::boundary_conditions::BoundaryConditions;
use rayon::prelude::*;
use std::sync::RwLock;

pub struct Lattice {
    sites: Vec<Arc<RwLock<Site>>>,
    pub settings: Arc<Settings>,
}

impl Lattice {
    pub fn new(settings: Settings) -> Self {
        // Initialise the lattice sites using Vec instead of array
        let sites: Vec<Site> = (0..usize::pow(LATTICE_SIZE, DIMENSIONS as u32))
            .map(|i| Site::new(i, settings.site_initialisation))
            .collect();

        // Create the Arc references to the sites
        let site_refs: Vec<Arc<RwLock<Site>>> = sites.iter()
            .map(|site| Arc::new(RwLock::new(site.clone())))
            .collect();

        // Create the lattice with the original sites
        let mut lattice = Self { 
            sites: site_refs.clone(),
            settings: Arc::new(settings.clone()) 
        };
        
        // Create the lattice according to the boundary conditions
        match settings.boundary_conditions {
            BoundaryConditions::Periodic => initalise_periodic_boundary_conditions(&mut lattice, &site_refs),
            BoundaryConditions::Open => initalise_open_boundary_conditions(&mut lattice, &site_refs),
        }

        lattice
    }

    pub fn get(&self, position: usize) -> Arc<RwLock<Site>> {
        self.sites[position].clone()
    }

    pub fn get_mut(&mut self, position: usize) -> Arc<RwLock<Site>> {
        self.sites[position].clone()
    }

    pub fn get_site_clone(&self, position: usize) -> Site {
        self.sites[position].read().unwrap().clone()
    }

    pub fn get_energy(&self) -> f64 {
        self.sites.par_iter().map(|site| site.read().unwrap().local_energy()).sum::<f64>() / 2.0 // Divide by 2 because each interaction is counted twice
    }

    pub fn montecarlo_sweep(&mut self) {

        // Monte Carlo sweep for the chessboard sites
        self.sites.iter_mut().filter(|site| site.read().unwrap().chessboard).for_each(|site| {
            let mut rng = rand::rng();
            site.write().unwrap().montecarlo_single_site(&self.settings, &mut rng);
        });

        // Monte Carlo sweep for the non-chessboard sites
        self.sites.iter_mut().filter(|site| !site.read().unwrap().chessboard).for_each(|site| {
            let mut rng = rand::rng();
            site.write().unwrap().montecarlo_single_site(&self.settings, &mut rng);
        });
    }
}

fn initalise_periodic_boundary_conditions(lattice: &mut Lattice, site_refs: &[Arc<RwLock<Site>>]) {
    for i in 0..lattice.sites.len() {
        for d in 0..DIMENSIONS {

            // Next site
            let next_pos = next_position(i, d);
            lattice.get_mut(i).write().unwrap().update_next(d, Some(site_refs[next_pos].clone()));
            
            // Previous site
            let prev_pos = previous_position(i, d);
            lattice.get_mut(i).write().unwrap().update_previous(d, Some(site_refs[prev_pos].clone()));
        }
    }
}

fn initalise_open_boundary_conditions(lattice: &mut Lattice, site_refs: &[Arc<RwLock<Site>>]) {
    for i in 0..lattice.sites.len() {
        for d in 0..DIMENSIONS {

            // Next site
            let next_pos = next_position(i, d);
            
            // If position at the boundary of the lattice, set the next site to None
            if next_pos < i {
                lattice.get_mut(i).write().unwrap().update_next(d, None);
            } else {
                lattice.get_mut(i).write().unwrap().update_next(d, Some(site_refs[next_pos].clone()));
            }
            
            // Previous site
            let prev_pos = previous_position(i, d);
            
            // If position at the boundary of the lattice, set the previous site to None
            if prev_pos > i {
                lattice.get_mut(i).write().unwrap().update_previous(d, None);
            } else {
                lattice.get_mut(i).write().unwrap().update_previous(d, Some(site_refs[prev_pos].clone()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::ising::IsingField;
    use crate::settings::SettingsBuilder;
    use crate::field::initialisation::Initialisation;
    use crate::geometry::lattice_geometry::boundary_conditions::BoundaryConditions;

    #[test]
    fn test_lattice_new() {
        let settings = SettingsBuilder::new().add_beta(1.0).add_boundary_conditions(BoundaryConditions::Periodic).add_site_initialisation(Initialisation::Uniform).build();
        let lattice = Lattice::new(settings);
        assert_eq!(lattice.sites.len(), usize::pow(LATTICE_SIZE, DIMENSIONS as u32));

        for (i, site) in lattice.sites.iter().enumerate() {
            assert_eq!(site.read().unwrap().position, i);
            assert_eq!(site.read().unwrap().field, IsingField::Up);
        }
    }

    #[test]
    fn test_lattice_get() {
        let settings = SettingsBuilder::new().add_beta(1.0).add_boundary_conditions(BoundaryConditions::Periodic).add_site_initialisation(Initialisation::Uniform).build();
        let lattice = Lattice::new(settings);
        for i in 0..lattice.sites.len() {
            let site = lattice.get(i);
            assert_eq!(site.read().unwrap().position, i);
            assert_eq!(site.read().unwrap().field, IsingField::Up);
        }
    }

    #[test]
    fn test_lattice_get_mut() {
        let settings = SettingsBuilder { beta: 1.0, boundary_conditions: BoundaryConditions::Periodic, site_initialisation: Initialisation::Uniform }.build();
        let mut lattice = Lattice::new(settings);
        for i in 0..lattice.sites.len() {
            let site = lattice.get_mut(i);
            site.write().unwrap().flip();
        }
    }

    #[test]
    fn test_lattice_geometry() {
        let settings = SettingsBuilder { beta: 1.0, boundary_conditions: BoundaryConditions::Periodic, site_initialisation: Initialisation::Uniform }.build();
        let lattice = Lattice::new(settings);
        assert_eq!(lattice.get(0).read().unwrap().next[0].as_ref().unwrap().read().unwrap().position, 1);
        assert_eq!(lattice.get(63).read().unwrap().next[0].as_ref().unwrap().read().unwrap().position, 0);
        assert_eq!(lattice.get(63).read().unwrap().next[0].as_ref().unwrap().read().unwrap().id, lattice.get(0).read().unwrap().id);

        let settings = SettingsBuilder { beta: 1.0, boundary_conditions: BoundaryConditions::Open, site_initialisation: Initialisation::Uniform }.build();
        let lattice = Lattice::new(settings);
        assert_eq!(lattice.get(0).read().unwrap().next[0].as_ref().unwrap().read().unwrap().position, 1);
        assert_eq!(lattice.get(63).read().unwrap().next[0].is_none(), true);
        assert_eq!(lattice.get(0).read().unwrap().previous[0].is_none(), true);
    }

    #[test]
    fn test_lattice_local_energy() {
        let settings = SettingsBuilder { beta: 1.0, boundary_conditions: BoundaryConditions::Periodic, site_initialisation: Initialisation::Uniform }.build();
        let lattice = Lattice::new(settings);
        let mut energy = 0.0;
        for i in 0..lattice.sites.len() {
            let site = lattice.get(i);
            energy += site.read().unwrap().local_energy();
        }
        assert_eq!(energy, -384.0);


        let settings = SettingsBuilder { beta: 1.0, boundary_conditions: BoundaryConditions::Open, site_initialisation: Initialisation::Uniform }.build();
        let lattice = Lattice::new(settings);
        let mut energy = 0.0;
        for i in 0..lattice.sites.len() {
            let site = lattice.get(i);
            energy += site.read().unwrap().local_energy();
        }
        assert_eq!(energy, -342.0);
    }

    #[test]
    fn test_periodic_boundary_conditions() {
        let settings = SettingsBuilder { beta: 1.0, boundary_conditions: BoundaryConditions::Periodic, site_initialisation: Initialisation::Uniform }.build();
        let lattice = Lattice::new(settings);
        
        for i in 0..lattice.sites.len() {
            for d in 0..DIMENSIONS {
                assert_eq!(lattice.get(i).read().unwrap().next[d].as_ref().unwrap().read().unwrap().id, lattice.get(next_position(i, d)).read().unwrap().id);
                assert_eq!(lattice.get(i).read().unwrap().previous[d].as_ref().unwrap().read().unwrap().id, lattice.get(previous_position(i, d)).read().unwrap().id);
            }
        }
    }

    #[test]
    fn test_lattice_montecarlo_sweep() {
        let settings = SettingsBuilder { beta: 1.0, boundary_conditions: BoundaryConditions::Periodic, site_initialisation: Initialisation::Uniform }.build();
        let mut lattice = Lattice::new(settings);
        lattice.montecarlo_sweep();
    }
}