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



    // Strings in Memory

    // Rust strings are sequences of Unicode characters, but they are not stored in memory as arrays of chars. Instead, they are stored using UTF-8, a variable-width encoding. Each ASCII character in a string is stored in one byte. Other characters take up multiple bytes.
    // Ex: Showing String and &str values create by the code (see page 112 for diagram):
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "ಠ_ಠ";

    // A String has a resizable buffer holding UTF-8 text. The buffer is allocated on the heap, so it can resize its buffer as needed or requested. In the example, noodles is a String that owns an eight-byte buffer, of which seven are in use. We can think of a String as a Vec<u8> that is guaranteed to hold well-formed UTF-8. In fact, this is how String is implemented.

    // A &str (pronounced "stir" or "string slice") is a reference to a run of UTF-8 text owned by someone else, it "borrows" the text. In the example, oodles is a &str referring to the last six bytes of the text belonging to noodles, so it represents the text "oodles". Like other slice reference, a &str is a fat pointer, containing both the address of the actual data and its length. We can think of a &str as being nothing more than a &[u8] that is guaranteed to hold well-formed UTF-8.

    // A string literal is a &str that refers to preallocated text, typically stored in read-only memory along with the program's machine code. In the preceding example, poodles is a string literal, pointing to seven bytes that are created when the program begins execution, and that lasts until it exits.

    // A String or &str's.len() method returns its length. The length is measured in bytes, not characters:
    assert_eq!("ಠ_ಠ".len(), 7);
    assert_eq!("ಠ_ಠ".chars().count(), 3);

    // It's impossible to modify a &str:
    let mut s = "hello";
    s[0] = 'c'; // error: the type `str` cannot be mutably indexed
    s.push('\n'); // error: no method named `push` found for type `&str`

    // For creating new strings at runtime, use String.



    // String

    // &str is very much like &[T]: a fat pointer to some data. String is analogous to Vec<T>:
    //                                                 Vec<T>             String
    // Automatically frees buffers                     Yes                Yes
    // Growable                                        Yes                Yes
    // ::new() and ::with_capacity() static methods    Yes                Yes
    // .reserve() and .capacity() methods              Yes                Yes
    // .push() and .pop() methods                      Yes                Yes
    // Range syntax v[start..stop]                     Yes, returns &[T]  Yes, returns &str
    // Automatic conversion                            &Vec<T> to &[T]    &String to &str
    // Inherits methods                                From &[T]          From &str

    // Like a Vec, each String has its own heap-allocated buffer that isn't shared with any other String. When a String variable goes out of scope, the buffer is automatically freed, unless the String was moved.

    // There are several ways to create Strings:
    // 1. The .to_string() method converts a &str to a String. This copies the string:
    let error_message = "too many pets".to_string();
    // 2. The format!() macro works just like println!(), except that it returns a new String instead of writing text to stdout, and it doesn't automatically add a newline at the end.
    assert_eq!(format!("{}°{:02}′{:02}″N", 24, 5, 23),
        "24°05′23″N".to_string());
    // 3. Arrays, slices, and vectors of strings have two methods, .concat(), and .join(sep), that form a new String from many strings.
    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    // The choice sometimes arises of which type to use: &str or String. For now, it will suffice to point out that a &str can refer to any slice of any string, whether it is a string literal (stored in the executable) or a String (allocated and freed at run time). This means that &str is more appropriate for function arguments when the caller should be allowed to pass either kind of string.



    // Using Strings

    // Strings support the == and != operators. Two strings are equal if they contain the same characters in the same order (regardless of whether they point to the same location in memory).
    assert!("ONE".to_lowercase() == "one");

    // Strings also support the comparison operators <, <=, >, and >=, as well as many useful methods and functions that can be found in the online docs by searching for str or std::str module. Examples:
    assert!("peanut".contains("nut"));
    assert_eq!("ಠ_ಠ".replace("ಠ", "■"), "■_■");
    assert_eq!(" clean\n".trim(), "clean");

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }

    // Keep in mind that given the nature of Unicode, simple char-by-char comparison does not always give the expected answers. For example, the Rust strings "th\u{e9}" and "the\u{301}" are both valid Unicode representations for thé, the French word for tea. Unicode says they should both be displayed and processed in the same way, but Rust treats them as two completely distinct strings. More on these issues are covered in chapt 17.



    // Other String-Like Types

    // Rust guarantees that strings are valid UTF-8. Sometimes a program really needs to be able to deal with strings that are not valid Unicode. This usually happens when a Rust program has to inter-operate with some other system that doesn't enforce any such rules. For example, in most operating systems it's easy to create a filename that isn't valid Unicode. The solution?
    // Rust offers a few string-like types for these situations:
    // 1. Stick to String and &str for Unicode text.
    // 2. When working with filenames, use std::path::PathBuf and &Path instead.
    // 3. When working with binary data that isn't character data at all, use Vec<u8> and &[u8].
    // 4. When working with environment variable names and command-line arguments in the native form presented by the operating system, use OsString and &OsStr.
    // 5. When iter-operating with C libraries that use null-terminated strings, use std::ffi::CString and &CStr.

}
