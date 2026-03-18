use std::sync::Arc;

use clients::{HypixelClient, MojangClient, SkinProvider};
use database::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub hypixel: Arc<HypixelClient>,
    pub mojang: Arc<MojangClient>,
    pub skin_provider: Option<Arc<dyn SkinProvider>>,
    pub internal_api_key: Option<String>,
}

impl AppState {
    pub fn new(
        db: Database,
        hypixel: HypixelClient,
        mojang: MojangClient,
        skin_provider: Option<Arc<dyn SkinProvider>>,
        internal_api_key: Option<String>,
    ) -> Self {
        Self {
            db: Arc::new(db),
            hypixel: Arc::new(hypixel),
            mojang: Arc::new(mojang),
            skin_provider,
            internal_api_key,
        }
    }
}
