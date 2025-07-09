# GmailStripper

Ever needed a check on your database to prevent all those scumbags from opting in 1000 times with fake and weird email addresses?

Now is the time to stop that! With this small Elixir library, all Gmail hacks will be history.

Gmail treats email addresses with dots in the username and plus-suffixes as aliases of the same address. For example, `pep@gmail.com`, `p.ep@gmail.com`, and `pep+anything@gmail.com` all deliver to the same inbox. This library normalizes these variations to help prevent duplicate signups.

## Installation

Add `gmail_stripper` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:gmail_stripper, "~> 0.1.2"}
  ]
end
```

## Usage

```elixir
GmailStripper.strip("p.ep@gmail.com")                    # "pep@gmail.com"
GmailStripper.strip("pep@gmail.com")                     # "pep@gmail.com"
GmailStripper.strip("p.e.p@gmail.com")                   # "pep@gmail.com"
GmailStripper.strip("p.ep+teststest@gmail.com")          # "pep@gmail.com"
GmailStripper.strip("pep+teststest@gmail.com")           # "pep@gmail.com"
GmailStripper.strip("p.e.p+teststest@gmail.com")         # "pep@gmail.com"
GmailStripper.strip("p.ep+test+more@gmail.com")          # "pep@gmail.com"
GmailStripper.strip("pep+test+more@gmail.com")           # "pep@gmail.com"
GmailStripper.strip("p.e.p+test+more@gmail.com")         # "pep@gmail.com"
```

### Custom Domains

You can also specify custom domains to apply the same normalization:

```elixir
GmailStripper.strip("p.ep+test@customdomain.com", ["customdomain"]) # "pep@customdomain.com"
```

### Default Behavior

By default, the library only normalizes addresses for Gmail (`gmail.com`) and Google Mail (`googlemail.com`) domains. Other domains are left unchanged:

```elixir
GmailStripper.strip("p.ep+test@example.com")  # "p.ep+test@example.com" (unchanged)
```

## Testing

Run the tests with:

```bash
mix test
```

## Documentation

Generate documentation with:

```bash
mix docs
```

## License

BSD-3-Clause
