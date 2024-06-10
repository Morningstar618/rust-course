//-----------------------------------------------------
//            Iterating Over Options
//-----------------------------------------------------

fn main() {
    let some_product = Some("laptop");
    let mut products = vec!["cellphone", "battery", "charger"];

    // match some_product {
    //     Some(product) => products.push(product),
    //     _ => {}
    // }

    // if let Some(product) = some_product {
    //     products.push(product);
    // }

    // products.extend(some_product);
    // println!("{:?}", products);

    // let products_iter = some_product.iter().chain(products.iter());
    // println!("{:?}", products_iter);
    // for product in products_iter {
    //     println!("{}", product);
    // }

    let products = vec![Some("charger"), Some("battery"), None, Some("cellphone")];

    // let mut products_without_none: Vec<&str> = Vec::new();
    // for product in products {
    //     if product.is_some() {
    //         products_without_none.push(product.unwrap());
    //     }
    // }

    // let products_without_none = products
    //     .into_iter()
    //     .filter(|product| product.is_some())
    //     .map(|product| product.unwrap())
    //     .collect::<Vec<&str>>();

    let products_without_none = products.into_iter().flatten().collect::<Vec<&str>>();
    println!("{:?}", products_without_none);
}
