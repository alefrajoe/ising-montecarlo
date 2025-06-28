use clap::Parser;
use ising_montecarlo::geometry::lattice_geometry::lattice::Lattice;
use ising_montecarlo::settings::SettingsBuilder;
use ising_montecarlo::geometry::lattice_geometry::boundary_conditions::BoundaryConditions;
use ising_montecarlo::field::initialisation::Initialisation;
use rayon;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Inverse temperature beta (β = 1/kT)
    #[arg(long, default_value_t = 1.0)]
    beta: f64,

    /// Number of Monte Carlo sweeps
    #[arg(long, default_value_t = 100000)]
    sweeps: u32,

    /// Boundary conditions (Periodic or Fixed)
    #[arg(long, default_value = "periodic")]
    boundary: BoundaryConditions,

    /// Initial state (Random or Uniform)
    #[arg(long, default_value = "random")]
    init: Initialisation,
}

fn main() {
    let args = Args::parse();
    
    println!("Number of threads: {}", rayon::current_num_threads());

    let settings = SettingsBuilder {
        beta: args.beta,
        boundary_conditions: args.boundary,
        site_initialisation: args.init,
    }.build();

    let mut lattice = Box::new(Lattice::new(settings));

    println!("Running simulation...");
    println!("Lattice size: {}", lattice.settings.lattice_size);
    println!("Beta: {}", lattice.settings.beta);
    println!("Boundary conditions: {:?}", lattice.settings.boundary_conditions);
    println!("Site initialisation: {:?}", lattice.settings.site_initialisation);
    println!("Dimensions: {}", lattice.settings.dimensions);
    println!("Lattice size: {}", lattice.settings.lattice_size);

    for _ in 0..args.sweeps {
        lattice.montecarlo_sweep();
        println!("Energy: {}", lattice.get_energy());
    }
}