use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: &String,
        to: &String,
        amount: u128,
    ) -> Result<(), &'static str> {
        /* TODO:
            - Get the balance of account `caller`.
            - Get the balance of account `to`.

            - Use safe math to calculate a `new_caller_balance`.
            - Use safe math to calculate a `new_to_balance`.

            - Insert the new balance of `caller`.
            - Insert the new balance of `to`.
        */
        let caller_balance: u128 = self.balance(&caller);
        let to_balance: u128 = self.balance(&to);
        let new_caller_balance: u128 = caller_balance
            .checked_sub(amount)
            .ok_or("Not enough funds.")?;
        let new_to_balance: u128 = to_balance
            .checked_add(amount)
            .ok_or("Overflow.")?;

        self.set_balance(caller, new_caller_balance);
        self.set_balance(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        /* TODO: Create a test that checks the following:
            - That `alice` cannot transfer funds she does not have.
            - That `alice` can successfully transfer funds to `carl`.
            - That the balance of `alice` and `bob` is correctly updated.
        */

        let alice = &"alice".to_string();        
        let bob = &"bob".to_string();
        let carl = &"carl".to_string();

        let alice_init_balance = 100;
        let bob_init_balance = std::u128::MAX;

        let mut balances = super::Pallet::new();
        balances.set_balance(alice, alice_init_balance);
        let t1 = balances.transfer(alice, bob, 101);
        assert!(t1.is_err());
        assert_eq!(t1.unwrap_err(), "Not enough funds.");
        assert_eq!(balances.balance(alice), 100);

        balances.set_balance(bob, bob_init_balance);
        let t2 = balances.transfer(alice, bob, 1);
        assert!(t2.is_err());
        assert_eq!(t2.unwrap_err(), "Overflow.");
        assert_eq!(balances.balance(alice), 100);
        assert_eq!(balances.balance(bob), bob_init_balance);

        balances.set_balance(carl, 10);
        let t3 = balances.transfer(alice, carl, 1);
        assert!(! t3.is_err());
        assert_eq!(balances.balance(alice), 99);
        assert_eq!(balances.balance(carl), 11);

        Ok::<(), &'static str>(());
    }
}
