# Demo script for GmailStripper
# Run with: elixir demo.exs

defmodule Demo do
  def run do
    IO.puts("GmailStripper Demo")
    IO.puts("================")
    IO.puts("")

    # Test cases from the original PHP version
    test_emails = [
      "p.ep@gmail.com",
      "pep@gmail.com", 
      "p.e.p@gmail.com",
      "p.ep+teststest@gmail.com",
      "pep+teststest@gmail.com",
      "p.e.p+teststest@gmail.com",
      "p.ep+teststest+sdaasddsa@gmail.com",
      "pep+teststest+sdaasddsa@gmail.com",
      "p.e.p+teststest+sdaasddsa@gmail.com"
    ]

    IO.puts("Testing Gmail addresses:")
    for email <- test_emails do
      stripped = GmailStripper.strip(email)
      IO.puts("  #{email} -> #{stripped}")
    end

    IO.puts("")
    IO.puts("Testing non-Gmail addresses (should remain unchanged):")
    non_gmail_emails = [
      "p.ep+test@example.com",
      "user.name+tag@company.org",
      "test@domain.co.uk"
    ]

    for email <- non_gmail_emails do
      stripped = GmailStripper.strip(email)
      IO.puts("  #{email} -> #{stripped}")
    end

    IO.puts("")
    IO.puts("Testing custom domains:")
    custom_domain_emails = [
      "p.ep+test@customdomain.com",
      "user.name+tag@customdomain.com"
    ]

    for email <- custom_domain_emails do
      stripped = GmailStripper.strip(email, ["customdomain"])
      IO.puts("  #{email} -> #{stripped} (custom domain)")
    end

    IO.puts("")
    IO.puts("Testing googlemail domain:")
    googlemail_email = "p.ep+test@googlemail.com"
    stripped = GmailStripper.strip(googlemail_email)
    IO.puts("  #{googlemail_email} -> #{stripped}")
  end
end

Demo.run()