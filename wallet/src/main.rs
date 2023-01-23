use std::fmt;
use std::ops::{AddAssign, SubAssign};

#[derive(Debug, PartialEq, PartialOrd)]
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

    fn withdraw(&mut self, amount: Bitcoin) -> Result<(), InsufficientFundsError> {
        match amount > self.balance {
            true => Err(InsufficientFundsError),
            false => {
                self.balance -= amount;
                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct InsufficientFundsError;

impl fmt::Display for InsufficientFundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Insufficient funds!")
    }
}

fn main() {
    let mut wallet = Wallet::new(Bitcoin(25));
    assert_eq!(&wallet.balance, &Bitcoin(25));
    println!("Starting with {} in wallet", &wallet.balance);

    wallet.deposit(Bitcoin(10));
    assert_eq!(&wallet.balance, &Bitcoin(35));
    println!("Now have {}", &wallet.balance);

    let withdrawal_amount = Bitcoin(120);
    match wallet.withdraw(withdrawal_amount) {
        Ok(_) => assert_eq!(&wallet.balance, &Bitcoin(15)),
        Err(err) => println!("Error: {:?} have {}", err, &wallet.balance),
    }
    println!("Finished with {}", &wallet.balance);
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
    fn test_wallet_withdrawal_sufficient_funds() {
        let mut wallet = wallet(20);
        assert!(wallet.withdraw(Bitcoin(10)).is_ok());
        assert_balance(wallet, Bitcoin(10));
    }

    #[test]
    fn test_wallet_withdrawal_insufficient_funds() {
        let mut wallet = wallet(20);
        assert_eq!(wallet.withdraw(Bitcoin(25)), Err(InsufficientFundsError));
        assert_balance(wallet, Bitcoin(20));
    }
}
