pub mod main_menu;
pub mod introduction;
pub mod challenges;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Introduction,
    Menu,
    Chellenge_1,
    Chellenge_2,
    Chellenge_3,
    Chellenge_4,
    Chellenge_5,
    Chellenge_6,
    Chellenge_7,
}