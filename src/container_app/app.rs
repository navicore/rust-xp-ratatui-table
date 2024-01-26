use crate::style::TableColors;
use crate::style::PALETTES;
use itertools::Itertools;
use ratatui::widgets::{ScrollbarState, TableState};
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Debug)]
pub struct Data {
    container: String,
    description: String,
}
use crate::style::ITEM_HEIGHT;

impl Data {
    pub(crate) const fn ref_array(&self) -> [&String; 2] {
        [&self.container, &self.description]
    }

    fn container(&self) -> &str {
        &self.container
    }

    fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Clone, Debug)]
pub struct App {
    pub(crate) state: TableState,
    pub(crate) items: Vec<Data>,
    pub(crate) longest_item_lens: (u16, u16),
    pub(crate) scroll_state: ScrollbarState,
    pub(crate) colors: TableColors,
    color_index: usize,
}

impl App {
    pub fn new() -> Self {
        let data_vec = generate_fake_containers();
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 2,
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

fn generate_fake_containers() -> Vec<Data> {
    use fakeit::generator;

    (0..2)
        .map(|_| {
            let container = generator::generate("???????????".to_string());
            let description = "Pod Container".to_string();

            Data {
                container,
                description,
            }
        })
        .sorted_by(|a, b| a.container.cmp(&b.container))
        .collect_vec()
}

#[allow(clippy::cast_possible_truncation)]
fn constraint_len_calculator(items: &[Data]) -> (u16, u16) {
    let container_len = items
        .iter()
        .map(Data::container)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let description_len = items
        .iter()
        .map(Data::description)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    (container_len as u16, description_len as u16)
}

#[cfg(test)]
mod tests {
    use crate::container_app::app::constraint_len_calculator;
    use crate::container_app::app::Data;

    #[test]
    fn test_constraint_len_calculator() {
        let test_data = vec![
            Data {
                container: "myreplica-123456-123456".to_string(),
                description: "Deployment".to_string(),
            },
            Data {
                container: "myreplica-923450-987654".to_string(),
                description: "Deployment".to_string(),
            },
        ];
        let (longest_container_len, longest_description_len) =
            constraint_len_calculator(&test_data);

        assert_eq!(23, longest_container_len);
        assert_eq!(10, longest_description_len);
    }
}
