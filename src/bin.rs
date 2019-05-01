use colored::*;
use json::*;

use json_highlight_writer::{highlight_with_colors, highlight};

pub fn main() {
  let res = object!{
    "code" => 200,
    "success" => true,
    "payload" => object!{
        "features" => array![
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
};

    println!("");
    println!("");
    println!("{:#}", highlight(&res, vec![&res["code"], &res["payload"]["features"]]));
    println!("");
    println!("");
    println!("{:#}", highlight_with_colors(&res, vec![&res["code"], &res["payload"]["features"]], vec![Color::Red, Color::Green]));
    println!("");
    println!("");
    println!("{:#}", highlight_with_colors(&res, vec![&res["payload"], &res["payload"]["features"]], vec![Color::Red, Color::Green]));
    println!("");
    println!("");
    println!("{:#}", highlight_with_colors(&res, vec![&res["code"], &res["payload"], &res["payload"]["features"]], vec![Color::Red, Color::Green]));
    println!("");
    println!("");

    let data = object!{
        "foo" => false,
        "bar" => json::Null,
        "answer" => 42,
        "list" => array![json::Null, "world", true],
        "obj" => object!{
          "foo" => false,
          "bar" => json::Null,
          "answer" => 42,
          "list" => array![json::Null, "world", true]
        },
        "2ndobj" => object!{
          "foo" => false,
          "bar" => json::Null,
          "answer" => 42,
          "list" => array![json::Null, "world", true]
        }
    };

    println!("{:#}",highlight_with_colors(&data, vec![&data["obj"], &data["2ndobj"]], vec![Color::Yellow, Color::Cyan]));
    println!("{:#}",highlight_with_colors(&data, vec![&data["obj"], &data["obj"]["list"]], vec![Color::Yellow, Color::Cyan]));
    println!("{:#}",highlight_with_colors(&data, vec![&data["2ndobj"]], vec![Color::Yellow, Color::Cyan]));
    println!("{:#}",highlight_with_colors(&data, vec![&data["list"]], vec![Color::Yellow, Color::Cyan]));
    println!("{:#}",highlight_with_colors(&data, vec![&data["bar"]], vec![Color::Yellow, Color::Cyan]));
    println!("{:#}",highlight_with_colors(&data, vec![&data["foo"]], vec![Color::Yellow, Color::Cyan]));
    println!("{:#}",highlight_with_colors(&data, vec![&data["answer"]], vec![Color::Yellow, Color::Cyan]));
    println!("{:#}",highlight_with_colors(&data, vec![&data["obj"]["list"]], vec![Color::Yellow, Color::Cyan]));

    println!("-------------");

    println!("{:#}",highlight(&data, vec![&data["obj"], &data["2ndobj"]]));
    println!("{:#}",highlight(&data, vec![&data["obj"], &data["obj"]["list"]]));
    println!("{:#}",highlight(&data, vec![&data["2ndobj"]]));
    println!("{:#}",highlight(&data, vec![&data["list"]]));
    println!("{:#}",highlight(&data, vec![&data["bar"]]));
    println!("{:#}",highlight(&data, vec![&data["foo"]]));
    println!("{:#}",highlight(&data, vec![&data["answer"]]));
    println!("{:#}",highlight(&data, vec![&data["obj"]["list"]]));
}