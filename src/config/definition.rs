pub struct Config {
    pub repository: Repository,
    pub profiles: Profiles,
}

pub struct Repository {
    pub remote_url: String,
    pub local_url: String,
}

pub struct Profiles {
    pub current_profile: String,
    pub profiles: Vec<String>
}
