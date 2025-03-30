mod pdk;

use extism_pdk::*;
use pdk::types::*;
use serde_json::Map;

// Called when the tool is invoked.
pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();
    let project_name = args
        .get("project_name")
        .ok_or_else(|| Error::msg("Project name is required"))?
        .as_str()
        .ok_or_else(|| Error::msg("Project name must be a string"))?;

    let request = HttpRequest::new(&format!(
        "https://release-monitoring.org/api/v2/projects/?name={}",
        urlencoding::encode(project_name)
    ));
    
    let response = http::request::<Vec<u8>>(&request, None)
        .map_err(|e| Error::msg(format!("Failed to make HTTP request: {}", e)))?;

    let text = String::from_utf8(response.body().to_vec())
        .map_err(|e| Error::msg(format!("Failed to parse response as UTF-8: {}", e)))?;

    let projects: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| Error::msg(format!("Failed to parse JSON response: {}", e)))?;

    let items = projects
        .get("items")
        .and_then(|v| v.as_array())
        .ok_or_else(|| Error::msg("Invalid response format"))?;

    if items.is_empty() {
        return Ok(CallToolResult {
            is_error: None,
            content: vec![Content {
                annotations: None,
                text: Some("No projects found".to_string()),
                mime_type: Some("text/plain".into()),
                r#type: ContentType::Text,
                data: None,
            }],
        });
    }

    let project = &items[0];
    
    let id = project
        .get("id")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| Error::msg("Project ID not found in response"))?;

    Ok(CallToolResult {
        is_error: None,
        content: vec![Content {
            annotations: None,
            text: Some(id.to_string()),
            mime_type: Some("text/plain".into()),
            r#type: ContentType::Text,
            data: None,
        }],
    })
}

pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    let mut project_name_prop: Map<String, serde_json::Value> = Map::new();
    project_name_prop.insert("type".into(), "string".into());
    project_name_prop.insert(
        "description".into(),
        "Name of the project to search for".into(),
    );

    let mut props: Map<String, serde_json::Value> = Map::new();
    props.insert("project_name".into(), project_name_prop.into());

    let mut schema: Map<String, serde_json::Value> = Map::new();
    schema.insert("type".into(), "object".into());
    schema.insert("properties".into(), serde_json::Value::Object(props));
    schema.insert("required".into(), serde_json::Value::Array(vec!["project_name".into()]));

    Ok(ListToolsResult {
        tools: vec![ToolDescription {
            name: "release-monitor-id".into(),
            description: "Get the project ID from release-monitoring.org".into(),
            input_schema: schema,
        }],
    })
}
