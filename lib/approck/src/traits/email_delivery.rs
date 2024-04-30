/// This trait should be implemented at the AppSystem level to allow any dependant modules to
/// be able to send email out without knowing the concerete implementation behind it.
pub trait EmailDeliveryModule {
    fn email_delivery_system(&self) -> &impl EmailDeliverySystem;
}

/// This trait should be implemented on any ________System that you wish to be able to stand in as
/// an email delivery system.
pub trait EmailDeliverySystem {
    fn send_plain_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> impl std::future::Future<Output = granite::Result<()>> + Sync + Send;
    fn queue_plain_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> impl std::future::Future<Output = granite::Result<()>> + Sync + Send;
    fn queue_plain_email_at(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        send_at: chrono::DateTime<chrono::Utc>,
    ) -> impl std::future::Future<Output = granite::Result<()>> + Sync + Send;
}
