<?php

use Pep\GmailStripper;

class GmailStripperTest extends PHPUnit_Framework_TestCase {

  public function testEmails() {
    $emails = [
      'p.senders92@gmail.com',
      'psenders92@gmail.com',
      'p.s.e.n.d.e.r.s.9.2@gmail.com',
      'p.senders92+teststest@gmail.com',
      'psenders92+teststest@gmail.com',
      'p.s.e.n.d.e.r.s.9.2+teststest@gmail.com',
      'p.senders92+teststest+sdaasddsa@gmail.com',
      'psenders92+teststest+sdaasddsa@gmail.com',
      'p.s.e.n.d.e.r.s.9.2+teststest+sdaasddsa@gmail.com',
    ];

    foreach ($emails as $email) {
      $this->assertEquals(GmailStripper::strip($email), 'psenders92@gmail.com');
    }
  }

}