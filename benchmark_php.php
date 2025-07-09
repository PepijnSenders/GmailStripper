<?php

require_once 'src/Pep/GmailStripper.php';

use Pep\GmailStripper;

function benchmark_php_gmail_stripper($iterations = 10000) {
    $test_emails = [
        'p.ep@gmail.com',
        'pep@gmail.com',
        'p.e.p@gmail.com',
        'p.ep+teststest@gmail.com',
        'pep+teststest@gmail.com',
        'p.e.p+teststest@gmail.com',
        'p.ep+teststest+sdaasddsa@gmail.com',
        'pep+teststest+sdaasddsa@gmail.com',
        'p.e.p+teststest+sdaasddsa@gmail.com',
        'user.name+alias+more@gmail.com',
        'test.user@googlemail.com',
        'normal@otherdomain.com', // This won't be processed
    ];

    echo "Running PHP Gmail Stripper benchmark with $iterations iterations...\n";
    
    // Benchmark default domains
    $start_time = microtime(true);
    for ($i = 0; $i < $iterations; $i++) {
        foreach ($test_emails as $email) {
            GmailStripper::strip($email);
        }
    }
    $end_time = microtime(true);
    $default_time = $end_time - $start_time;
    
    // Benchmark custom domains
    $start_time = microtime(true);
    for ($i = 0; $i < $iterations; $i++) {
        foreach ($test_emails as $email) {
            GmailStripper::strip($email, ['gmail', 'googlemail', 'testdomain']);
        }
    }
    $end_time = microtime(true);
    $custom_time = $end_time - $start_time;

    // Benchmark single email processing
    $start_time = microtime(true);
    for ($i = 0; $i < $iterations; $i++) {
        GmailStripper::strip('p.e.p+teststest+sdaasddsa@gmail.com');
    }
    $end_time = microtime(true);
    $single_time = $end_time - $start_time;

    echo "Results:\n";
    echo "========\n";
    echo "Default domains: " . sprintf("%.4f", $default_time) . " seconds\n";
    echo "Custom domains:  " . sprintf("%.4f", $custom_time) . " seconds\n";
    echo "Single email:    " . sprintf("%.4f", $single_time) . " seconds\n";
    echo "Avg per operation (default): " . ($default_time / $iterations / count($test_emails) * 1000000) . " μs\n";
    echo "Avg per operation (custom):  " . ($custom_time / $iterations / count($test_emails) * 1000000) . " μs\n";
    echo "Avg per operation (single):  " . ($single_time / $iterations * 1000000) . " μs\n";

    return [
        'default_time' => $default_time,
        'custom_time' => $custom_time,
        'single_time' => $single_time,
        'iterations' => $iterations,
        'emails_count' => count($test_emails)
    ];
}

// Run the benchmark
$results = benchmark_php_gmail_stripper();

// Save results to a file for comparison
file_put_contents('php_benchmark_results.json', json_encode($results, JSON_PRETTY_PRINT));
echo "\nResults saved to php_benchmark_results.json\n";

?>