use crate::helpers::{spawn_app, assert_is_redirect_response};

#[tokio::test]
async fn you_must_be_logged_in_to_accedd_admin_dashboard() {
    let app = spawn_app().await;

    // Arrange
    let response = app.get_admin_dashboard().await;

    // Assert
    assert_is_redirect_response(&response, "/login");
}
