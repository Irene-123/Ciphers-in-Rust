use std::io; 
use std::str;


mod Vernam {
    use ascii::ToAsciiChar;

    pub fn encrypt(message: &str, key: &str)->String{
        
        let mut message_vec: Vec<u8> = Vec::new();
        for ch in message.chars(){
            message_vec.push((ch as u8)- 64);  
        }
        let mut key_vec: Vec<u8> = Vec::new();
        for ch in key.chars(){
            key_vec.push((ch as u8)- 64);  
        }


        let mut cipherText= String::new(); 
        for i in 1..15{
            let x= ((message_vec[i]^key_vec[i])%26) + 64; 
            // println!("{} {} {}", message_vec[i], key_vec[i], x);
            let c= x.to_ascii_char();
       
            cipherText.push(c.unwrap().into());
        } 
        cipherText
    }
}

fn main() {
      
    let key: &str= "BNHSDGDHWRETOKM"; //15 
    
    let message: &str= "BLOCKCHAINRULES"; // 15
    let cipherText: String= Vernam::encrypt(message, key); 
    println!("CipherText after performing Vernam encryption: {}", cipherText); 
}
