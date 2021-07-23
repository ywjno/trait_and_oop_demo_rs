pub struct Article {
    status: Option<Box<dyn Status>>,
    body: String,
}

impl Article {
    pub fn new() -> Article {
        Article {
            status: Some(Box::new(Draft {})),
            body: String::new(),
        }
    }

    pub fn write(&mut self, body: &str) {
        self.body.push_str(body)
    }

    // Draft -> Opened
    pub fn request_next(&mut self) {
        if let Some(status) = self.status.take() {
            self.status = Some(status.request_next());
        }
    }

    pub fn view(&self) -> &str {
        self.status.as_ref().unwrap().view(self)
    }

    pub fn self_view(&self) -> &str {
        self.status.as_ref().unwrap().self_view(self)
        // 或许不把实现内容放到 trait 里面而是直接实现这个方法会好些？？
        // self.body.as_str()
    }
}

trait Status {
    // Draft -> return ""
    // Opened -> return body
    fn view<'a>(&self, _: &'a Article) -> &'a str {
        ""
    }
    fn self_view<'a>(&self, article: &'a Article) -> &'a str {
        &article.body
    }
    fn request_next(self: Box<Self>) -> Box<dyn Status>;
}

struct Draft {}
impl Status for Draft {
    fn request_next(self: Box<Self>) -> Box<dyn Status> {
        Box::new(Opened {})
    }
}

struct Opened {}
impl Status for Opened {
    fn request_next(self: Box<Self>) -> Box<dyn Status> {
        self
    }

    fn view<'a>(&self, article: &'a Article) -> &'a str {
        &article.body
    }
}

fn main() {
    let mut article = Article::new();
    article.write("アトムの音");
    println!("草稿： {}", article.view());
    println!("文章的内容： {}", article.self_view());
    article.request_next();
    println!("已发布： {}", article.view());
    println!("文章的内容： {}", article.self_view());
}
