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
            println!("State quarter from {:?}!",state);
            25
        }
        _ => 0, // All other inputs return 0 (not necessary because rust can see I've used all the enum options)
    };
}

fn main() 
{
    let test_quarter = Coin::Quarter(UsState::Alabama);
    let value = value_in_cents(&test_quarter);
    println!("The value is {} cents.",value);
}
