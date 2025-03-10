pub trait Payment {
    fn payment_method(&self,deposited: u64);
}
