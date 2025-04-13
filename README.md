# CosmWasm MCP Server


## Quickstart

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
      "counter": {
        "command": "PATH-TO/rust-sdk/target/release/examples/servers_std_io.exe",
        "args": []
      }
    }
  }
  ```

  MacOS/Linux

  ```json
  {
    "mcpServers": {
      "counter": {
        "command": "PATH-TO/rust-sdk/target/release/examples/servers_std_io",
        "args": []
      }
    }
  }
  ```

3. **Ensure that the MCP UI elements appear in Claude Desktop**
  The MCP UI elements will only show up in Claude for Desktop if at least one server is properly configured. It may require to restart Claude for Desktop.

4. **Once Claude Desktop is running, try chatting:**

  ```text
  counter.say_hello
  ```

  Or test other tools like:

  ```texts
  counter.increment
  counter.get_value
  counter.sum {"a": 3, "b": 4}
  ```
