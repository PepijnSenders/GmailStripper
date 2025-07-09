use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gmail_stripper_rust::GmailStripper;

fn benchmark_gmail_stripper(c: &mut Criterion) {
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
        "user.name+alias+more@gmail.com",
        "test.user@googlemail.com",
        "normal@otherdomain.com", // This won't be processed
    ];

    c.bench_function("gmail_stripper_rust", |b| {
        b.iter(|| {
            for email in &test_emails {
                black_box(GmailStripper::strip(black_box(email), None));
            }
        })
    });

    c.bench_function("gmail_stripper_rust_custom_domains", |b| {
        b.iter(|| {
            for email in &test_emails {
                black_box(GmailStripper::strip(black_box(email), Some(&["gmail", "googlemail", "testdomain"])));
            }
        })
    });

    // Single email processing benchmark
    c.bench_function("gmail_stripper_rust_single", |b| {
        b.iter(|| {
            black_box(GmailStripper::strip(black_box("p.e.p+teststest+sdaasddsa@gmail.com"), None));
        })
    });
}

criterion_group!(benches, benchmark_gmail_stripper);
criterion_main!(benches);