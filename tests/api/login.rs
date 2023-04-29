use crate::helpers::{assert_is_redirect_response, spawn_app};

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    let app = spawn_app().await;

    let login = serde_json::json!({
        "username": "random-username",
        "password": "random-password",
    });
    let response = app.post_login(&login).await;

    assert_is_redirect_response(&response, "/login");

    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));

    let html_page = app.get_login_html().await;
    assert!(!html_page.contains(r#"<p><i>Authentication failed</i></p>"#));
}


#[tokio::test]
async fn redirect_to_admin_dashboard_after_login_success() {
    let app = spawn_app().await;

    // Act - part 1
    let login = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });

    let response = app.post_login(&login).await;
    assert_is_redirect_response(&response, "/admin/dashboard");

    // Act - part 2
    let html_page = app.get_admin_dashboard().await;
    assert!(html_page.contains(&format!("Welcome {}", &app.test_user.username)));
}
