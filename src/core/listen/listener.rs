use crate::bindings::*;
use std::ptr::null_mut;

pub type TopicListener = DDS_TopicListener;
pub type PublisherListener = DDS_PublisherListener;
pub type SubscriberListener = DDS_SubscriberListener;
pub type DataWriterListener = DDS_DataWriterListener;
pub type DataReaderListener = DDS_DataReaderListener;

impl Default for TopicListener {
    fn default() -> Self {
        Self {
            listener: DDS_Listener {
                user_data: null_mut(),
            },
            on_inconsistent_topic: None,
        }
    }
}

impl Default for PublisherListener {
    fn default() -> Self {
        Self {
            datawriter_listener: DataWriterListener::default(),
        }
    }
}

impl Default for SubscriberListener {
    fn default() -> Self {
        Self {
            datareader_listener: DataReaderListener::default(),
            on_data_on_readers: None,
        }
    }
}

impl Default for DataWriterListener {
    fn default() -> Self {
        Self {
            listener: DDS_Listener {
                user_data: null_mut(),
            },
            on_offered_deadline_missed: None,
            on_offered_incompatible_qos: None,
            on_liveliness_lost: None,
            on_publication_matched: None,
        }
    }
}

impl Default for DataReaderListener {
    fn default() -> Self {
        Self {
            listener: DDS_Listener {
                user_data: null_mut(),
            },
            on_requested_deadline_missed: None,
            on_requested_incompatible_qos: None,
            on_sample_rejected: None,
            on_sample_lost: None,
            on_liveliness_changed: None,
            on_data_available: None,
            on_data_arrived: None,
            on_subscription_matched: None,
        }
    }
}
