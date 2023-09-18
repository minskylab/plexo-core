use crate::config::{ADMIN_EMAIL, ADMIN_NAME, ADMIN_PASSWORD, ORGANIZATION_NAME};

use super::core::Engine;
use async_trait::async_trait;
use sqlx::migrate;

#[async_trait]
pub trait Prelude {
    async fn prelude(&self);
}

#[async_trait]
impl Prelude for Engine {
    async fn prelude(&self) {
        match migrate!().run(self.pool.as_ref()).await {
            Ok(_) => println!("Database migration successful"),
            Err(e) => println!("Database migration failed: {:?}\n", e),
        }

        self.set_organization_name(ORGANIZATION_NAME.to_owned())
            .await;

        let admin_email = ADMIN_EMAIL.to_owned();

        if self
            .get_member_by_email(admin_email.clone())
            .await
            .is_none()
        {
            let admin_password = ADMIN_PASSWORD.to_owned();
            let admin_name = ADMIN_NAME.to_owned();

            let admin_password_hash = self.auth.hash_password(admin_password.as_str());

            let admin_member = self
                .create_member_from_email(admin_email.clone(), admin_name, admin_password_hash)
                .await;

            if admin_member.is_none() {
                println!("Failed to create admin member");
            } else {
                println!(
                    "Admin created with email: '{}' and password: '{}'",
                    admin_email, admin_password
                );
            }
        }
    }
}
