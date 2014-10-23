<?php

use Pep\GmailStripper;

class GmailStripperTest extends PHPUnit_Framework_TestCase {

  public function testEmails() {
    $emails = [
      'p.ep@gmail.com',
      'pep@gmail.com',
      'p.e.p@gmail.com',
      'p.ep+teststest@gmail.com',
      'pep+teststest@gmail.com',
      'p.e.p+teststest@gmail.com',
      'p.ep+teststest+sdaasddsa@gmail.com',
      'pep+teststest+sdaasddsa@gmail.com',
      'p.e.p+teststest+sdaasddsa@gmail.com',
    ];

    foreach ($emails as $email) {
      $this->assertEquals(GmailStripper::strip($email), 'pep@gmail.com');
    }
    $emails = [
      'p.ep@testdomain.com',
      'pep@testdomain.com',
      'p.e.p@testdomain.com',
      'p.ep+teststest@testdomain.com',
      'pep+teststest@testdomain.com',
      'p.e.p+teststest@testdomain.com',
      'p.ep+teststest+sdaasddsa@testdomain.com',
      'pep+teststest+sdaasddsa@testdomain.com',
      'p.e.p+teststest+sdaasddsa@testdomain.com',
    ];

    foreach ($emails as $email) {
      $this->assertEquals(GmailStripper::strip($email, ['testdomain']), 'pep@testdomain.com');
    }
  }

}