use std::sync::Arc;
use eyre::Result;
use async_trait::async_trait;
use lapin::{
  message::{Delivery},
};
use common_data::{
  helpers::{send_write},
  repositories::event::{update_metadata_uploaded, update_image_uploaded},
};
use amqp_helpers::core::types::Handler;
use crate::{
  utils::store::Store,
  models::user_verification::UserVerification,
};
