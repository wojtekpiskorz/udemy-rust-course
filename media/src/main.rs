mod content;

use content::catalog::Catalog;
use content::media::Media;

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("My audiobook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    let podcast = Media::Podcast(10);

    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(40);
    let itemTwo = catalog.get_by_index(1);

    println!("Item: {:#?}", item);
    println!("Item: {:#?}", itemTwo);

    if let Some(item) = catalog.get_by_index(100) {
        println!("Yeap something, {:#?}", item);
    } else {
        println!("bummer");
    }

    // match catalog.items.get(0) {
    //     Some(v) => {
    //         println!("Item: {:#?}", v);
    //     }
    //     None => {
    //         println!("Nothing here");
    //     }
    // }

    // println!("{:#?}", catalog.items.get(0));
    // println!("{:#?}", good_movie.description());
    // println!("{:#?}", bad_book.description());

    // print_media(audiobook);
    // print_media(good_movie);
    // print_media(bad_book);
}
