#[derive(Debug, PartialEq, Clone, Copy, clap::ValueEnum)]
pub enum BoundaryConditions {
    Periodic,
    Open,
}
