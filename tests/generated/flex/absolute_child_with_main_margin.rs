#[test]
fn absolute_child_with_main_margin() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, Taffy};
    let mut taffy: Taffy<crate::TextMeasure> = Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(9f32),
                height: taffy::style::Dimension::Length(9f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(7f32),
                right: zero(),
                top: zero(),
                bottom: zero(),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(20f32),
                    height: taffy::style::Dimension::Length(37f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node, 20f32, size.width);
    assert_eq!(size.height, 37f32, "height of node {:?}. Expected {}. Actual {}", node, 37f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 9f32, "width of node {:?}. Expected {}. Actual {}", node0, 9f32, size.width);
    assert_eq!(size.height, 9f32, "height of node {:?}. Expected {}. Actual {}", node0, 9f32, size.height);
    assert_eq!(location.x, 7f32, "x of node {:?}. Expected {}. Actual {}", node0, 7f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
}