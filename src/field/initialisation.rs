#[derive(Debug, PartialEq, Clone, Copy, clap::ValueEnum)]
pub enum Initialisation {
    Random,
    Uniform,
}
