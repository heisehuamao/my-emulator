use crate::executor::communication::TinyConnection;
use crate::executor::Executor;

#[derive(Debug)]
pub struct Scheduler {
    name: String,
    conn: Option<TinyConnection<String>>,
}

impl Scheduler {
    pub fn new(name: String) -> Self {
        Scheduler { name, conn: None }
    }
    
    pub fn set_conn(&mut self, conn: TinyConnection<String>) {
        self.conn = Some(conn);
    }
    
    pub fn try_recv(&mut self) -> Result<String, ()> {
        match self.conn { 
            Some(ref mut conn) => {
                conn.try_recv()
            }
            None => {
                Err(())
            }
        }
    }
}

