use crate::{
    edupage::{Edupage, EdupageError},
    types::timeline::*,
};
/// Gets the timeline from Edupage's servers
pub trait Timeline {
    fn filter_timeline_by_item_type(
        &self,
        item_type: TimelineItemType,
    ) -> Result<Vec<TimelineItem>, EdupageError>;
    fn filter_timeline_by_item_types(
        &self,
        item_types: Vec<TimelineItemType>,
    ) -> Result<Vec<TimelineItem>, EdupageError>;
}

impl Timeline for Edupage {
    /// Get timeline events (notifications) specifying the type of events you want.
    /// 
    /// If you want multiple types of events, you can use [`Timeline::filter_timeline_by_item_types`].
    fn filter_timeline_by_item_type(
        &self,
        item_type: crate::types::timeline::TimelineItemType,
    ) -> Result<Vec<crate::types::TimelineItem>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        let mut items = Vec::new();
        for item in data.items.clone() {
            if item.item_type == item_type {
                items.push(item);
            }
        }

        Ok(items)
    }

    /// Get timeline events (notifications) specifying the types of events you want.
    fn filter_timeline_by_item_types(
        &self,
        item_types: Vec<crate::types::timeline::TimelineItemType>,
    ) -> Result<Vec<crate::types::TimelineItem>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        let mut items = Vec::new();
        for item in data.items.clone() {
            if item_types.contains(&item.item_type) {
                items.push(item);
            }
        }

        Ok(items)
    }
}
