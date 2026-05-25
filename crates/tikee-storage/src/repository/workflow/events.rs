use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::entities::instance_event;

use super::{InstanceEventSummary, WorkflowRepository};

impl WorkflowRepository {
    pub async fn list_instance_events(
        &self,
        instance_id: &str,
    ) -> Result<Vec<InstanceEventSummary>, sea_orm::DbErr> {
        let rows = instance_event::Entity::find()
            .filter(instance_event::Column::InstanceId.eq(instance_id.to_owned()))
            .order_by_asc(instance_event::Column::CreatedAt)
            .all(&self.db)
            .await?;
        Ok(rows.into_iter().map(InstanceEventSummary::from).collect())
    }
}
