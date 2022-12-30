#[derive(Debug, Default)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Default)]
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
