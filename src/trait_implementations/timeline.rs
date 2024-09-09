use crate::{
    edupage::{Edupage, EdupageError},
    edupage_traits::Timeline,
};

impl Timeline for Edupage {
    fn filter_timeline_by_item_type(
        &self,
        item_type: crate::edupage_types::timeline::TimelineItemType,
    ) -> Result<Vec<crate::edupage_types::TimelineItem>, crate::edupage::EdupageError> {
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

    fn filter_timeline_by_item_types(
        &self,
        item_types: Vec<crate::edupage_types::timeline::TimelineItemType>,
    ) -> Result<Vec<crate::edupage_types::TimelineItem>, crate::edupage::EdupageError> {
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
