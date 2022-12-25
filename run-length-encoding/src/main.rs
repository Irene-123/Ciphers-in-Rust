
mod run_length_encoding{

fn encoded(text: &str)-> String{
    let mut count=0; 
    let mut encoded= String::new(); 
    let mut prev: Option<char>= None;
    
    while let Some(c)= text.chars().next() {
        if prev.is_none() {
            prev= Some(c); 
        }

        let _prev= prev.unwrap(); 
        if _prev != c {
            encoded.push_str(&format!("{}{}", count, _prev)); 
        }
        count+=1; 
    }

    if prev.is_some(){
        encoded.push_str(&format!("{}{}", count, prev.unwrap()));
    }
    encoded
}
}

fn main() {
    
}


#[test]
fn abc(){
    use run_length_encoding::*; 
    assert_eq!(encoded("abc"), "1a1b1c"); 
}

#[test]
fn round_trip(){
    use run_length_encoding::*; 
    let input= "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA"; 
    assert_eq!(encoded(input), "5A1 9A1A1 9A9A2A");
}