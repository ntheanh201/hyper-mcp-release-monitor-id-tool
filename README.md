# Release Monitor ID Tool

A tool that retrieves project IDs from release-monitoring.org.

## Usage

The tool accepts a single parameter:
- `project_name`: The name of the project to search for

Example response:
- If found: Returns the project ID as a string
- If not found: Returns "No projects found"

Add the plugin to your hyper-mcp configuration:
```json
{
  "plugins": [
    {
      "name": "release-monitor-id",
      "path": "oci://ghcr.io/ntheanh201/release-monitor-id-plugin:latest",
      "runtime_config": {
        "allowed_host": "release-monitoring.org"
      }
    }
  ]
}
```