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

    /// Returns the 0-based index of the tab title under `(column, row)` within a bordered
    /// `ratatui::widgets::Tabs` widget's outer area (`Borders::ALL`), replicating its
    /// layout algorithm (sequential padding_left + title + padding_right + divider per
    /// tab, all on the single row directly inside the border), or `None` if the click
    /// missed the tab row, landed on a border/divider/padding, or is past the last title.
    ///
    /// Titles are assumed to be plain (unstyled, single-line) text, which is all this
    /// widget ever renders here — width is measured in `char`s, matching ratatui's byte-
    /// oblivious `Line` width calculation for ASCII content.
    pub fn tab_at_position(
        area: Rect,
        column: u16,
        row: u16,
        titles: &[String],
        padding_left_width: u16,
        padding_right_width: u16,
        divider_width: u16,
    ) -> Option<usize> {
        if area.width < 2 || area.height < 3 || row != area.y + 1 {
            return None
        }

        let inner_left = area.x + 1;
        let inner_right = area.x + area.width - 1;
        if column < inner_left || column >= inner_right {
            return None
        }

        let mut x = inner_left;
        for (i, title) in titles.iter().enumerate() {
            let last = i == titles.len() - 1;
            if x >= inner_right { break }
            x += padding_left_width.min(inner_right - x);
            if x >= inner_right { break }

            let title_start = x;
            let title_end = (x + title.chars().count() as u16).min(inner_right);
            if column >= title_start && column < title_end {
                return Some(i)
            }
            x = title_end;
            if x >= inner_right { break }

            x += padding_right_width.min(inner_right - x);
            if x >= inner_right || last { break }

            x += divider_width.min(inner_right - x);
        }

        None
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

        /// A bordered Tabs widget at (0, 0), 30 wide 3 tall — matching the top_menu.rs
        /// layout: titles ["A", "BB", "CCC"], " == " padding on both sides, "|" divider.
        /// Rendered content row (y=1): "│ == A ==|== BB ==|== CCC == |    │"
        /// (up to truncation/whichever fits within width 30).
        fn sample_tabs_area() -> Rect {
            Rect { x: 0, y: 0, width: 30, height: 3 }
        }

        fn sample_titles() -> Vec<String> {
            vec!["A".to_string(), "BB".to_string(), "CCC".to_string()]
        }

        #[test]
        fn hits_each_tab_title() {
            let area = sample_tabs_area();
            let titles = sample_titles();
            // "│ == A ==|== BB ==|== CCC == |"
            //  0123456789...
            // inner_left = 1; "A" at column 5 (1 + 4 padding)
            assert_eq!(tab_at_position(area, 5, 1, &titles, 4, 4, 1), Some(0));
            // "BB" starts after "A" (1) + right padding (4) + divider (1) + left padding (4) = 5+1+4+1+4=15
            assert_eq!(tab_at_position(area, 15, 1, &titles, 4, 4, 1), Some(1));
            assert_eq!(tab_at_position(area, 16, 1, &titles, 4, 4, 1), Some(1));
        }

        #[test]
        fn misses_padding_divider_and_borders() {
            let area = sample_tabs_area();
            let titles = sample_titles();
            assert_eq!(tab_at_position(area, 0, 1, &titles, 4, 4, 1), None, "left border");
            assert_eq!(tab_at_position(area, 1, 1, &titles, 4, 4, 1), None, "left padding of first tab");
            assert_eq!(tab_at_position(area, 5, 0, &titles, 4, 4, 1), None, "top border row");
        }

        #[test]
        fn misses_past_the_last_title() {
            let area = sample_tabs_area();
            let titles = sample_titles();
            assert_eq!(tab_at_position(area, 29, 1, &titles, 4, 4, 1), None);
        }
    }
}