use std::collections::HashSet;

use once_cell::sync::Lazy;

// AI-Generated. May not be comprehensive. 
// TODO: find a proper list of these somewhere
pub static TRACKING_PARAMETERS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let params = vec![
        // Google Analytics / Ads
        "utm_source",
        "utm_medium",
        "utm_campaign",
        "utm_term",
        "utm_content",
        "gclid",         // Google Click Identifier
        "msclkid",       // Microsoft Click ID (Bing Ads)
        "dclid",         // DoubleClick Click ID

        // Facebook
        "fbclid",        // Facebook Click Identifier

        // Instagram
        "igshid",        // Instagram Share ID

        // YouTube
        "si",            // YouTube Share Identifier Parameter (often appears like ?si=...)
        "feature",       // Often indicates how the user got to the video (e.g., 'feature=share')

        // Amazon
        "tag",           // Amazon Affiliate Tag (e.g., tag=...)
        "ref",           // Amazon Navigation/Referral Tag (can sometimes be overly broad, but often tracked)
                         // Note: Removing all 'ref_*' prefixes might be desired but is more complex;
                         // this list only removes the exact 'ref' parameter if present.
        "_encoding",     // Often seen alongside tracking/affiliate tags
        "psc",           // Persistent Shopping Cart parameter, sometimes tied to campaigns
        "customId",
        "sr",
        "rnid",
        "customizationToken",
        "refinements",
        "s",
        "sprefix",
        "th",
        "keywords",
        "qid",
        "crid",
        "dib",
        "dib_tag",

        // Mailchimp
        "mc_cid",        // Mailchimp Campaign ID
        "mc_eid",        // Mailchimp Email ID

        // HubSpot
        "_hsenc",        // HubSpot Encoded User ID
        "_hsmi",         // HubSpot Email ID

        // Others
        "mkt_tok",       // Adobe Marketo
        // Add more common parameters here as needed
    ];
    params.into_iter().collect()
});
