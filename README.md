# Gmail Stripper (Rust)

Ever needed a check on your database to prevent all those scumbags from opting in 1000 times with fake and weird email addresses.

Now is the time to stop that, with this small library all gmail hacks will be history.

This is a Rust port of the original PHP library.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gmail-stripper = "0.1.2"
```

## Usage

```rust
use gmail_stripper::GmailStripper;

// Basic usage with default domains (gmail, googlemail)
let normalized = GmailStripper::strip("p.ep@gmail.com", None);
assert_eq!(normalized, "pep@gmail.com");

let normalized = GmailStripper::strip("pep@gmail.com", None);
assert_eq!(normalized, "pep@gmail.com");

let normalized = GmailStripper::strip("p.e.p@gmail.com", None);
assert_eq!(normalized, "pep@gmail.com");

let normalized = GmailStripper::strip("p.ep+teststest@gmail.com", None);
assert_eq!(normalized, "pep@gmail.com");

let normalized = GmailStripper::strip("pep+teststest@gmail.com", None);
assert_eq!(normalized, "pep@gmail.com");

let normalized = GmailStripper::strip("p.e.p+teststest@gmail.com", None);
assert_eq!(normalized, "pep@gmail.com");

let normalized = GmailStripper::strip("p.ep+teststest+sdaasddsa@gmail.com", None);
assert_eq!(normalized, "pep@gmail.com");

let normalized = GmailStripper::strip("pep+teststest+sdaasddsa@gmail.com", None);
assert_eq!(normalized, "pep@gmail.com");

let normalized = GmailStripper::strip("p.e.p+teststest+sdaasddsa@gmail.com", None);
assert_eq!(normalized, "pep@gmail.com");
```

Or add custom domains:

```rust
let normalized = GmailStripper::strip(
    "pep@customdomain.com", 
    Some(vec!["customdomain".to_string()])
);
assert_eq!(normalized, "pep@customdomain.com");
```

## Features

- ✅ Removes dots (.) from the local part of Gmail addresses
- ✅ Removes plus signs (+) and everything after them
- ✅ Works with custom domains
- ✅ Zero-copy when no normalization is needed
- ✅ Memory safe and fast
- ✅ Comprehensive test coverage

## License

BSD-3-Clause
