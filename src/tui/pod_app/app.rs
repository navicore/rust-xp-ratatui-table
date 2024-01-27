use crate::tui::style::{TableColors, ITEM_HEIGHT, PALETTES};
use ratatui::widgets::{ScrollbarState, TableState};
use unicode_width::UnicodeWidthStr;
use crate::tui::data::{generate_pod_recs, Pod};
use crate::tui::table_ui::TuiTableState;

#[derive(Clone, Debug)]
pub struct App {
    pub(crate) state: TableState,
    pub(crate) items: Vec<Pod>,
    pub(crate) longest_item_lens: (u16, u16, u16, u16),
    pub(crate) scroll_state: ScrollbarState,
    pub(crate) colors: TableColors,
    color_index: usize,
}

impl TuiTableState for App {
    type Item = Pod;

    fn get_items(&self) -> &[Self::Item] {
        &self.items
    }

    fn get_state(&mut self) -> &mut TableState {
        &mut self.state
    }

    fn get_scroll_state(&self) -> &ScrollbarState {
        &self.scroll_state
    }

    fn set_scroll_state(&mut self, scroll_state: ScrollbarState) {
        self.scroll_state = scroll_state;
    }
    fn get_table_colors(&self) -> &TableColors {
        &self.colors
    }

    fn set_table_colors(&mut self, colors: TableColors) {
        self.colors = colors;
    }

    fn get_color_index(&self) -> usize {
        self.color_index
    }

    fn set_color_index(&mut self, color_index: usize) {
        self.color_index = color_index;
    }
}
impl App {
    pub fn new() -> Self {
        let data_vec = generate_pod_recs();
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 1,
            items: data_vec,
        }
    }


}


#[allow(clippy::cast_possible_truncation)]
fn constraint_len_calculator(items: &[Pod]) -> (u16, u16, u16, u16) {
    let name_len = items
        .iter()
        .map(Pod::podname)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let description_len = items
        .iter()
        .map(Pod::description)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let age_len = items
        .iter()
        .map(Pod::age)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let containers_len = items
        .iter()
        .map(Pod::containers)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    (
        name_len as u16,
        description_len as u16,
        age_len as u16,
        containers_len as u16,
    )
}

#[cfg(test)]
mod tests {
    use crate::tui::pod_app::app::constraint_len_calculator;
    use crate::tui::pod_app::app::Pod;

    #[test]
    fn test_constraint_len_calculator() {
        let test_data = vec![
            Pod {
                name: "replica-123456-123456".to_string(),
                description: "Deployment".to_string(),
                age: "150d".to_string(),
                containers: "2/2".to_string(),
            },
            Pod {
                name: "replica-923450-987654".to_string(),
                description: "Deployment".to_string(),
                age: "10d".to_string(),
                containers: "2/2".to_string(),
            },
        ];
        let (longest_pod_name_len, longest_description_len, longest_age_len, longest_containers_len) =
            constraint_len_calculator(&test_data);

        assert_eq!(21, longest_pod_name_len);
        assert_eq!(10, longest_description_len);
        assert_eq!(4, longest_age_len);
        assert_eq!(3, longest_containers_len);
    }
}
