use dashmap::DashMap;
use yggdrasil_shared::records::Url;

pub struct BibliographyStorage {
    source: Url,
    kv_store: DashMap<String, String>,
}
