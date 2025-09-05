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
            balance: 0,
            holder,
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
        format!("{} has a balance: {}", self.holder, self.balance)
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

fn main() {
    let mut bank = Bank::new();

    let account = Account::new(1, String::from("me"));
    let accountTwo = Account::new(1, String::from("another"));

    bank.add_account(account);
    bank.add_account(accountTwo);

    bank.accounts[0].deposit(100);
    bank.accounts[1].deposit(200);

    println!("{:#?}", bank);
    println!("{:#?}", bank.accounts[0].summary());
    println!("{:#?}", bank.accounts[1].summary());
    println!("{:#?}", bank.total_balance());
    println!("{:#?}", bank.summary());
}
