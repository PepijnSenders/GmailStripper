use regex::Regex;

/// A utility struct for normalizing Gmail addresses
pub struct GmailStripper;

impl GmailStripper {
    /// Strip dots and plus signs from Gmail addresses to normalize them
    /// 
    /// This function removes:
    /// - All dots (.) from the local part of the email (before @)
    /// - Plus signs (+) and everything after them in the local part
    /// 
    /// # Arguments
    /// 
    /// * `email` - The email address to normalize
    /// * `domains` - Optional list of domains to apply normalization to (defaults to gmail and googlemail)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use gmail_stripper::GmailStripper;
    /// 
    /// let normalized = GmailStripper::strip("p.ep+test@gmail.com", None);
    /// assert_eq!(normalized, "pep@gmail.com");
    /// 
    /// let normalized = GmailStripper::strip("p.ep+test@custom.com", Some(vec!["custom".to_string()]));
    /// assert_eq!(normalized, "pep@custom.com");
    /// ```
    pub fn strip(email: &str, domains: Option<Vec<String>>) -> String {
        let target_domains = domains.unwrap_or_else(|| vec!["gmail".to_string(), "googlemail".to_string()]);
        
        // Check if email contains one of the target domains
        let domain_pattern = target_domains.join("|");
        let domain_regex = Regex::new(&format!(r"@({})", domain_pattern)).unwrap();
        
        if !domain_regex.is_match(email) {
            return email.to_string();
        }
        
        // Split email into local and domain parts
        let parts: Vec<&str> = email.splitn(2, '@').collect();
        if parts.len() != 2 {
            return email.to_string();
        }
        
        let mut local_part = parts[0].to_string();
        let domain_part = parts[1];
        
        // Remove dots from local part
        local_part = local_part.replace('.', "");
        
        // Remove plus sign and everything after it
        if let Some(plus_index) = local_part.find('+') {
            local_part = local_part[..plus_index].to_string();
        }
        
        format!("{}@{}", local_part, domain_part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gmail_addresses() {
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
        ];

        for email in test_emails {
            let result = GmailStripper::strip(email, None);
            assert_eq!(result, "pep@gmail.com", "Failed for email: {}", email);
        }
    }

    #[test]
    fn test_custom_domain() {
        let test_emails = vec![
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

        for email in test_emails {
            let result = GmailStripper::strip(email, Some(vec!["testdomain".to_string()]));
            assert_eq!(result, "pep@testdomain.com", "Failed for email: {}", email);
        }
    }

    #[test]
    fn test_non_target_domain() {
        let email = "p.ep+test@otherdomain.com";
        let result = GmailStripper::strip(email, None);
        assert_eq!(result, "p.ep+test@otherdomain.com");
    }

    #[test]
    fn test_googlemail_domain() {
        let email = "p.ep+test@googlemail.com";
        let result = GmailStripper::strip(email, None);
        assert_eq!(result, "pep@googlemail.com");
    }

    #[test]
    fn test_multiple_plus_signs() {
        let email = "p.ep+test+more+stuff@gmail.com";
        let result = GmailStripper::strip(email, None);
        assert_eq!(result, "pep@gmail.com");
    }

    #[test]
    fn test_only_dots() {
        let email = "p.e.p@gmail.com";
        let result = GmailStripper::strip(email, None);
        assert_eq!(result, "pep@gmail.com");
    }

    #[test]
    fn test_only_plus() {
        let email = "pep+test@gmail.com";
        let result = GmailStripper::strip(email, None);
        assert_eq!(result, "pep@gmail.com");
    }
}