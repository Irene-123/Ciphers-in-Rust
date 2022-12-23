#![allow(dead_code)]

Enum Card{
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand{
    cards: Vec<Card>, 
}


impl Hand{
    fn new()-> Self{
        Hand{
            cards: vec![]
        }
    }

    fn add(&mut self, card: Card){
        self.cards.push(card); 
    }

    fn value(&self)-> usize{
        use Card::*; 
        let mut subtotal=0; 
        let mut aces=0; 

         for card in &self.cards.iter() {
            subtotal+= match card{
                Ace=> {aces+=1; 0}
                Two => 2,
                Three => 3,
                Four => 4,
                Five => 5,
                Six => 6,
                Seven => 7,
                Eight => 8,
                Nine => 9,
                Jack => 10,
                Queen => 10,
                King => 10,
            }
         }

         for _ in 0..aces{
            let acevalue= if subtotal <= 10 {11} else {1};
            subtotal += acevalue; 
         }
         subtotal
    }
    fn is_loosing_hand(&self)->bool{
        self.value() > 21
    }
}

fn main() {
    println!("Getting the value of all cards present! \n");

    let mut hand= Hand::new(); 
    hand.add(Card::King); 
    hand.add(Card::Ace); 
}
