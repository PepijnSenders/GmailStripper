defmodule GmailStripper do
  @moduledoc """
  A library to normalize Gmail email addresses by removing dots and plus-suffixes.

  Gmail treats email addresses with dots in the username and plus-suffixes as aliases
  of the same address. For example, `pep@gmail.com`, `p.ep@gmail.com`, and 
  `pep+anything@gmail.com` all deliver to the same inbox.

  This library normalizes these variations to help prevent duplicate signups.
  """

  @default_domains ["gmail", "googlemail"]

  @doc """
  Strips dots and plus-suffixes from email addresses for specified domains.

  ## Parameters

    * `email` - The email address to normalize
    * `domains` - List of domains to apply normalization to (defaults to Gmail domains)

  ## Examples

      iex> GmailStripper.strip("p.ep@gmail.com")
      "pep@gmail.com"

      iex> GmailStripper.strip("pep+test@gmail.com")
      "pep@gmail.com"

      iex> GmailStripper.strip("p.e.p+test+more@gmail.com")
      "pep@gmail.com"

      iex> GmailStripper.strip("test@example.com")
      "test@example.com"

      iex> GmailStripper.strip("p.ep@customdomain.com", ["customdomain"])
      "pep@customdomain.com"

  """
  @spec strip(String.t(), list(String.t())) :: String.t()
  def strip(email, domains \\ @default_domains) when is_binary(email) and is_list(domains) do
    if should_strip?(email, domains) do
      email
      |> remove_dots()
      |> remove_plus_suffix()
    else
      email
    end
  end

  # Check if the email domain is in the list of domains to strip
  defp should_strip?(email, domains) do
    case String.split(email, "@") do
      [_username, domain] ->
        domain_name = domain |> String.split(".") |> hd()
        Enum.member?(domains, domain_name)
      _ ->
        false
    end
  end

  # Remove dots from the username part of the email
  defp remove_dots(email) do
    case String.split(email, "@") do
      [username, domain] ->
        cleaned_username = String.replace(username, ".", "")
        "#{cleaned_username}@#{domain}"
      _ ->
        email
    end
  end

  # Remove plus-suffix from the username part of the email
  defp remove_plus_suffix(email) do
    case String.split(email, "@") do
      [username, domain] ->
        base_username = username |> String.split("+") |> hd()
        "#{base_username}@#{domain}"
      _ ->
        email
    end
  end
end