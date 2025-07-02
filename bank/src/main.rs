#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn add_account(bank: &mut Bank, account: Account) {
    bank.accounts.push(account);
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(
        1,
        String::from("me"), // "me" is a slice
    );

    add_account(&mut bank, account);
    println!("{:#?}", bank);
    let account_ref1 = &bank.accounts[0];
    let account_ref2 = &bank.accounts[0];
    print_account(account_ref1);
    print_account(account_ref2);
    println!("{:#?}", bank.accounts[0]);
}
