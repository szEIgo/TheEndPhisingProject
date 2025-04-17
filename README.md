Connect to email account using IMAP

Fetch all emails from inbox and sent folders

Parse email headers (From, To, Subject, Message-ID, Return-Path, Received)

Extract sender email address and domain

Extract display name

Extract body content (text + HTML)

Identify and extract URLs and domain names in email body

Extract DKIM, SPF, and DMARC results (if available)

Normalize sender data (lowercase, remove formatting)

Group similar emails by sender/domain/template similarity

Build a whitelist of known legitimate senders and domains

Build a fingerprint/signature from legitimate emails (text structure, link patterns, wording, layout)

Build a blacklist of known malicious domains/senders

Compare new emails to whitelist/blacklist using rule-based filtering

Use LLM to semantically compare unknown emails to known legitimate ones

Flag emails with low similarity and/or suspicious patterns for manual review

Train a model on labeled examples (legit vs phishing) using features (domain reputation, wording, structure, link behavior, etc.)

Continuously update whitelist/blacklist and retrain the model

Archive or delete flagged phishing/spam emails

Log decisions and feedback from manual reviews to improve future detection

Schedule regular email fetch + analysis cycle (e.g., cron job)

Add interface/dashboard for manual review and classification

Add automated alert system for new potential phishing attempts# TheEndPhisingProject
