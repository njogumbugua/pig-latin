# Pig Latin Translator
This is a simple Pig Latin translator written in Rust. Pig Latin is a language game predominantly used in English-speaking countries. It alters the word according to a simple set of rules.

## Installation

1. Clone the repository to your local machine:

```bash
git clone https://github.com/njogumbugua/pig-latin
```

2. Navigate to project directory:
```bash
cd pig-latin-rust
```

3. Build the project:
```bash
cargo build
```

4. Run the translator with a word or phrase as input:
```bash
cargo run -- <word or phrase>
```

Replace `<word or phrase>` with the word or phrase you want to translate into Pig Latin.

## Rules of Pig Latin

The rules of Pig Latin are simple:

- For words that begin with consonant sounds, all letters before the initial vowel are placed at the end of the word sequence. Then, "ay" is added. For example, "pig" becomes "ig-pay" and "latin" becomes "atin-lay".
- For words that begin with vowel sounds, simply add "way" to the end of the word. For example, "eat" becomes "eat-way" and "omelet" becomes "omelet-way".


## Example

Input:
```bash
cargo run first
```

Output:
```bash
irst-fay
```

## License

This project is licensed under the MIT License - see the [MIT](https://choosealicense.com/licenses/mit/) file for details.
