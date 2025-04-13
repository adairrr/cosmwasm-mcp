# CosmWasm MCP Server
A Model Context Protocol (MCP) server that provides tools for interacting with CosmWasm smart contracts, enabling AI agents to manage and interact with blockchain-based contracts programmatically. Learn more at [Model Context Protocol](https://modelcontextprotocol.io/).

This MCP server is written in Rust and uses [cw-orchestrator](https://github.com/AbstractSDK/cw-orchestrator) to interact with CosmWasm smart contracts.

## Get started

1. **Build the Server (Counter Example)**

  ```sh
  cargo build --release
  ```

  This builds a standard input/output MCP server binary.

2. **Add or update this section in your** `PATH-TO/claude_desktop_config.json`

  Windows

  ```json
  {
    "mcpServers": {
      "cosmwasm-mcp": {
        "command": "PATH-TO/rust-sdk/target/release/examples/cw-orch-mcp.exe",
        "args": [],
        "env": {
          "TEST_MNEMONIC": "<24 word mnemonic>",
          "RUST_LOG": "info"
        }
      }
    }
  }
  ```

  MacOS/Linux (`~/Library/Application\ Support/Claude/claude_desktop_config.json`)

  ```json
  {
    "mcpServers": {
      "cosmwasm-mcp": {
        "command": "PATH-TO/rust-sdk/target/release/examples/cw-orch-mcp",
        "args": [],
        "env": {
          "TEST_MNEMONIC": "<24 word mnemonic>",
          "RUST_LOG": "info"
        }
      }
    }
  }
  ```

3. **Ensure that the MCP UI elements appear in Claude Desktop**
  The MCP UI elements will only show up in Claude for Desktop if at least one server is properly configured. It may require to restart Claude for Desktop.

4. **Once Claude Desktop is running, try chatting:**

```text
Please query `neutron14kjnjnmeyz2pzaa9ga50dnsu5dwr39q0jhnz50nwr6yyqdkg6ayqwfsur2` for `get_count`.
```

```
Please increment `neutron14kjnjnmeyz2pzaa9ga50dnsu5dwr39q0jhnz50nwr6yyqdkg6ayqwfsur2`
```

```
Please get the balance of neutron14kjnjnmeyz2pzaa9ga50dnsu5dwr39q0jhnz50nwr6yyqdkg6ayqwfsur2
```

## TODO

- [ ] Don't hardcode chain, allow to be passed in by env or into any command
- [ ] Instantiate a contract
