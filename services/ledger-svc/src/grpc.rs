pub mod ledger_server {
    #![allow(clippy::all)]
    tonic::include_proto!("fractal.ledger");
}
use ledger_server::{ledger_server::Ledger, CreateEntryReq, EntryRes, EntryList, GetEntriesReq};
use sqlx::{PgPool, Row};
use tonic::{Request, Response, Status};

pub struct LedgerService {
    pub db: PgPool,
}

#[tonic::async_trait]
impl Ledger for LedgerService {
    async fn create_entry(
        &self,
        req: Request<CreateEntryReq>,
    ) -> Result<Response<EntryRes>, Status> {
        let e = req.into_inner();
        let uid = uuid::Uuid::parse_str(&e.user_id)
            .map_err(|_| Status::invalid_argument("bad uuid"))?;
        let mut tx = self.db.begin().await.map_err(|_| Status::internal("tx"))?;
        let row = sqlx::query!(
            "INSERT INTO ledger_entries (user_id, amount_cents, currency)
             VALUES ($1, $2, $3) RETURNING id, balance_cents",
            uid,
            e.amount_cents,
            e.currency
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| Status::internal("insert"))?;
        tx.commit().await.map_err(|_| Status::internal("commit"))?;
        Ok(Response::new(EntryRes {
            entry_id: row.id.to_string(),
            balance_cents: row.balance_cents,
        }))
    }

    async fn get_entries(
        &self,
        req: Request<GetEntriesReq>,
    ) -> Result<Response<EntryList>, Status> {
        let uid = uuid::Uuid::parse_str(&req.into_inner().user_id)
            .map_err(|_| Status::invalid_argument("bad uuid"))?;
        let rows = sqlx::query!(
            "SELECT id, balance_cents FROM ledger_entries WHERE user_id = $1 ORDER BY created_at DESC",
            uid
        )
        .fetch_all(&self.db)
        .await
        .map_err(|_| Status::internal("select"))?;
        let entries = rows
            .into_iter()
            .map(|r| EntryRes {
                entry_id: r.id.to_string(),
                balance_cents: r.balance_cents,
            })
            .collect();
        Ok(Response::new(EntryList { entries }))
    }
}
