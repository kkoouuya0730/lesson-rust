pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline)
    }
}

pub struct Tweet {
    pub username: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username)
    }
}

// item: Summaryトレイトを実装している何らかの型
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 以下は同じ
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}

fn some_func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_func<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

// トレイトを実装している型を返す
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
