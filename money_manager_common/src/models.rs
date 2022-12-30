use rand::{
    distributions::{Standard, Uniform},
    prelude::Distribution,
    seq::SliceRandom,
};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Default, Clone)]
pub struct Contact {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Default)]
pub struct DashboardData {
    pub personal_debt: f64,
    pub contact_debt: f64,
    pub history: Vec<Debt>,
}

#[derive(Debug, Default)]
pub struct Debt {
    pub id: u32,
    pub contact: Contact,
    pub amount: f64,
    pub debt_type: DebtType,
}

#[derive(Debug, Default)]
pub enum DebtType {
    #[default]
    PersonalDebt,
    ContactDebt,
}

static NAMES: [&str; 10] = [
    "Rachael Curry",
    "Brynn Kelly",
    "Damion Sparks",
    "Edward Holmes",
    "Marques Pacheco",
    "Anya Robinson",
    "Jazmyn Mata",
    "Carissa Blevins",
    "Nathalie Vega",
    "Alan Leon",
];

impl Distribution<Contact> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Contact {
        let id = rng.gen();
        let name = NAMES.choose(rng).unwrap().to_string();

        Contact { id, name }
    }
}

impl Distribution<Debt> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Debt {
        let dist = Uniform::new(10.0, 750.0);

        let id = rng.gen();
        let amount = dist.sample(rng);
        let debt_type = rng.gen();
        let contact = rng.gen();
        Debt {
            id,
            contact,
            amount,
            debt_type,
        }
    }
}

impl Distribution<DebtType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> DebtType {
        if rng.gen_bool(0.5) {
            DebtType::PersonalDebt
        } else {
            DebtType::ContactDebt
        }
    }
}
