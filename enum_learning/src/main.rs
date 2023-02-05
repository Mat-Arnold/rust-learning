#[derive(Debug)]
enum UsState 
{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
}

enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8
{
    return match coin 
    {
        Coin::Penny     => 
        {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel    => 5,
        Coin::Dime      => 10,
        Coin::Quarter(state)   =>
        {
            println!("State quater from {:?}!",state);
            25
        }
    };
}

fn main() 
{
    
}
