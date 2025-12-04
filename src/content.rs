use once_cell::sync::Lazy;
use serde::Deserialize;

// Embed the YAML file at compile time
const CONTENT_YAML: &str = include_str!("../content/content.yaml");

#[derive(Deserialize, Clone, PartialEq)]
pub struct HeroContent {
    pub title: String,
    pub subtitle: String,
    pub logo: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct AboutContent {
    pub photo: String,
    pub title: String,
    pub description: String,
    pub skills: Vec<String>,
    pub work_experience_title: String,
    pub work_experience: Vec<WorkExperience>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct WorkExperience {
    pub year: String,
    pub position: String,
    pub company: String,
    pub description: Vec<String>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct PortfolioContent {
    pub title: String,
    pub projects_title: String,
    pub skills_title: String,
    pub projects: Vec<Project>,
    pub skills: Vec<Skill>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Project {
    pub name: String,
    pub description: Vec<String>,
    pub link: Option<String>,
    pub technologies: Vec<String>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Skill {
    pub name: String,
    pub level: u8,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct ContactContent {
    pub title: String,
    pub phone: String,
    pub phone_label: String,
    pub email: String,
    pub email_label: String,
    pub social_title: String,
    pub social_networks: Vec<SocialNetwork>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct SocialNetwork {
    pub name: String,
    pub url: String,
    pub icon: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct FooterContent {
    pub copyright: String,
    pub credits: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct LangContent {
    pub hero: HeroContent,
    pub about: AboutContent,
    pub portfolio: PortfolioContent,
    pub contact: ContactContent,
    pub footer: FooterContent,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Content {
    pub ru: LangContent,
    pub en: LangContent,
}

// Parse YAML once and cache it
static PARSED_CONTENT: Lazy<Content> = Lazy::new(|| {
    serde_yaml::from_str(CONTENT_YAML).expect("Failed to parse embedded content.yaml")
});

/// Get content for the specified language
pub fn get_content(lang: &str) -> &'static LangContent {
    match lang {
        "en" => &PARSED_CONTENT.en,
        _ => &PARSED_CONTENT.ru,
    }
}

/// Get hero content for the specified language
pub fn get_hero_content(lang: &str) -> &'static HeroContent {
    &get_content(lang).hero
}

/// Get about content for the specified language
pub fn get_about_content(lang: &str) -> &'static AboutContent {
    &get_content(lang).about
}

/// Get portfolio content for the specified language
pub fn get_portfolio_content(lang: &str) -> &'static PortfolioContent {
    &get_content(lang).portfolio
}

/// Get contact content for the specified language
pub fn get_contact_content(lang: &str) -> &'static ContactContent {
    &get_content(lang).contact
}

/// Get footer content for the specified language
pub fn get_footer_content(lang: &str) -> &'static FooterContent {
    &get_content(lang).footer
}
