mod content;

use content::catalog::Catalog;
use content::catalog::MightHaveAValue;
use content::media::Media;

fn print_media(media: &Media) {
    println!("{:#?}", media);
}

#[derive(Debug)]
struct Account {
    balance: i32,
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());

    println!("{}", audiobook.description2());
    println!("{}", good_movie.description2());
    println!("{}", bad_book.description2());

    print_media(&audiobook);
    print_media(&good_movie);
    print_media(&bad_book);

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog);
    println!("{:#?}", catalog.items.get(0));
    println!("{:#?}", catalog.items.get(100));

    match catalog.items.get(0) {
        Option::Some(value) => {
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("Nothing");
        }
    }

    match catalog.get_by_index(40) {
        MightHaveAValue::ThereIsAValue(value) => {
            println!("Item: {:#?}", value);
        }
        MightHaveAValue::NoValueAvailable => {
            println!("No value here!");
        }
    }

    if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index(0) {
        println!("Item in pattern match {:#?}", value)
    } else {
        println!("No Value!!!!")
    }

    let item = catalog.items.get(0);
    println!("unwrap {:#?}", item.unwrap());

    let item = catalog.items.get(40);
    let placeholder = Media::Placeholder;
    println!("unwrap_or {:#?}", item.unwrap_or(&placeholder));

    let mut accounts: Vec<Account> = vec![Account { balance: 0 }, Account { balance: 10 }];
    println!("{:#?}", accounts);
    match accounts.first_mut() {
        Some(account) => {
            account.balance = 30;
            println!("{:#?}", account);
        }
        None => {
            println!("No account found")
        }
    }
    // code below will panic
    println!("expect {:#?}", item.expect("There should be a value here"));
}
