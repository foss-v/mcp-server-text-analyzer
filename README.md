# mcp-server-text-analyzer

A PoC MCP server written in rust. Provides tools to analyze text.

## Features

### Tools

* `count_characters`
  * Counts the number of characters for a given String.

## Configuration

1. Build the binary:

```bash
cargo build --release
```

2. Update the MCP client compatible host:
```json
{
  "mcpServers": {
    "text-analyzer": {
      "command": "<path to binary>"
}
```
