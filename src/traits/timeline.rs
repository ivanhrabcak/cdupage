use crate::{
    edupage::{Edupage, EdupageError}, types::{dbi::*, person::*, timeline::*, timetable::Timetable as EduTimetable, RingingTime},
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

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
