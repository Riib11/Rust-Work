/* - - - - - - - - - -
* number_to_words.rs \\
* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - *
*
* Author: Henry Blanchette
* Organization: independent, computer science student at Reed College
* Github: github.com/riib11/Parity/RustIntro
*
* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - *
*
* Prompt, from Parity:
* could you write a Rust program that accepts a numeric literal
* through stdin and writes out a human-readable english form of it?
* 
* ex:
* 1355823 ->
*     "one million, three hundred fifty five thousand, eight hundred twenty three"
*
* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - *
*
*/


use std::fmt;
use std::io;

/*
*
* >> NumBlock <<
*
* Stores sets of 3 digits.
* This is useful because verbally that's how we
* seperate thousands, millions, etc.
*/

#[derive(Debug)]
struct NumBlock {
    index : u32, // index
    last : bool, // indicates if last in sequence
    ones : u32, // ones
    tens : u32, // tens
    huns : u32 // hundreds
}

impl NumBlock {
    
    fn is_empty(&self) -> bool {
        self.huns == 0 &&
        self.tens == 0 &&
        self.ones == 0
    }

}

static ONES    : &'static [&str;10] = &["zero","one","two","three","four","five","six","seven","eight","nine"];
static TEENS   : &'static [&str;10] = &["","eleven","twelve","thirteen","fourteen","fifteen","sixteen","seventeen","eighteen","nineteen"];
static TENS    : &'static [&str;10] = &["","ten","twenty","thirty","fourty","fifty","sixty","seventy","eighty","ninety"];
static ENDINGS : &'static [&str;16]  = &["","thousand","million","billion","trillion","quadrillion","quintillion","sextillion","septillion","octillion","nonillion","decillion","undecillion","duodecillion","tredecillion","quattuordecillion"];

// formats a NumBlock into word form
impl fmt::Display for NumBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let mut words = Vec::new();

        if !self.last &&
            !self.is_empty()    {
                                    words.push(", ")
                            }

        if self.huns != 0       {   
                                    words.push( ONES[self.huns as usize] );
                                    words.push( " " );
                                    words.push( "hundred" );
                            }
        
        // deal with teens
        if self.tens == 1
            && self.ones != 0   {   
                                    if self.huns != 0 { words.push( " " ) };
                                    words.push( TEENS[self.ones as usize] );
                                    
                            }
        
        // not teens
        else {

            if self.tens != 0   { 
                                    if self.huns != 0 { words.push( " " ); }
                                    words.push( TENS[self.tens as usize] );
                            }
            
            if self.ones != 0   {   
                                    if self.tens != 0 ||
                                        self.huns != 0 { words.push( " " ); }
                                    words.push( ONES[self.ones as usize] );
                            }
        }

        
        if  self.index != 0 &&
            !self.is_empty()    {   words.push( " " );
                                    words.push( ENDINGS[self.index as usize] );
                            }

        
        // display as full sentence
        let sentence : String = words.into_iter().collect();
        write!(f, "{}", sentence)
        
    }
}

impl NumBlock {

    /* 
    * reads input and outputs
    * the resulting collection of NumBlocks
    * in word form
    */

    fn to_num_blocks(s:&str) -> Vec<NumBlock> {
        
        // vector of digits representing reverese of input string
        let mut nums = Vec::new(); 
        for c in s.chars() { nums.push(char_to_int(c)); }
        nums.reverse();

        let mut nbs = Vec::new(); // vector of NumBlocks, in reverse order as well (stating with NumBlock index = 0)
        let mut cur = [0,0,0]; // the three digits for the current NumBlock in progess
        let mut cur_i = 0; // index within the current NumBlock
        let mut nb_i : u32 = 0; // current NumBlock index
        let mut last = false; // indicates last block in sequence

        // translate digits into NumBocks
        for i in 0..nums.len() {
            
            cur[cur_i] = nums[i]; // set the current number
            if i == nums.len()-1 { last = true; }
            if (i+1) % 3 == 0 {
                // create the completed NumBlock
                let nb = NumBlock{ index:nb_i, last:last, ones:cur[0], tens:cur[1], huns:cur[2] };
                nbs.push( nb );

                nb_i += 1; // advance NumBlock index every three
                cur_i = 0; // reset index within new NumBlock
                cur = [0,0,0]; // reset digits for new NumBlock
            }
            else {
                cur_i += 1; // advance index within current NumBlock
            }
        }
        
        // if the last NumBlock wasn't quite full, create it (unfilled digits are 0)
        if cur[0] != 0 || cur[1] != 0 || cur[2] != 0 {
            let nb = NumBlock{ index:nb_i, last:last, ones:cur[0], tens:cur[1], huns:cur[2] };
            nbs.push( nb );
        }

        nbs
    }

    // display NumBlocks all together, as a 'sentence'
    fn display_full(mut nbs:Vec<NumBlock>) {
        nbs.reverse(); // since we decide to verbalize numbers this way
        for nb in nbs {
            print!("{}", nb);
        }
        println!(""); // return
    }

}

fn char_to_int(c:char) -> u32 {
    let r : u32 = c.to_string().parse().unwrap();
    
    r
}

// read input and print output
fn process_input() {

    let mut input = String::new(); // user input

    match io::stdin().read_line(&mut input) {

        // convert to word form and display!
        Ok(_bits)    => {
            let input_filtered = &input[..input.len()-1];
            NumBlock::display_full(NumBlock::to_num_blocks( &input_filtered ));
        }
            
        // any error
        Err(error)  => println!("[!] {}", error),
    }   
}

fn main() {

    loop { process_input(); }

}