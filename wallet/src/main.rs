use std::ops::AddAssign;

#[derive(Debug, PartialEq)]
struct Bitcoin(isize);

impl AddAssign for Bitcoin {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}

struct Wallet {
    balance: Bitcoin,
}

impl Wallet {
    fn new() -> Self {
        Wallet {
            balance: Bitcoin(0),
        }
    }

    fn deposit(&mut self, amount: Bitcoin) {
        self.balance += amount;
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet() {
        let mut wallet = Wallet::new();

        wallet.deposit(Bitcoin(10));
        let got = wallet.balance;
        let want = Bitcoin(10);

        assert_eq!(got, want, "got {:?} want {:?}", got, want);
    }
}
