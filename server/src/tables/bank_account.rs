use spacetimedb::Identity;

#[spacetimedb::table(name = bank_account)]
pub struct BankAccount {
    #[primary_key]
    pub identity: Identity,

    pub balance: u64,
    pub created_at: i64,
    pub last_transaction: i64,
}
