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

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("{} has a balance {}", self.holder, self.balance)
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

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn make_and_print_account() {
    // lifetime only within this scope
    let account = Account::new(1, String::from("you"));
    println!("irrelevant account -- {:#?}", account);
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(
        1,
        String::from("me"), // "me" is a slice
    );

    account.deposit(500);
    account.withdraw(250);
    println!("account summary -- {:#?}", account.summary());

    bank.add_account(account);
    println!("{:#?}", bank);

    let account_ref1 = &bank.accounts[0];
    let account_ref2 = &bank.accounts[0];
    print_account(account_ref1);
    print_account(account_ref2);
    println!("account reference summary -- {:#?}", account_ref2.summary());

    println!(
        "bank account -- {} {:#?}",
        bank.accounts[0].id, bank.accounts[0]
    );
    println!("bank summary -- {:#?}", bank.summary());
    println!("bank total balance -- {:#?}", bank.total_balance());

    make_and_print_account();
}
