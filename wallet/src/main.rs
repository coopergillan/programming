use std::fmt;
use std::ops::{AddAssign, SubAssign};

#[derive(Debug, PartialEq)]
struct Bitcoin(isize);

// A little surprised that this boilerplate code is needed and cannot be automatically derived
impl AddAssign for Bitcoin {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}

impl SubAssign for Bitcoin {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0
    }
}

impl fmt::Display for Bitcoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} BTC", self.0)
    }
}

struct Wallet {
    balance: Bitcoin,
}

impl Wallet {
    fn new(starting_balance: Bitcoin) -> Self {
        Wallet {
            balance: starting_balance,
        }
    }

    fn deposit(&mut self, amount: Bitcoin) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: Bitcoin) {
        self.balance -= amount;
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wallet(starting_balance: isize) -> Wallet {
        Wallet::new(Bitcoin(starting_balance))
    }

    fn assert_balance(wallet: Wallet, want: Bitcoin) {
        let got = wallet.balance;
        assert_eq!(got, want, "got {} want {}", got, want);
    }

    #[test]
    fn test_wallet_deposit() {
        let mut wallet = wallet(0);
        wallet.deposit(Bitcoin(10));
        assert_balance(wallet, Bitcoin(10));
    }

    #[test]
    fn test_wallet_withdrawal() {
        let mut wallet = wallet(20);
        wallet.withdraw(Bitcoin(10));
        assert_balance(wallet, Bitcoin(10));
    }
}
