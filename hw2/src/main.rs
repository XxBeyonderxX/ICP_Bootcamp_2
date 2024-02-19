use std::io;
struct WordCounter{
    text: String,
}
impl WordCounter{
    fn new(text:&str)->WordCounter{
        WordCounter{
            text: text.to_string()
        }
    }
    fn count_words(&self)->usize{
        let mut count:usize=0;
        for c in self.text.split_whitespace(){
            count+=1;
        }

        return count;
    }
}
fn main() {
    let mut text = String::new();
    println!("Enter Text");
    io::stdin().read_line(&mut text).expect("Failed to read line");
    let x=WordCounter:: new(&text);
    let c=x.count_words();
    if c>0{
        println!("Text has {c} words");
    }
    else{
        println!("Incorrect Input");
    }


}
