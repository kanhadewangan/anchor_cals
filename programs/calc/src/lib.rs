use anchor_lang::prelude::*;

#[derive(Debug)]
struct Rect{
    w:u32,
    h:u32
}
impl Rect {
    fn self(&self)->u32{
        self.w *self.h;
    }
}

declare_id("")

#[program]
pub mod anchor_calc{
    use super::*;
        fn init(ctx:Context<Initialize>)->Result<()>{
            ctx.accounts.account.data =1;
            OK(()),
        }
        fn duble(ctx:Context<Duble>)->Result<()>{
            ctx.accounts.account.data = ctx.accounts.account.data*2;
            OK(())
        }
        fn add(ctx:Context<Add>,amount:u32)->Result<()>{
            ctx.accounts.account.data = ctx.accounts.account.data+amount;
            OK(())
        }
}

