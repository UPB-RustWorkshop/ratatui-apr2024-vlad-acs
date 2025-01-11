use std::ptr::null;

use ratatui::{layout::{Constraint, Direction, Layout}, widgets::{List, ListItem, Widget}, Frame};
use reqwest::get;
use crate::{app::App, connection::{get_all_cities, get_city_info, CityInfo}};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    
    // TODO: Split the layout
    // let [area1, area2, area3 ...] = 
    let layout = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    // TODO: get the list of cities
    // let cities: Vec<ListItem> =
    // let list_component = 
    let cities: Vec<CityInfo> = get_all_cities().iter().filter_map(|x| get_city_info(x)).collect();
    let list_component = List::new(cities.iter().map(|x| x.name.clone()).collect::<Vec<String>>());

    // TODO: render the list of cities
    // frame.render_widget(list_component, area);
    frame.render_widget(list_component, layout[0]);

    // TODO: Create the weather info component
    // let weather_info = 
    let weather_info = List::new(cities.iter().map(|x| x.weather.clone()).collect::<Vec<String>>());

    // TODO: Render the weather info component
    // frame.render_widget(weather_info, area);
    frame.render_widget(weather_info, layout[1]);

}
