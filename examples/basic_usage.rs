use gmail_stripper::GmailStripper;

fn main() {
    println!("Gmail Stripper Examples");
    println!("======================");

    // Example 1: Basic Gmail address with dots
    let email1 = "p.e.p@gmail.com";
    let normalized1 = GmailStripper::strip(email1, None);
    println!("Original: {} -> Normalized: {}", email1, normalized1);

    // Example 2: Gmail address with plus sign
    let email2 = "pep+newsletter@gmail.com";
    let normalized2 = GmailStripper::strip(email2, None);
    println!("Original: {} -> Normalized: {}", email2, normalized2);

    // Example 3: Complex Gmail address with dots and plus
    let email3 = "p.e.p+test+more@gmail.com";
    let normalized3 = GmailStripper::strip(email3, None);
    println!("Original: {} -> Normalized: {}", email3, normalized3);

    // Example 4: Non-Gmail domain (no change)
    let email4 = "p.e.p+test@example.com";
    let normalized4 = GmailStripper::strip(email4, None);
    println!("Original: {} -> Normalized: {}", email4, normalized4);

    // Example 5: Custom domain
    let email5 = "p.e.p+test@company.com";
    let normalized5 = GmailStripper::strip(email5, Some(vec!["company".to_string()]));
    println!("Original: {} -> Normalized: {} (custom domain)", email5, normalized5);

    // Example 6: Googlemail domain
    let email6 = "p.e.p+test@googlemail.com";
    let normalized6 = GmailStripper::strip(email6, None);
    println!("Original: {} -> Normalized: {}", email6, normalized6);
}