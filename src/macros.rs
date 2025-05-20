macro_rules! vertical {
    ($first:expr $(, $rest:expr)*) => {
        ratatui::layout::Layout::vertical(vec![$first $(, $rest)*])
    };
}
macro_rules! horizontal {
    ($first:expr $(, $rest:expr)*) => {
        ratatui::layout::Layout::horizontal(vec![$first $(, $rest)*])
    };
}
