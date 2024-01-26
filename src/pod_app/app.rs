use crate::style::TableColors;
use crate::style::PALETTES;
use itertools::Itertools;
use ratatui::widgets::{ScrollbarState, TableState};
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Debug)]
pub struct Data {
    podname: String,
    description: String,
    age: String,
    containers: String,
}
use crate::style::ITEM_HEIGHT;

impl Data {
    pub(crate) const fn ref_array(&self) -> [&String; 4] {
        [
            &self.podname,
            &self.description,
            &self.age,
            &self.containers,
        ]
    }

    fn podname(&self) -> &str {
        &self.podname
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn age(&self) -> &str {
        &self.age
    }

    fn containers(&self) -> &str {
        &self.containers
    }
}

#[derive(Clone, Debug)]
pub struct App {
    pub(crate) state: TableState,
    pub(crate) items: Vec<Data>,
    pub(crate) longest_item_lens: (u16, u16, u16, u16),
    pub(crate) scroll_state: ScrollbarState,
    pub(crate) colors: TableColors,
    color_index: usize,
}

impl App {
    pub fn new() -> Self {
        let data_vec = generate_fake_podnames();
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 1,
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

fn generate_fake_podnames() -> Vec<Data> {
    use fakeit::generator;

    (0..20)
        .map(|_| {
            let podname = generator::generate("replica###-??#?#?##-??#?#?#".to_string());
            let description = "Deployment Pod".to_string();
            let age = "200d".to_string();
            let containers = "2/2".to_string();

            Data {
                podname,
                description,
                age,
                containers,
            }
        })
        .sorted_by(|a, b| a.podname.cmp(&b.podname))
        .collect_vec()
}

#[allow(clippy::cast_possible_truncation)]
fn constraint_len_calculator(items: &[Data]) -> (u16, u16, u16, u16) {
    let podname_len = items
        .iter()
        .map(Data::podname)
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
    let age_len = items
        .iter()
        .map(Data::age)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let containers_len = items
        .iter()
        .map(Data::containers)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    (
        podname_len as u16,
        description_len as u16,
        age_len as u16,
        containers_len as u16,
    )
}

#[cfg(test)]
mod tests {
    use crate::pod_app::app::constraint_len_calculator;
    use crate::pod_app::app::Data;

    #[test]
    fn test_constraint_len_calculator() {
        let test_data = vec![
            Data {
                podname: "myreplica-123456-123456".to_string(),
                description: "Deployment".to_string(),
                age: "150d".to_string(),
                containers: "2/2".to_string(),
            },
            Data {
                podname: "myreplica-923450-987654".to_string(),
                description: "Deployment".to_string(),
                age: "10d".to_string(),
                containers: "2/2".to_string(),
            },
        ];
        let (longest_podname_len, longest_description_len, longest_age_len, longest_containers_len) =
            constraint_len_calculator(&test_data);

        assert_eq!(23, longest_podname_len);
        assert_eq!(10, longest_description_len);
        assert_eq!(4, longest_age_len);
        assert_eq!(3, longest_containers_len);
    }
}
