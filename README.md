# Cloudflare Dynamic DNS Updater for Self-Hosted Mail Server

A lightweight Rust utility that automatically updates your Cloudflare DNS records with your current IPv6 address. Born from the need to maintain a self-hosted Postfix mail server on a dynamic IPv6 connection.

## The Problem

My ISP provides a `/64` IPv6 address block, which is perfect for hosting services like a Postfix mail server. I configured the server to receive mail directly on port 25 to my laptop. However, every time I reconnected to my hotspot, the IPv6 address would change, breaking my MX record and making my mail server unreachable.

## The Solution

This Rust tool automatically detects IPv6 address changes and updates the corresponding Cloudflare DNS AAAA record, ensuring my mail server (`mx1.fustin.top` → `nnt@jol.fustin.top`) remains accessible despite dynamic IP changes.

## Features

- 🚀 Fast and efficient IPv6 address detection via ip.se
- ☁️ Automatic Cloudflare DNS AAAA record updates
- 🔒 Secure API token management via environment variables
- ⚡ Asynchronous operations with Tokio
- 📧 Perfect for self-hosted mail servers on dynamic IPs
- 🔄 Designed for automation on network connection

## Prerequisites

- Rust 1.70 or higher
- A Cloudflare account with API access
- A domain managed by Cloudflare
## Configuration

Create a `.env` file in the project root with the following variables:

```env
CFTOKEN=your_cloudflare_api_token
ZONE_ID=your_cloudflare_zone_id
DNS_RECORD_ID=your_dns_record_id
DOMAIN=your.domain.com
```

### Getting Your Cloudflare Credentials

1. **API Token** (`CFTOKEN`):
   - Log into Cloudflare Dashboard
   - Go to "My Profile" → "API Tokens"
   - Create token with "Edit DNS" permissions

2. **Zone ID** (`ZONE_ID`):
   - Select your domain in Cloudflare Dashboard
   - Scroll down to "API" section on the right sidebar
   - Copy the Zone ID

3. **DNS Record ID** (`DNS_RECORD_ID`):
   - Use Cloudflare API or CLI to list DNS records
   - Find the ID of the record you want to update
## One-liner using Docker
Make sure the .env file is also int the same working directory 
```bash
docker run --rm --env-file $(pwd)/.env --network=host ghcr.io/lsnnt/ipv6updater:latest
```
## Installation

1. Clone this repository:
```bash
git clone https://github.com/lsnnt/ipv6updater.git
cd ipv6updater
```

2. Build the project:
```bash
cargo build --release
```


## Usage

### One-time execution:
```bash
cargo run --release
```

### Automated updates (macOS with Shortcuts):

This is the recommended approach for macOS users who want to trigger updates when connecting to a specific WiFi network.

1. Build the release binary:
```bash
cargo build --release
```

2. Create a shell script (`run_updater.sh`):
```bash
#!/bin/bash
cd /path/to/ipv6updater
set -a  # Automatically export variables
source .env
set +a
./target/release/ipv6updater
```

3. Make it executable:
```bash
chmod +x run_updater.sh
```

4. Create a Shortcuts automation:
   - Open Shortcuts app on macOS
   - Create a new automation triggered by "WiFi" connection
   - Filter by your specific SSID (e.g., your hotspot name)
   - Add "Run Shell Script" action pointing to your script

Now the DNS record updates automatically whenever you connect to your designated WiFi network!

### Automated updates (Linux):

Add to crontab to run every 5 minutes:
```bash
*/5 * * * * cd /path/to/ipv6updater && set -a && source .env && set +a && ./target/release/ipv6updater >> /var/log/ddns.log 2>&1
```

## Dependencies

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde_json = "1.0"
dotenv = "0.15"
```

## Real-World Use Case

### Self-Hosted Mail Server Setup

**Problem**: Running a Postfix mail server on a dynamic IPv6 connection where the address changes on every reconnection.

**Solution**: This tool ensures the MX record always points to the current IPv6 address.

**Example Configuration**:
- Domain: `jol.fustin.top`
- Mail server hostname: `mx1.fustin.top`
- Email address: `nnt@jol.fustin.top`
- MX record points to: `mx1.fustin.top`
- AAAA record for `mx1.fustin.top` → Auto-updated with current IPv6

**Flow**:
1. Laptop connects to hotspot → New IPv6 address assigned
2. Shortcuts automation triggers on WiFi connection
3. Script runs → Detects new IPv6 from `curl ip.se`
4. Updates `mx1.fustin.top` AAAA record via Cloudflare API
5. Mail server remains reachable at consistent hostname

## How It Works

1. Retrieves your current IPv6 address from ip.se
2. Connects to Cloudflare API using your credentials
3. Updates the specified DNS AAAA record with your current IP
4. Adds a comment with the timestamp and IP for tracking

## Example Output

```json
{
  "success": true,
  "errors": [],
  "messages": [],
  "result": {
    "id": "...",
    "name": "your.domain.com",
    "type": "AAAA",
    "content": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
    "proxied": false,
    "ttl": 1
  }
}
```

## Security Considerations

- Never commit your `.env` file to version control
- Keep your API token secure and rotate it periodically
- Use API tokens with minimal required permissions
- Consider using systemd with `ProtectSystem=strict` on Linux

## Troubleshooting

**"Failed to get IP address"**
- Check your internet connection
- Verify ip.se is accessible from your network
- Test manually: `curl ip.se`

**"Cloudflare API error"**
- Verify your API token has DNS edit permissions
- Confirm Zone ID and DNS Record ID are correct
- Check that the domain matches the DNS record
- Test API token: Visit https://dash.cloudflare.com/profile/api-tokens

**macOS Shortcuts automation not running**
- Check Shortcuts app permissions in System Preferences → Privacy & Security
- Verify the shell script has execute permissions (`chmod +x`)
- Test the script manually first
- Check Console.app for error messages

**Mail server still unreachable after update**
- DNS propagation can take a few minutes
- Check DNS update with: `dig AAAA mx1.yourdomain.com`
- Verify Postfix is listening on the correct IPv6 address
- Check firewall rules for port 25

## Future Improvements

- [ ] Better error handling and retry logic
- [ ] Configurable IP detection sources
- [ ] Multiple DNS record updates
- [ ] Logging to file
- [ ] Change detection (only update if IP changed)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- IP detection powered by [v6 ident](https://v6.ident.me/)
- DNS updates via [Cloudflare API](https://developers.cloudflare.com/api/)
- Cloudflare DNS Records API: https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-create-dns-record

## Resources

- [Cloudflare API Tokens](https://dash.cloudflare.com/profile/api-tokens) - Create your API token
- [Create API Token Guide](https://developers.cloudflare.com/fundamentals/api/get-started/create-token/) - Official documentation
- [Cloudflare DNS API Reference](https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-update-dns-record) - Update DNS records

## Author

Created by [@lsnnt](https://github.com/lsnnt)

Repository: https://github.com/lsnnt/ipv6updater

---

**Note**: This tool is designed for IPv6 addresses. If you need IPv4 support, modify the DNS record type from "AAAA" to "A" in the code.
