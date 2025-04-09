use std::sync::Arc;
use std::sync::Mutex;

// Gelecekte routing kuralları eklenecek
#[derive(Clone)]
pub struct Router {
    // Yönteme göre route tanımları vb.
    // matchit vs. entegre edebilirsiniz.
    // route bilgilerini tutacak bir struct yeri:
    // routes: Vec<(Method, String, HandlerFn)>
    // ...
    // Şimdilik boş bırakalım.
    internal: Arc<Mutex<()>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            internal: Arc::new(Mutex::new(()))
        }
    }
}
