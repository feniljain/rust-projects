use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("!!!!Guess The Number!!!!");

    let mut rng = thread_rng();
    let secret_number: u32 = rng.gen_range(0, 101);

    //println!("{}", secret_number);

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to take guess input!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Number you guessed: {}", guess);

        //match guess.cmp(&secret_number.to_string()) {
        //    Ordering::Less => println!("Too small"),
        //    Ordering::Greater => println!("Too big"),
        //    Ordering::Equal => println!("Too win"),
        //}

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

/*
    Explain better what we are going to cover today and why it is different
    Better example of ledger, leverage bank example
    Still high terms which are tossed
    I like the village example, maybe change the example to rules in old times
    Mark the difference between blockchain and bitcoin
    Show the message which was in first tnx.
    Cutting down bank part is wrong, its banned coz of illegal activities
    Too much toogle between blockhain and bitcoin in the starting
    Make meaning of decentralized more clear by making taking example of RBI(put in Scam 1992 if needed)
    Include problems with centralization
    Make P2P more clear
    Give them an idea of how hard it would be to fork a new branch by giving them a good scope
    * -> Was the fork in ethereum due to this computation problem from someone(acc. to me its an wrong example)
    Dark Web was not the cause of bitcoin, but it was actually inverse in payment industrty of dark web
    Maybe we can introduce public and private key in a better way, and explain how wallet is actually like the UPI example is good but keep it in a more better way
    Nonce isnt explained properly
    Try to include the meaning of birthday problem and tell how rare is it to get same seed, and as it is a one way system one has to bruteforce it
    Role of miners is still kind of vague
    Introduce smart contracts in a better way
    Why was setMessage function there?
    Explain code more as analogy to other programming language instead of what it does
    Maybe explain a better example in smart contract code
    Introduce tera hackathon after "Where to go from here?"
*/
/*
 * Still not clear what is ledger
 * Inter is still not that natural
 * Think about how you approach inter
 * Do not use wallet and all in starting
 * Ishan talked too much
*/
