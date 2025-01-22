impl LicenseManager {
    pub async fn refresh_license(&mut self) -> Result<(), Box<dyn Error>> {
        // Recheck Windows Store license
        let result = self.store_context.GetStoreProductForCurrentAppAsync()?.await?;
        if let Some(product) = result {
            let license = product.License()?;
            if license.IsActive()? {
                self.current_license = Some(self.get_license_info(&product).await?);
                return Ok(());
            }
        }
        Err("License not active".into())
    }

    pub fn get_user_id(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.device_id.clone())
    }

    pub async fn get_store_token(&self) -> Result<String, Box<dyn Error>> {
        let token = self.store_context.GetCustomerCollectionsIdAsync()?.await?;
        Ok(token.to_string())
    }
}
