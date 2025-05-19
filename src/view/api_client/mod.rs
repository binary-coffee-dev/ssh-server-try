use reqwest::Error;
use serde_json::{json, Value};
use std::collections::HashMap;

pub async fn get_posts() -> Result<Value, Error> {
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("query", json!(r#"
    query ($limit: Int!, $start: Int!, $filters: PostFiltersInput!, $sort: [String], $state: PublicationState!) {
        posts( filters: $filters pagination: {limit: $limit, start: $start} sort: $sort publicationState: $state ) {
            data {
                id attributes {
                    title
                    name
                    comments
                    likes
                    views
                    publishedAt
                    enable
                    author {
                        data {
                            id
                            attributes {
                                username
                            }
                        }
                    }
                    tags {
                        data {
                            id
                            attributes {
                                name
                            }
                        }
                    }
                }
            }
            meta {
                pagination {
                    total
                }
            }
        }
    }"#));
    map.insert(
        "variables",
        json!({
            "limit": 12,
            "start": 0,
            "state": "LIVE",
            "sort": ["publishedAt:desc"],
            "filters": {
                "enable": {"eq": true},
                "title": {"contains": ""},
            },
        }),
    );

    let res = client
        .post("https://api.binarycoffee.dev/graphql")
        .json(&map)
        .send()
        .await;

    let body_json: Value = serde_json::from_str(&res?.text().await.unwrap()).unwrap();
    // println!("Response: {:?}", body_json.to_string());

    Ok(body_json)
}

pub async fn get_post_by_name(name: &String) -> Result<Value, Error> {
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert(
        "query",
        json!(
            r#"query ($name: String!, $noUpdate: Boolean) {
                postByName(name: $name, noUpdate: $noUpdate) {
                    data {
                        id
                        attributes {
                            title
                            body
                            author {
                                data {
                                    id
                                    attributes {
                                        username
                                    }
                                }
                            }
                            banner {
                                data {
                                    attributes {
                                        url
                                    }
                                }
                            }
                            tags {
                                data {
                                    id
                                    attributes {
                                        name
                                    }
                                }
                            }
                            enable
                            name
                            views
                            readingTime
                            comments
                            likes
                            createdAt
                            updatedAt
                            publishedAt
                        }
                    }
                }
                likes: opinions(filters: {post: {name: {eq: $name}}, type: {eq: "like"}}) {
                    meta {
                        pagination {
                            total
                        }
                    }
                }
            }"#
        ),
    );
    map.insert(
        "variables",
        json!({
            "name": name,
            "noUpdate:": false,
        }),
    );

    let res = client
        .post("https://api.binarycoffee.dev/graphql")
        .json(&map)
        .send()
        .await;

    let body_json: Value = serde_json::from_str(&res?.text().await.unwrap()).unwrap();
    // println!("Response: {:?}", body_json.to_string());

    Ok(body_json)
}
