use std::env;

use postgrest::Postgrest;

pub fn database_connection() -> Postgrest {
    let endpoint = env::var("SUPABASE_ENDPOINT").expect("SUPABASE_ENDPOINT should be set!");
    let api_key = env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY should be set!");
    Postgrest::new(endpoint).insert_header("apikey", api_key)
}
