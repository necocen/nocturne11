use chrono::FixedOffset;
use diesel::{
    backend::Backend,
    expression::{AppearsOnTable, AsExpression, NonAggregate, SelectableExpression},
    query_builder::{AstPass, QueryFragment},
    sql_types::*,
    Connection, Expression, QueryResult, RunQueryDsl,
};
use r2d2::CustomizeConnection;

/// 各コネクションのタイムゾーンを設定するためのCustomizer
#[derive(Debug, Clone)]
pub(crate) struct TimezoneCustomizer {
    pub offset: FixedOffset,
}

impl<C: Connection> CustomizeConnection<C, diesel::r2d2::Error> for TimezoneCustomizer {
    fn on_acquire(&self, conn: &mut C) -> std::result::Result<(), diesel::r2d2::Error> {
        conn.execute(&format!(
            "SET TIME ZONE INTERVAL '{}' HOUR TO MINUTE;",
            &self.offset
        ))
        .map_err(diesel::r2d2::Error::QueryError)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub(crate) enum DatePart {
    Year,
    Month,
    Day,
}

impl<DB: Backend> QueryFragment<DB> for DatePart {
    fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        use DatePart::*;
        match self {
            Year => out.push_sql("YEAR"),
            Month => out.push_sql("MONTH"),
            Day => out.push_sql("DAY"),
        }
        Ok(())
    }
}

pub(crate) fn extract<TS: AsExpression<Timestamptz>>(
    part: DatePart,
    from: TS,
) -> Extracted<TS::Expression> {
    Extracted {
        timestamp: from.as_expression(),
        part,
    }
}
#[derive(Debug, Clone, Copy, QueryId)]
pub(crate) struct Extracted<TS> {
    timestamp: TS,
    part: DatePart,
}

impl<TS> Expression for Extracted<TS> {
    type SqlType = Double;
}

impl<DB: Backend, TS: QueryFragment<DB>> QueryFragment<DB> for Extracted<TS> {
    fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        out.push_sql("EXTRACT(");
        QueryFragment::walk_ast(&self.part, out.reborrow())?;
        out.push_sql(" FROM ");
        QueryFragment::walk_ast(&self.timestamp, out.reborrow())?;
        out.push_sql(")");
        Ok(())
    }
}
impl<TS, C: Connection> RunQueryDsl<C> for Extracted<TS> {}
impl<QS, TS: SelectableExpression<QS>> SelectableExpression<QS> for Extracted<TS> {}
impl<QS, TS: AppearsOnTable<QS>> AppearsOnTable<QS> for Extracted<TS> {}
impl<TS: NonAggregate> NonAggregate for Extracted<TS> {}
