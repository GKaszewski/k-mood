mod false_discovery;
mod incomplete_beta;
mod log_gamma;
mod normal;
mod p_value;
mod student_t;
mod two_proportion;
mod welch;

pub use false_discovery::FalseDiscoveryControl;
pub use p_value::PValue;
pub use student_t::student_t_two_sided;
pub use two_proportion::two_proportion;
pub use welch::{welch_satterthwaite_degrees_of_freedom, welch_two_sample};
