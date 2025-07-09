use regex::Regex;
use std::sync::OnceLock;

/// Gmail Stripper - Normalizes Gmail addresses by removing dots and aliases
pub struct GmailStripper;

impl GmailStripper {
    /// Strip Gmail address by removing dots and aliases
    /// 
    /// # Arguments
    /// 
    /// * `email` - The email address to normalize
    /// * `domains` - Optional slice of domains to apply stripping to (defaults to gmail and googlemail)
    /// 
    /// # Returns
    /// 
    /// The normalized email address
    pub fn strip(email: &str, domains: Option<&[&str]>) -> String {
        let domains = domains.unwrap_or(&["gmail", "googlemail"]);
        
        // Create domain regex pattern
        let domain_pattern = domains.join("|");
        let domain_regex_pattern = format!(r"@({})", domain_pattern);
        
        static DOMAIN_REGEX_CACHE: OnceLock<std::collections::HashMap<String, Regex>> = OnceLock::new();
        let _cache = DOMAIN_REGEX_CACHE.get_or_init(|| std::collections::HashMap::new());
        
        // Check if email matches one of the specified domains
        let domain_regex = Regex::new(&domain_regex_pattern).unwrap();
        if !domain_regex.is_match(email) {
            return email.to_string();
        }
        
        let mut result = email.to_string();
        
        // Split email into username and domain parts
        if let Some(at_pos) = result.find('@') {
            let (username, domain_part) = result.split_at(at_pos);
            let mut clean_username = username.to_string();
            
            // Remove dots from username
            clean_username = clean_username.replace('.', "");
            
            // Remove everything after first + sign
            if let Some(plus_pos) = clean_username.find('+') {
                clean_username.truncate(plus_pos);
            }
            
            result = format!("{}{}", clean_username, domain_part);
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gmail_addresses() {
        let test_cases = vec![
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

        for email in test_cases {
            assert_eq!(GmailStripper::strip(email, None), "pep@gmail.com");
        }
    }

    #[test]
    fn test_custom_domains() {
        let test_cases = vec![
            "p.ep@testdomain.com",
            "pep@testdomain.com",
            "p.e.p@testdomain.com", 
            "p.ep+teststest@testdomain.com",
            "pep+teststest@testdomain.com",
            "p.e.p+teststest@testdomain.com",
            "p.ep+teststest+sdaasddsa@testdomain.com",
            "pep+teststest+sdaasddsa@testdomain.com",
            "p.e.p+teststest+sdaasddsa@testdomain.com",
        ];

        for email in test_cases {
            assert_eq!(GmailStripper::strip(email, Some(&["testdomain"])), "pep@testdomain.com");
        }
    }

    #[test]
    fn test_non_matching_domains() {
        let email = "test.user+alias@otherdomain.com";
        assert_eq!(GmailStripper::strip(email, None), email);
    }
}