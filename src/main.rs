fn main() {
    println!("Hello, world!");



    // String Types

    // String Literals

    // String literals are enclosed in double quotes. They use the same backslash escape sequences as char literals:
    let speech = "\"Ouch!\" said the well.\n";

    // In string literals, unlike char literals, single quotes don't need a backslash escape, and double quotes do.

    // A string may span multiple lines:
    println!("In the room the women come and go,
        Singing of Mount Abora");

    // The newline character in that string literal is included in the string, and there in the output. So are the spaces at the beginning of the second line.
    // If one line of a string ends with a backslash, then the newline character and the leading whitespace on the next line are dropped:
    println!("It was a bright, cold day in April, and \
        there were four of us--\
        more or less.");
    // This prints a single line of text.

    // In a few cases, the need to double every backslash in a string is a nuisance (regular expressions and Windows paths). For these cases, Rust offers raw strings. A raw string is tagged with the lowercase letter r. All backslashes and whitespace characters inside a raw string are included verbatim in the string. No escape sequences are recognized.
    let default_win_install_path = r"C:\Program Files\Gorillas";
    // let pattern = Regex::new(r"\d+(\.\d+)*");

    // We can't include a double-quote character in a raw string simply by putting a backslash in front of it (remember no escape sequences are recognized). However, there is a solution to this problem as well. The start and end of a raw string can be marked with pound signs:
    println!(r###"
        This raw string started with 'r###'".
        Therefore it does not end until we reach a quote mark ("")
        followed immediately by three pound signs ('###'):
    "###);

    // We can add as few or as many pound signs as needed to make it clear where the string ends.



    // Byte Strings

    // A string literal with the b prefix is a byte string. Such a string is a slice of u8 values, that is, bytes, rather than Unicode text:
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    // This combines with all the other string syntax previously learned. Byte strings can span multiple lines, use escape sequences, and use backslashes to join lines. Raw byte strings start with br".

    // Byte strings can't contain arbitrary Unicode characters. They must make do with ASCII and \xHH escape sequences.

    // The type method shown here is &[u8; 3]: it's a reference to an array of three bytes. It doesn't have any of the string methods that will be discussed shortly. The most string-like thing about it is the syntax we used to write it.

    
}
