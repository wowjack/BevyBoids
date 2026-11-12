use bevy::{feathers::controls::{SliderProps, slider}, prelude::*, ui_widgets::{SliderPrecision, SliderStep, ValueChange, observe, slider_self_update}};


pub fn value_slider(
    label: &str,
    min: f32,
    max: f32,
    start_val: f32,
    precision: i32,
    step: f32
) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            height: Val::Auto,
            column_gap: px(10),
            ..default()
        },
        children![
            Text(label.into()),
            (
                slider(
                    SliderProps {
                        min,
                        max,
                        value: start_val,
                    },
                    (SliderStep(step), SliderPrecision(precision))
                ),
                observe(slider_self_update),
                observe(slider_change)
            )
        ]
    )
}

fn slider_change(
    on: On<ValueChange<f32>>,
    query: Query<&ChildOf>,
    mut commands: Commands
) {
    let Ok(parent) = query.get(on.source) else { return };
    let Ok(mut e) = commands.get_entity(parent.parent()) else { return };
    e.trigger(|e| ValueChange { source: e, value: on.value });
}