/// general service item (database, syncthing, etc...)
pub struct ServiceItem {
    pub title: String,
    pub description: Option<String>,
    pub sub_services: Vec<SubService>,
}

impl ServiceItem {
    pub fn new(title: impl ToString) -> Self {
        Self {
            title: title.to_string(),
            description: None,
            sub_services: vec![],
        }
    }

    pub fn description(mut self, desc: Option<impl ToString>) -> Self {
        self.description = desc.map(|e| e.to_string());
        self
    }

    pub fn service(mut self, sub_service: SubService) -> Self {
        self.sub_services.push(sub_service);
        self
    }
}

/// child service/option that belongs to a service
pub struct SubService {
    pub title: String,
    pub href: String,
}

impl SubService {
    pub fn new(title: &str, href: &str) -> Self {
        Self {
            title: title.into(),
            href: href.into(),
        }
    }
}
