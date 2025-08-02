pub mod home;
pub mod projects;
pub mod skills;
pub mod about;

/// Available views
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    /// Home view
    Home,
    /// Projects view
    Projects,
    /// Skills view
    Skills,
    /// About view
    About,
}