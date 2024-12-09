use charts_rs::{
    BarChart, Box, SeriesCategory, THEME_GRAFANA
};

pub fn chart() -> String {
    let mut bar_chart = BarChart::new_with_theme(
        vec![
            ("Evaporation", vec![2.0, 4.9, 7.0, 23.2, 25.6, 76.7, 135.6]).into(),
            (
                "Precipitation",
                vec![2.6, 5.9, 9.0, 26.4, 28.7, 70.7, 175.6],
            )
                .into(),
            ("Temperature", vec![2.0, 2.2, 3.3, 4.5, 6.3, 10.2, 20.3]).into(),
        ],
        vec![
            "Mon".to_string(),
            "Tue".to_string(),
            "Wed".to_string(),
            "Thu".to_string(),
            "Fri".to_string(),
            "Sat".to_string(),
            "Sun".to_string(),
        ],
        THEME_GRAFANA,
    );
    bar_chart.title_text = "Mixed Line and Bar".to_string();
    bar_chart.legend_margin = Some(Box {
        top: bar_chart.title_height,
        bottom: 5.0,
        ..Default::default()
    });
    bar_chart.series_list[2].category = Some(SeriesCategory::Line);
    bar_chart.series_list[2].y_axis_index = 1;
    bar_chart.series_list[2].label_show = true;

    bar_chart
        .y_axis_configs
        .push(bar_chart.y_axis_configs[0].clone());
    bar_chart.y_axis_configs[0].axis_formatter = Some("{c} ml".to_string());
    bar_chart.y_axis_configs[1].axis_formatter = Some("{c} °C".to_string());

    format!("{}", &bar_chart.svg().unwrap())
}