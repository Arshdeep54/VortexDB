#[derive(Debug, Default, Clone, PartialEq)]
pub enum AppState {
    #[default]
    Dashboard,
    Database,
    VectorOperations,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModalType {
    CreateDatabase,
    DeleteDatabase,
    DatabaseList,
}
