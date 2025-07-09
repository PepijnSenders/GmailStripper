use gmail_stripper_rust::GmailStripper;

fn main() {
    println!("Gmail Stripper Rust Implementation");
    println!("==================================");
    
    let test_emails = vec![
        "p.ep@gmail.com",
        "pep@gmail.com",
        "p.e.p@gmail.com", 
        "p.ep+teststest@gmail.com",
        "pep+teststest@gmail.com",
        "p.e.p+teststest@gmail.com",
        "p.ep+teststest+sdaasddsa@gmail.com",
        "pep+teststest+sdaasddsa@gmail.com",
        "p.e.p+teststest+sdaasddsa@gmail.com",
    ];

    println!("\nTesting Gmail addresses:");
    for email in &test_emails {
        let stripped = GmailStripper::strip(email, None);
        println!("{} -> {}", email, stripped);
    }

    println!("\nTesting custom domain addresses:");
    let custom_test_emails = vec![
        "p.ep@testdomain.com",
        "pep@testdomain.com",
        "p.e.p@testdomain.com",
        "p.ep+teststest@testdomain.com",
    ];

    for email in &custom_test_emails {
        let stripped = GmailStripper::strip(email, Some(&["testdomain"]));
        println!("{} -> {}", email, stripped);
    }
}