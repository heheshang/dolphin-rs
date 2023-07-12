use rbatis::RBatis;
#[cfg(feature = "sqlserver")]
use rbdc_mssql::driver::MssqlDriver;
#[cfg(feature = "mysql")]
use rbdc_mysql::driver::MysqlDriver;
#[cfg(feature = "postgres")]
use rbdc_pg::driver::PgDriver;
#[cfg(feature = "sqlite")]
use rbdc_sqlite::driver::SqliteDriver;

#[derive(Clone, Debug)]
pub struct DbConnectionFactory {
    pub inner: RBatis,
}

impl DbConnectionFactory {
    pub async fn builder(url: &str) -> Self {
        let rb = RBatis::new();
        #[cfg(feature = "sqlite")]
        rb.link(SqliteDriver {}, url).await.unwrap();
        // mysql
        #[cfg(feature = "mysql")]
        rb.link(MysqlDriver {}, url).await.unwrap();
        // postgresql
        #[cfg(feature = "postgres")]
        rb.link(PgDriver {}, url).await.unwrap();
        // mssql/sqlserver
        #[cfg(feature = "sqlserver")]
        rb.link(MssqlDriver {}, url).await.unwrap();

        DbConnectionFactory { inner: rb }
    }

    pub async fn build(&self) -> RBatis {
        self.inner.clone()
    }
}
