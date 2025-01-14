use diesel::{associations::HasTable, BoolExpressionMethods, ExpressionMethods};
use router_env::{instrument, tracing};

use super::generics;
use crate::{
    dispute::{Dispute, DisputeNew, DisputeUpdate, DisputeUpdateInternal},
    errors,
    schema::dispute::dsl,
    PgPooledConn, StorageResult,
};

impl DisputeNew {
    #[instrument(skip(conn))]
    pub async fn insert(self, conn: &PgPooledConn) -> StorageResult<Dispute> {
        generics::generic_insert(conn, self).await
    }
}

impl Dispute {
    #[instrument(skip(conn))]
    pub async fn find_by_merchant_id_payment_id_connector_dispute_id(
        conn: &PgPooledConn,
        merchant_id: &str,
        payment_id: &str,
        connector_dispute_id: &str,
    ) -> StorageResult<Option<Self>> {
        generics::generic_find_one_optional::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::payment_id.eq(payment_id.to_owned()))
                .and(dsl::connector_dispute_id.eq(connector_dispute_id.to_owned())),
        )
        .await
    }

    #[instrument(skip(conn))]
    pub async fn update(self, conn: &PgPooledConn, dispute: DisputeUpdate) -> StorageResult<Self> {
        match generics::generic_update_with_unique_predicate_get_result::<
            <Self as HasTable>::Table,
            _,
            _,
            _,
        >(
            conn,
            dsl::dispute_id.eq(self.dispute_id.to_owned()),
            DisputeUpdateInternal::from(dispute),
        )
        .await
        {
            Err(error) => match error.current_context() {
                errors::DatabaseError::NoFieldsToUpdate => Ok(self),
                _ => Err(error),
            },
            result => result,
        }
    }
}
