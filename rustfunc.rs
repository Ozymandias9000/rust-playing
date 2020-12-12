use std::env;

fn first_word(s: &String) -> &str {
	let bytes = s.as_bytes();
	
	for (i, &byte) in bytes.iter().enumerate() {
		if byte == b' ' {
			return &s[..i];
		}
	}

	&s[..]
}

fn main() {
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
	for (_, str) in args.iter().enumerate() {
		first_word(str.to_string());
	}

	first_word("Around the rocks the");
}
 
