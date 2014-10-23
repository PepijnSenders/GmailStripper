# GmailStripper

Ever needed a check on your database to prevent all those scumbags from opting in 1000 times with fake and weird email addresses.

Now is the time to stop that, with this small plugin all gmail hacks will be history.

```
GmailStripper::strip('p.ep@gmail.com'); // pep@gmail.com
GmailStripper::strip('pep@gmail.com'); // pep@gmail.com
GmailStripper::strip('p.e.p@gmail.com'); // pep@gmail.com
GmailStripper::strip('p.ep+teststest@gmail.com'); // pep@gmail.com
GmailStripper::strip('pep+teststest@gmail.com'); // pep@gmail.com
GmailStripper::strip('p.e.p+teststest@gmail.com'); // pep@gmail.com
GmailStripper::strip('p.ep+teststest+sdaasddsa@gmail.com'); // pep@gmail.com
GmailStripper::strip('pep+teststest+sdaasddsa@gmail.com'); // pep@gmail.com
GmailStripper::strip('p.e.p+teststest+sdaasddsa@gmail.com'); // pep@gmail.com
```