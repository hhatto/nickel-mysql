use std::default::Default;
use std::sync::Arc;

use mysql::conn::OptsBuilder;
use mysql::conn::pool::Pool;
use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use typemap::Key;
use plugin::Extensible;

pub struct MysqlMiddleware {
    pub pool: Arc<Pool>,
}

impl MysqlMiddleware {
    pub fn new(db_name: &str, user: &str, pass: &str) -> MysqlMiddleware {
        let mut options = OptsBuilder::default();
        options.user(Some(user))
            .pass(Some(pass))
            .db_name(Some(db_name));
        let pool = Pool::new(options).unwrap();
        MysqlMiddleware { pool: Arc::new(pool) }
    }
}

impl Key for MysqlMiddleware {
    type Value = Arc<Pool>;
}

impl<D> Middleware<D> for MysqlMiddleware {
    fn invoke<'mw, 'conn>(&self,
                          request: &mut Request<'mw, 'conn, D>,
                          response: Response<'mw, D>)
                          -> MiddlewareResult<'mw, D> {
        request.extensions_mut().insert::<MysqlMiddleware>(self.pool.clone());
        Ok(Continue(response))
    }
}

pub trait MysqlRequestExtensions {
    fn db_connection(&self) -> Arc<Pool>;
}

impl<'a, 'b, D> MysqlRequestExtensions for Request<'a, 'b, D> {
    fn db_connection(&self) -> Arc<Pool> {
        self.extensions().get::<MysqlMiddleware>().unwrap().clone()
    }
}
