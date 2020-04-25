use ron::de::{from_reader}; 
use serde::Deserialize;
use std::{fs::File, collections::HashMap , io::self};
use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Sabi")]
/// A commandline language learning tool. 
///
/// If you want to use all of the vocabulary in your dictionary,
/// just leave out any extra flags.
struct Arguments {
    // TODO Argument for future release | Expects list - read key + value through stdin
    // a: String,
    // -d | to select dictionary
    //#[structopt(short = "d", long = "dictionary", help = "Insert dictionary", parse(from_os_str))]  
    #[structopt(short = "d", long = "dictionary", help = "Insert dictionary")]  
    //dictionary: std::path::PathBuf,
    dictionary: String,
    // -h | hiragana - neither k nor h flag: both | can't enable both 
    #[structopt(short = "h", long = "hira", help = "Set Hiragana mode")]  
    hira: bool,
    // -k | katakana
    #[structopt(short = "k", long = "kata", help = "Set Katakana mode")]  
    kata: bool,
    // -n | kanji 
    #[structopt(short = "n", long = "kanji", help = "Set Kanji mode")]  
    kanji: bool,
    // TODO -r | -> Reverse (Show romaji write kana ) | Might be troublesome with terminal and its reading inputs
    // n: bool,
}

struct Result{
    right: u32,
    wrong: u32, 
}

impl Result {
    pub fn print_result(self) {
        if self.right > self.wrong {
            println!("Nice! You had {} right and {} wrong!", self.right, self.wrong);
        }else {
            println!("You had {} right and {} wrong!", self.right, self.wrong);

        }

    }

}

#[derive(Debug, Deserialize)]
struct Kana{
    hiragana: HashMap<String , String>,
    katakana: HashMap<String , String>,
    kanji:    HashMap<String , String>,
}

impl Arguments {
    pub fn validate(&self) -> i8 {
        if (self.hira && self.kata) || 
           (self.kata && self.kanji) || 
           (self.kanji && self.hira)  {
               return -1;
           }
        if self.hira || self.kata || self.kanji{
            return 1;
        }
        0
    }
}

fn quit(message: String){
    println!("{}", message);
    std::process::exit(1);
}

fn main() {
    let args = Arguments::from_args();

    let input_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), args.dictionary);


    let f = File::open(&input_path).expect("Failed opening file");
    let japan : Kana = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    let mut main_map: HashMap<String, String> = HashMap::new();
    let arg_amount = args.validate();

    match arg_amount {
        -1 =>  quit("Error! \nPlease only use one mode (h,k,n) or use --help!".to_string()),
        0 =>  {
            // merge maps 
            // TODO - Gotta Add multimap, currently the keys are being overwritten
            main_map.extend(japan.hiragana.into_iter());
            main_map.extend(japan.katakana.into_iter());
            main_map.extend(japan.kanji.into_iter());
        }, 
        1 =>  {
            if args.hira{
                main_map = japan.hiragana.clone();
            }else if args.kata {
                main_map = japan.katakana.clone();
                // kanji
            }else if args.kanji {
                main_map = japan.kanji.clone();
            }else {
                // Should not happen
                quit("An error has occured! Invalid flag set!".to_string());
            }
        },  // select main map 
        _ =>  quit("An error has occured!".to_string()), 
    };

    let mut res = Result{ right: 0 ,wrong: 0};

    // TODO wait til Esc key is pressed for smoother exit 
    let mut reading;
    let mut solution;

    println!("{}", main_map.len());

    loop {
        let mut buffer = String::new();

        // randomly generate hira / kata / kanjji according to arguments 
        // or or -r set -> print value and check for key

        let mut it = main_map.iter();
        let _ret = match it.next() {
            Some(kv) => { 
                reading = kv.1.to_string(); 
                solution = kv.0.to_string(); 
            }
            None =>{ 
                res.print_result();
                break;
            }
        };

        println!("\nWhat is the romaji for {}", reading.blue().bold());

        main_map.remove(&solution);
    
        match io::stdin().read_line(&mut buffer) {
            Ok(_n) => {
                buffer = buffer[0..buffer.len() - 1].to_string();
                if buffer == "q" {
                    println!("");
                    res.print_result();
                    println!("Exiting ... ");
                    break; 
                }
                if buffer == solution {
                    res.right += 1; 
                    println!("Correct!"); // Sugoi! ... nah 
                }else {
                    res.wrong += 1; 
                    println!("{} was the wrong one.", buffer.red().bold()); // _nani !?_
                    println!("It should have been {}", solution.green().bold());
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
