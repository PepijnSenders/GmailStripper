defmodule GmailStripperTest do
  use ExUnit.Case
  doctest GmailStripper

  test "strips Gmail addresses correctly" do
    emails = [
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

    for email <- emails do
      assert GmailStripper.strip(email) == "pep@gmail.com"
    end
  end

  test "strips custom domain addresses correctly" do
    emails = [
      "p.ep@testdomain.com",
      "pep@testdomain.com",
      "p.e.p@testdomain.com", 
      "p.ep+teststest@testdomain.com",
      "pep+teststest@testdomain.com",
      "p.e.p+teststest@testdomain.com",
      "p.ep+teststest+sdaasddsa@testdomain.com",
      "pep+teststest+sdaasddsa@testdomain.com",
      "p.e.p+teststest+sdaasddsa@testdomain.com"
    ]

    for email <- emails do
      assert GmailStripper.strip(email, ["testdomain"]) == "pep@testdomain.com"
    end
  end

  test "does not strip non-Gmail addresses by default" do
    email = "p.ep+test@example.com"
    assert GmailStripper.strip(email) == email
  end

  test "handles invalid email formats gracefully" do
    assert GmailStripper.strip("not-an-email") == "not-an-email"
    assert GmailStripper.strip("") == ""
  end

  test "handles googlemail domain" do
    assert GmailStripper.strip("p.ep+test@googlemail.com") == "pep@googlemail.com"
  end

  test "preserves domain with subdomains" do
    assert GmailStripper.strip("p.ep+test@gmail.co.uk", ["gmail"]) == "pep@gmail.co.uk"
  end
end