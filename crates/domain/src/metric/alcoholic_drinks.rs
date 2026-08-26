use crate::macros::bounded_metric;

const MIN_ALCOHOLIC_DRINKS: u16 = 0;
const MAX_ALCOHOLIC_DRINKS: u16 = 60;

bounded_metric!(
    AlcoholicDrinks,
    u16,
    MIN_ALCOHOLIC_DRINKS,
    MAX_ALCOHOLIC_DRINKS,
    "alcoholic drinks"
);
