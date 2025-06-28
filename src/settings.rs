#[cfg(not(test))]
pub const DIMENSIONS: usize = 2;
#[cfg(test)]
pub const DIMENSIONS: usize = 3; //---TESTS---

#[cfg(not(test))]
pub const LATTICE_SIZE: usize = 20;
#[cfg(test)]
pub const LATTICE_SIZE: usize = 4; //---TESTS---

// ------------------- SETTINGS -------------------

use crate::field::initialisation::Initialisation;
use crate::geometry::lattice_geometry::boundary_conditions::BoundaryConditions;

#[derive(Clone)]
pub struct Settings {
    pub dimensions: usize,
    pub lattice_size: usize,
    pub beta: f64,
    pub boundary_conditions: BoundaryConditions,
    pub site_initialisation: Initialisation,
}

pub struct SettingsBuilder {
    pub beta: f64,
    pub boundary_conditions: BoundaryConditions,
    pub site_initialisation: Initialisation,
}

impl Default for SettingsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsBuilder {
    pub fn new() -> SettingsBuilder {
        Self {
            beta: 0.0,
            boundary_conditions: BoundaryConditions::Periodic,
            site_initialisation: Initialisation::Uniform,
        }
    }

    pub fn add_beta(&mut self, beta: f64) -> SettingsBuilder {
        self.beta = beta;
        Self {
            beta,
            boundary_conditions: self.boundary_conditions,
            site_initialisation: self.site_initialisation,
        }
    }

    pub fn add_boundary_conditions(
        &mut self,
        boundary_conditions: BoundaryConditions,
    ) -> SettingsBuilder {
        self.boundary_conditions = boundary_conditions;
        Self {
            beta: self.beta,
            boundary_conditions,
            site_initialisation: self.site_initialisation,
        }
    }

    pub fn add_site_initialisation(
        &mut self,
        site_initialisation: Initialisation,
    ) -> SettingsBuilder {
        self.site_initialisation = site_initialisation;
        Self {
            beta: self.beta,
            boundary_conditions: self.boundary_conditions,
            site_initialisation,
        }
    }

    pub fn build(self) -> Settings {
        Settings {
            dimensions: DIMENSIONS,
            lattice_size: LATTICE_SIZE,
            beta: self.beta,
            boundary_conditions: self.boundary_conditions,
            site_initialisation: self.site_initialisation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_builder_add_beta() {
        let builder = SettingsBuilder::new()
            .add_beta(1.0)
            .add_boundary_conditions(BoundaryConditions::Periodic)
            .add_site_initialisation(Initialisation::Uniform);
        let settings = builder.build();
        assert_eq!(settings.beta, 1.0);
        assert_eq!(settings.boundary_conditions, BoundaryConditions::Periodic);
        assert_eq!(settings.site_initialisation, Initialisation::Uniform);
    }

    #[test]
    fn test_settings_builder() {
        let settings = SettingsBuilder {
            beta: 1.0,
            boundary_conditions: BoundaryConditions::Periodic,
            site_initialisation: Initialisation::Uniform,
        }
        .build();
        assert_eq!(settings.dimensions, DIMENSIONS);
        assert_eq!(settings.lattice_size, LATTICE_SIZE);
        assert_eq!(settings.beta, 1.0);
        assert_eq!(settings.boundary_conditions, BoundaryConditions::Periodic);
        assert_eq!(settings.site_initialisation, Initialisation::Uniform);
    }
}
