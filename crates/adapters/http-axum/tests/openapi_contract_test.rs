use utoipa::OpenApi;

fn spec() -> serde_json::Value {
    serde_json::to_value(http_axum::openapi::ApiDoc::openapi()).unwrap()
}

fn operations(spec: &serde_json::Value) -> Vec<(String, String, serde_json::Value)> {
    spec["paths"]
        .as_object()
        .unwrap()
        .iter()
        .flat_map(|(path, methods)| {
            methods
                .as_object()
                .unwrap()
                .iter()
                .map(move |(verb, op)| (verb.to_uppercase(), path.clone(), op.clone()))
        })
        .collect()
}

#[test]
fn every_operation_that_can_succeed_with_a_body_declares_its_shape() {
    let spec = spec();
    let mut untyped = Vec::new();

    for (verb, path, op) in operations(&spec) {
        let responses = op["responses"].as_object().unwrap();

        let describes_a_success = responses
            .iter()
            .any(|(code, response)| code.starts_with('2') && response.get("content").is_some());
        let says_nothing_comes_back = responses.contains_key("204");
        let serves_bytes = path.starts_with("/api/v1/media/");

        if !describes_a_success && !says_nothing_comes_back && !serves_bytes {
            untyped.push(format!("{verb} {path}"));
        }
    }

    assert!(
        untyped.is_empty(),
        "a generated client cannot type these responses: {untyped:?}"
    );
}

#[test]
fn every_operation_documents_the_refusals_a_client_must_handle() {
    let spec = spec();
    let mut silent = Vec::new();

    for (verb, path, op) in operations(&spec) {
        let responses = op["responses"].as_object().unwrap();

        if !responses.keys().any(|code| code.starts_with('4')) {
            silent.push(format!("{verb} {path}"));
        }
    }

    assert!(
        silent.is_empty(),
        "these operations can refuse a client but never say so: {silent:?}"
    );
}

#[test]
fn every_guarded_operation_says_what_a_wrong_credential_looks_like() {
    let spec = spec();
    let mut silent = Vec::new();

    for (verb, path, op) in operations(&spec) {
        if op.get("security").is_none() {
            continue;
        }

        let responses = op["responses"].as_object().unwrap();

        if !responses.contains_key("401") || !responses.contains_key("403") {
            silent.push(format!("{verb} {path}"));
        }
    }

    assert!(
        silent.is_empty(),
        "these need a credential but never describe refusing one: {silent:?}"
    );
}

#[test]
fn every_refusal_is_the_one_error_shape() {
    let spec = spec();
    let mut odd = Vec::new();

    for (verb, path, op) in operations(&spec) {
        for (code, response) in op["responses"].as_object().unwrap() {
            if !code.starts_with('4') && !code.starts_with('5') {
                continue;
            }

            let named = response["content"]["application/json"]["schema"]["$ref"]
                .as_str()
                .unwrap_or_default();

            if named != "#/components/schemas/ErrorResponse" {
                odd.push(format!("{verb} {path} -> {code}"));
            }
        }
    }

    assert!(
        odd.is_empty(),
        "a client should parse one error shape, not several: {odd:?}"
    );
}

#[test]
fn the_server_tells_a_client_what_it_allows_without_a_credential() {
    let spec = spec();
    let info = &spec["paths"]["/api/v1/server"]["get"];

    assert!(
        info.get("security").is_none(),
        "a client decides whether to offer registration before anyone has signed in"
    );
    assert_eq!(
        info["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
        "#/components/schemas/ServerInfoResponse"
    );
}
