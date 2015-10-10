#![allow(non_snake_case)]

extern crate docopt;
use docopt::Docopt;

extern crate rustc_serialize;

use std::io; //for stdin
use std::fs::File; //for the file
use std::io::BufReader; //buffered reader so we can handle large files
use std::io::Read; //to read from the above file
use std::str; //to read utf-8

const USAGE: &'static str = "
Usage: scule [options] [<file>]

Options:
    -h, --help  display this help and exit
    -u, --upper  convert to upper case 
    -l, --lower  convert to lower case 
    -v, --version  print the version
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: String,
    flag_upper: bool,
    flag_lower: bool,
    flag_help: bool,
    flag_version: bool,
}

fn main(){
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        println!("scule {}", VERSION);
        std::process::exit(0);        
    }

    let mut upperCase = false;

    if args.flag_upper {
        upperCase = true;
    }
    if args.flag_lower {
        upperCase = false;
    }
    if !args.flag_upper && !args.flag_lower {
        upperCase = false;    
    }

    let filename = args.arg_file.clone();

    if filename.len() == 0 { 
        let reader = io::stdin();
        let mut bytes: Vec<u8> = Vec::new(); 
        for byte in reader.bytes() {
            let byte = byte.unwrap();
            bytes.push(byte);
            if isPrintableASCII(byte){
                print!("{}", convertCase(bytes.clone(), upperCase));
                bytes = Vec::new(); 
                bytes = bytes;
            }
        }
        print!("{}", convertCase(bytes.clone(), upperCase));
        std::process::exit(0);
    }
    
    let file = openFile(filename.clone());
    convertCase(file, upperCase);
}

fn isPrintableASCII(char: u8) -> bool { 
    if char >= 32u8 && char <= 126u8 { 
        return true;
    }
    return false;
}

fn convertCase(bytes: Vec<u8>, isUpperCase: bool) -> String {
    let byteSlice = &bytes[..];
    let str = match str::from_utf8(byteSlice) {
        Ok(n) => n,
        Err(err) => "",
    };
    let string = String::from(str);
    if !isUpperCase {
        return string.to_lowercase();
    }
    else {
        return string.to_uppercase();
    }
}

fn openFile(filename: String) -> Vec<u8> { 
    let file = match File::open(filename) {
        Ok(file) => file, 
        Err(_) => panic!("Failed to open the file!"),
    };
    let mut bytes: Vec<u8> = Vec::new(); 
    let mut reader = BufReader::new(file);
    return match reader.read_to_end(&mut bytes) {
        Ok(x) => bytes, 
        Err(_) => panic!("Failed to read the file!"),
    };
}

#[cfg(test)]
mod tests {
    use std::process::Command; 
    use std::str; 

    use super::openFile;
    use super::convertCase;
    use super::isPrintableASCII;

    #[test]
    fn testIsPrintableASCII() {
        assert_eq!(true, isPrintableASCII(97u8));
        assert_eq!(false, isPrintableASCII(10u8));
    }


    #[test]
    fn testConvertCaseUnicode() {
        let upperCased = convertCase(String::from("Übercode Idą gęsi łąką").into_bytes(), true);
        assert_eq!(upperCased, "ÜBERCODE IDĄ GĘSI ŁĄKĄ");

        let lowerCased = convertCase(String::from("Übercode Idą gęsi łąką").into_bytes(), false);
        assert_eq!(lowerCased, "übercode idą gęsi łąką");
    }

    #[test]
    fn testConvertCaseASCII() {
        let upperCased = convertCase(String::from("The Quick Brown Fox Jumps Over The Lazy Dog").into_bytes(), true);
        assert_eq!(upperCased, "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG");

        let lowerCased = convertCase(String::from("The Quick Brown Fox Jumps Over The Lazy Dog").into_bytes(), false);
        assert_eq!(lowerCased, "the quick brown fox jumps over the lazy dog");
    }

    #[test]
    fn testOpenFile() {
        let file = openFile(String::from("./LICENSE")); //just testing whether or not we can open and read the LICENSE file
        let mut sum = 0u32;
        for byte in file {
            sum += byte as u32;
        }
        assert_eq!(sum, 1603466);
    }

}
