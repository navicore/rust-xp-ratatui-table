use crate::tui::style::{TableColors, ITEM_HEIGHT, PALETTES};
use ratatui::widgets::{ScrollbarState, TableState};
use unicode_width::UnicodeWidthStr;
use crate::tui::data::{generate_rs_recs, Rs};


#[derive(Clone, Debug)]
pub struct App {
    pub(crate) state: TableState,
    pub(crate) items: Vec<Rs>,
    pub(crate) longest_item_lens: (u16, u16, u16, u16, u16),
    pub(crate) scroll_state: ScrollbarState,
    pub(crate) colors: TableColors,
    color_index: usize,
}

impl App {
    pub fn new() -> Self {
        let data_vec = generate_rs_recs();
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: data_vec,
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index]);
    }
}



#[allow(clippy::cast_possible_truncation)]
fn constraint_len_calculator(items: &[Rs]) -> (u16, u16, u16, u16, u16) {
    let replicaset_len = items
        .iter()
        .map(Rs::replicaset)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let description_len = items
        .iter()
        .map(Rs::description)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let age_len = items
        .iter()
        .map(Rs::age)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let pods_len = items
        .iter()
        .map(Rs::pods)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let containers_len = items
        .iter()
        .map(Rs::containers)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    (
        replicaset_len as u16,
        description_len as u16,
        age_len as u16,
        pods_len as u16,
        containers_len as u16,
    )
}

#[cfg(test)]
mod tests {
    use crate::tui::rs_app::app::constraint_len_calculator;
    use crate::tui::rs_app::app::Rs;

    #[test]
    fn test_constraint_len_calculator() {
        let test_data = vec![
            Rs {
                name: "replica-123456".to_string(),
                description: "Deployment".to_string(),
                age: "300d".to_string(),
                pods: "10/10".to_string(),
                containers: "19/30".to_string(),
            },
            Rs {
                name: "replica-923450".to_string(),
                description: "Deployment".to_string(),
                age: "10d".to_string(),
                pods: "1/1".to_string(),
                containers: "2/2".to_string(),
            },
        ];
        let (
            longest_replicaset_len,
            longest_description_len,
            longest_age_len,
            longest_pods_len,
            longest_containers_len,
        ) = constraint_len_calculator(&test_data);

        assert_eq!(14, longest_replicaset_len);
        assert_eq!(10, longest_description_len);
        assert_eq!(4, longest_age_len);
        assert_eq!(5, longest_pods_len);
        assert_eq!(5, longest_containers_len);
    }
}
