use std::{fmt::Display, ops};

use chrono::{DateTime, Local, TimeZone};
use rand::{
    distributions::{Standard, Uniform},
    prelude::Distribution,
    seq::SliceRandom,
};
use rust_decimal::{prelude::FromPrimitive, Decimal};

use thousands::Separable;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub preferred_coin: Coin,
}

#[derive(Debug, Default, Clone)]
pub struct Contact {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Default)]
pub struct DashboardData {
    pub personal_debt: MoneyAmount,
    pub contact_debt: MoneyAmount,
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
    pub amount: MoneyAmount,
    pub debt_type: DebtType,
    pub date: DateTime<Local>,
    pub is_paid: bool,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MoneyAmount {
    pub amount: f64,
    pub coin: Coin,
}

impl ops::Add<&MoneyAmount> for &MoneyAmount {
    type Output = MoneyAmount;

    fn add(self, rhs: &MoneyAmount) -> Self::Output {
        let coin = if rhs.coin == self.coin {
            self.coin.clone()
        } else {
            Coin::Unknown
        };
        let amount = self.amount + rhs.amount;

        MoneyAmount { coin, amount }
    }
}
impl ops::Sub<&MoneyAmount> for &MoneyAmount {
    type Output = MoneyAmount;

    fn sub(self, rhs: &MoneyAmount) -> Self::Output {
        let coin = if rhs.coin == self.coin {
            self.coin.clone()
        } else {
            Coin::Unknown
        };
        let amount = self.amount - rhs.amount;

        MoneyAmount { coin, amount }
    }
}

impl Display for MoneyAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_amount = Decimal::from_f64(self.amount)
            .expect("Number couldn't be converted to Decimal!")
            .abs()
            .round_dp(2)
            .separate_with_commas();
        write!(f, "{}. {}", self.coin, formatted_amount)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Coin {
    #[default]
    Quetzal,
    Dollar,
    Unknown,
}

impl Display for Coin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Coin::Quetzal => write!(f, "Q"),
            Coin::Dollar => write!(f, "$"),
            Coin::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Coin {
    pub fn to_currency_code(&self) -> &'static str {
        match self {
            Coin::Quetzal => "GTQ",
            Coin::Dollar => "USD",
            Coin::Unknown => "Unknown",
        }
    }
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

impl Distribution<MoneyAmount> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> MoneyAmount {
        let dist = Uniform::new(10.0, 750.0);
        let amount = dist.sample(rng);
        let coin = rng.gen();
        MoneyAmount { amount, coin }
    }
}

impl Distribution<Coin> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Coin {
        if rng.gen_bool(0.5) {
            Coin::Quetzal
        } else {
            Coin::Dollar
        }
    }
}

impl Distribution<Contact> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Contact {
        let id = rng.gen();
        let name = NAMES.choose(rng).unwrap().to_string();

        Contact { id, name }
    }
}

impl Distribution<Debt> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Debt {
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
        let debt_type = rng.gen();
        let contact = rng.gen();
        let is_paid = rng.gen();
        let amount = rng.gen();
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
