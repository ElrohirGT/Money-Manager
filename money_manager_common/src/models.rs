use chrono::{DateTime, Local, TimeZone};
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
    pub active_debts: Vec<Debt>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateDivision {
    pub year: i32,
    pub month: u32,
}

#[derive(Debug, Default, Clone)]
pub struct Debt {
    pub id: u32,
    pub contact: Contact,
    pub amount: f64,
    pub debt_type: DebtType,
    pub date: DateTime<Local>,
    pub is_paid: bool,
}

#[derive(Debug, Default, Clone)]
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
        let year_dist = Uniform::new_inclusive(2018, 2022);
        let month_dist = Uniform::new_inclusive(1, 12);
        let day_dist = Uniform::new_inclusive(1, 27);
        let hour_dist = Uniform::new_inclusive(0, 23);
        let minute_dist = Uniform::new_inclusive(0, 59);

        let year = year_dist.sample(rng);
        let month = month_dist.sample(rng);
        let day = day_dist.sample(rng);
        let hours = hour_dist.sample(rng);
        let minutes = minute_dist.sample(rng);
        let seconds = minute_dist.sample(rng);

        let id = rng.gen();
        let amount = dist.sample(rng);
        let debt_type = rng.gen();
        let contact = rng.gen();
        let is_paid = rng.gen();
        log::debug!(
            "Trying to create date: {}/{}/{}T{}:{}:{}",
            year,
            month,
            day,
            hours,
            minutes,
            seconds
        );
        let date = Local
            .with_ymd_and_hms(year, month, day, hours, minutes, seconds)
            .earliest()
            .expect("Failed to generate random date!");

        Debt {
            id,
            contact,
            amount,
            debt_type,
            date,
            is_paid,
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
