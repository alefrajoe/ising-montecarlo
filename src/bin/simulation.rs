use ising_montecarlo::geometry::lattice_geometry::lattice::Lattice;
use ising_montecarlo::settings::SettingsBuilder;
use ising_montecarlo::geometry::lattice_geometry::boundary_conditions::BoundaryConditions;
use ising_montecarlo::field::initialisation::Initialisation;

fn main() {
    let settings = SettingsBuilder { beta: 1.0, boundary_conditions: BoundaryConditions::Periodic, site_initialisation: Initialisation::Random }.build();
    let mut lattice = Box::new(Lattice::new(settings));

    println!("Running simulation...");
    println!("Lattice size: {}", lattice.settings.lattice_size);
    println!("Beta: {}", lattice.settings.beta);
    println!("Boundary conditions: {:?}", lattice.settings.boundary_conditions);
    println!("Site initialisation: {:?}", lattice.settings.site_initialisation);
    println!("Dimensions: {}", lattice.settings.dimensions);
    println!("Lattice size: {}", lattice.settings.lattice_size);

    for _ in 0..100000 {
        lattice.montecarlo_sweep();
        println!("Energy: {}", lattice.get_energy());
    }
}