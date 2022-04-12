use anyhow::{anyhow, Result};
use k8s_openapi::api::core::v1::Event;
use k8s_openapi::http;
use term_table::row::Row;
use term_table::table_cell::TableCell;

use crate::kube_config::ConnectionConfig;
use crate::wasi_outbound_http_helper_k8s::make_request;

pub fn print_kubewarden_events(
    connection_config: &ConnectionConfig,
    req_cfg_id: &str,
) -> Result<()> {
    //TODO: filter by event type
    let event_list = get_kubewarden_events(&connection_config, &req_cfg_id)?;

    if event_list.items.is_empty() {
        println!("No events found");
        return Ok(());
    }

    println!("Found {} events", event_list.items.len());

    let mut table = term_table::Table::new();
    table.max_column_width = 40;
    table.style = term_table::TableStyle::extended();

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Source", 1, term_table::table_cell::Alignment::Center),
        TableCell::new_with_alignment("Type", 1, term_table::table_cell::Alignment::Center),
        TableCell::new_with_alignment("Message", 1, term_table::table_cell::Alignment::Center),
        TableCell::new_with_alignment(
            "Involved object",
            1,
            term_table::table_cell::Alignment::Center,
        ),
        TableCell::new_with_alignment("First seen", 1, term_table::table_cell::Alignment::Center),
        TableCell::new_with_alignment("Last seen", 1, term_table::table_cell::Alignment::Center),
    ]));

    for event in event_list.items {
        let first_seen = event
            .first_timestamp
            .map(|t| t.0.to_rfc3339())
            .unwrap_or("-".to_string());
        let last_seen = event
            .last_timestamp
            .map(|t| t.0.to_rfc3339())
            .unwrap_or("-".to_string());
        let involved_object = format!("{:?}", event.involved_object);

        let source_component = event
            .source
            .map(|s| s.component.unwrap_or("-".to_string()))
            .unwrap_or("-".to_string());
        let event_type = event.type_.unwrap_or("-".to_string());

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(
                source_component,
                1,
                term_table::table_cell::Alignment::Left,
            ),
            TableCell::new_with_alignment(event_type, 1, term_table::table_cell::Alignment::Left),
            TableCell::new_with_alignment(
                event.message.unwrap_or("N/A".to_string()),
                1,
                term_table::table_cell::Alignment::Left,
            ),
            TableCell::new_with_alignment(
                involved_object,
                1,
                term_table::table_cell::Alignment::Center,
            ),
            TableCell::new_with_alignment(first_seen, 1, term_table::table_cell::Alignment::Center),
            TableCell::new_with_alignment(last_seen, 1, term_table::table_cell::Alignment::Center),
        ]));
    }
    println!("{}", table.render());
    Ok(())
}

fn get_kubewarden_events(
    connection_config: &ConnectionConfig,
    req_cfg_id: &str,
) -> Result<k8s_openapi::List<Event>> {
    let (k8s_req, response_body) = Event::list_event_for_all_namespaces(Default::default())?;

    let response = make_request(k8s_req, connection_config, req_cfg_id)?;

    // Got a status code from executing the request.
    let status_code = http::StatusCode::from_u16(response.status)?;

    // Construct the `ResponseBody<ListResponse<Event>>` using the
    // constructor returned by the API function.
    let mut response_body = response_body(status_code);

    response_body.append_slice(response.body.ok_or(anyhow!("no response body"))?.as_slice());
    let response = response_body.parse();

    let event_list = match response {
        // Successful response (HTTP 200 and parsed successfully)
        Ok(k8s_openapi::ListResponse::Ok(event_list)) => event_list,

        // Some unexpected response
        // (not HTTP 200, but still parsed successfully)
        Ok(other) => return Err(anyhow!("expected Ok but got {} {:?}", status_code, other)),

        // Some other error, like the response body being
        // malformed JSON or invalid UTF-8.
        Err(err) => return Err(anyhow!("error: {} {:?}", status_code, err)),
    };

    Ok(event_list)
}
