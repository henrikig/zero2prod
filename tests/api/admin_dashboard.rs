use crate::helpers::{assert_is_redirect_response, spawn_app};

#[tokio::test]
async fn you_must_be_logged_in_to_accedd_admin_dashboard() {
    let app = spawn_app().await;

    // Arrange
    let response = app.get_admin_dashboard().await;

    // Assert
    assert_is_redirect_response(&response, "/login");
}
