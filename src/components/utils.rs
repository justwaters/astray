pub mod widget_utils {
    use ratatui::layout::Rect;

    /// Returns the 0-based index of the item under `(column, row)` within a bordered
    /// List's rendered area (`Borders::ALL`, one row per item), or `None` if the click
    /// missed the list, landed on its border, or is past the end of `item_count`.
    ///
    /// Callers must store the `Rect` they last passed to `render_stateful_widget` for
    /// this list and only trust it while their tab is actually on screen — a hidden
    /// component's Rect is stale until it's drawn again.
    pub fn list_item_at_position(area: Rect, column: u16, row: u16, item_count: usize) -> Option<usize> {
        if area.width == 0 || area.height < 3 {
            return None
        }
        if column < area.x || column >= area.x + area.width {
            return None
        }
        if row <= area.y || row >= area.y + area.height - 1 {
            return None
        }

        let index = (row - area.y - 1) as usize;
        (index < item_count).then_some(index)
    }

    pub fn select_next_in_list(current_item: usize, list_length: usize) -> usize {
        if list_length == 0 { return 0 }
        if current_item != list_length - 1 {
            current_item + 1
        } else {
            0
        }
    }
    
    pub fn select_prev_in_list(current_item: usize, list_length: usize) -> usize {
        if list_length == 0 { return 0 }
        if current_item != 0 {
            current_item - 1
        } else {
            list_length - 1
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// A bordered list at (10, 5) sized 20x6: border rows at y=5 and y=10, four
        /// content rows at y=6..=9.
        fn sample_area() -> Rect {
            Rect { x: 10, y: 5, width: 20, height: 6 }
        }

        #[test]
        fn hits_each_content_row_in_order() {
            let area = sample_area();
            assert_eq!(list_item_at_position(area, 15, 6, 4), Some(0));
            assert_eq!(list_item_at_position(area, 15, 7, 4), Some(1));
            assert_eq!(list_item_at_position(area, 15, 8, 4), Some(2));
            assert_eq!(list_item_at_position(area, 15, 9, 4), Some(3));
        }

        #[test]
        fn misses_the_top_and_bottom_borders() {
            let area = sample_area();
            assert_eq!(list_item_at_position(area, 15, 5, 4), None, "top border row");
            assert_eq!(list_item_at_position(area, 15, 10, 4), None, "bottom border row");
        }

        #[test]
        fn misses_outside_the_horizontal_bounds() {
            let area = sample_area();
            assert_eq!(list_item_at_position(area, 9, 7, 4), None, "one left of the area");
            assert_eq!(list_item_at_position(area, 30, 7, 4), None, "one right of the area");
        }

        #[test]
        fn misses_a_content_row_past_the_end_of_the_list() {
            let area = sample_area();
            // Row 9 is a valid content row in the widget, but the list itself only has
            // 2 items, so it should not resolve to an out-of-bounds index.
            assert_eq!(list_item_at_position(area, 15, 9, 2), None);
        }

        #[test]
        fn misses_when_the_area_is_degenerate() {
            assert_eq!(list_item_at_position(Rect { x: 0, y: 0, width: 0, height: 6 }, 0, 1, 4), None);
            assert_eq!(list_item_at_position(Rect { x: 0, y: 0, width: 20, height: 2 }, 5, 1, 4), None);
        }
    }
}