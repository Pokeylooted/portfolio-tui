pub mod home;
pub mod projects;
pub mod skills;
pub mod about;
pub mod content;

/// Available views
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    /// Home view
    Home,
    /// Dynamic content view with section index
    Content(usize),
}