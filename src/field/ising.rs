use crate::field::schema::Field;
use crate::field::initialisation::Initialisation;

#[derive(Debug, PartialEq)]
pub enum IsingField {
    Up,
    Down,
}

impl IsingField {
    pub fn new(initialisation: Initialisation) -> Self {
        match initialisation {
            Initialisation::Random => {
                if rand::random::<f64>() < 0.5 {
                    IsingField::Up
                } else {
                    IsingField::Down
                }
            }
            Initialisation::Uniform => {
                IsingField::Up
            }
        }
    }
}

impl Field<IsingField> for IsingField {
    fn interaction(&self, site: &IsingField) -> f64 {
        match self {
            IsingField::Up => match site {
                IsingField::Up => 1.0,
                IsingField::Down => -1.0,
            },
            IsingField::Down => match site {
                IsingField::Up => -1.0,
                IsingField::Down => 1.0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ising_field_interaction() {
        let up = IsingField::Up;
        let down = IsingField::Down;
        assert_eq!(up.interaction(&up), 1.0);
        assert_eq!(up.interaction(&down), -1.0);
        assert_eq!(down.interaction(&up), -1.0);
        assert_eq!(down.interaction(&down), 1.0);
    }

    #[test]
    fn test_ising_field_new() {
        let random = IsingField::new(Initialisation::Random);
        let uniform = IsingField::new(Initialisation::Uniform);
        assert!(random == IsingField::Up || random == IsingField::Down);
        assert_eq!(uniform, IsingField::Up);
    }   
}   