use super::*;

pub mod log;

#[test]
fn stock_register_product_on_store() {
    let mut stock = StockImpl::new(log::new());

    stock.register(
        Product::new("Laptop air", "AbCorp", "computers/laptops"),
        proto::Store::BERLIN_DE,
        8,
    );

    let store = stock
        .stores
        .get(&proto::Store::BERLIN_DE)
        .expect("Could not recover store");

    assert_eq!(store.len(), 1);

    for (k, v) in store {
        assert_eq!(*v, 8);

        assert_eq!(k.name, "Laptop air");
        assert_eq!(k.brand, "AbCorp");
        assert_eq!(k.category, "computers/laptops");
    }
}

#[test]
fn stock_register_more_than_one_product_on_store() {
    let mut stock = StockImpl::new(log::new());

    stock.register(
        Product::new("Laptop air", "AbCorp", "computers/laptops"),
        proto::Store::BERLIN_DE,
        8,
    );
    stock.register(
        Product::new(
            "Laptop UltraPower",
            "GNUFoundation",
            "computers/desktop-computers",
        ),
        proto::Store::BERLIN_DE,
        4,
    );

    let store = stock
        .stores
        .get(&proto::Store::BERLIN_DE)
        .expect("Could not recover store");

    assert_eq!(store.len(), 2);

    let pdct = Product::new("Laptop air", "AbCorp", "computers/laptops");
    let pdct2 = Product::new(
        "Laptop UltraPower",
        "GNUFoundation",
        "computers/desktop-computers",
    );

    assert_eq!(store.get(&pdct).expect("Could not recover product"), &8);
    assert_eq!(store.get(&pdct2).expect("Could not recover product"), &4);
}

#[test]
fn stock_return_product_by_store() {
    let mut stock = StockImpl::new(log::new());

    stock.register(
        Product::new("Laptop air", "AbCorp", "computers/laptops"),
        proto::Store::BERLIN_DE,
        8,
    );
    stock.register(
        Product::new(
            "Laptop UltraPower",
            "GNUFoundation",
            "computers/desktop-computers",
        ),
        proto::Store::BERLIN_DE,
        4,
    );
    stock.register(
        Product::new(
            "SantosDumond watch Active2",
            "EvilCorp",
            "wearables/smartwatches",
        ),
        proto::Store::VENEZA_IT,
        3,
    );

    assert_eq!(stock.get(proto::Store::BERLIN_DE, false).len(), 2);
    let berlin = stock.get(proto::Store::VENEZA_IT, false);
    assert_eq!(berlin.len(), 1);

    for item in berlin {
        assert_eq!(item.name, "SantosDumond watch Active2");
        assert_eq!(item.brand, "EvilCorp");
        assert_eq!(item.category, "wearables/smartwatches");
        assert_eq!(item.quantity, 3);
        assert_eq!(
            item.availability_description,
            proto::AvailabilityDescription::AVAILABLE
        );
    }
}

#[test]
fn stock_return_product_by_store_not_show_unavailable() {
    let mut stock = StockImpl::new(log::new());

    stock.register(
        Product::new("Laptop air", "AbCorp", "computers/laptops"),
        proto::Store::BERLIN_DE,
        8,
    );
    stock.register(
        Product::new(
            "Laptop UltraPower",
            "GNUFoundation",
            "computers/desktop-computers",
        ),
        proto::Store::BERLIN_DE,
        4,
    );
    stock.register(
        Product::new(
            "SantosDumond watch Active2",
            "EvilCorp",
            "wearables/smartwatches",
        ),
        proto::Store::VENEZA_IT,
        3,
    );
    stock.register(
        Product::new(
            "SantosDumond watch Active4",
            "EvilCorp",
            "wearables/smartwatches",
        ),
        proto::Store::VENEZA_IT,
        0,
    );

    assert_eq!(stock.get(proto::Store::BERLIN_DE, false).len(), 2);
    let berlin = stock.get(proto::Store::VENEZA_IT, false);
    assert_eq!(berlin.len(), 1);

    for item in berlin {
        assert_eq!(item.name, "SantosDumond watch Active2");
        assert_eq!(item.brand, "EvilCorp");
        assert_eq!(item.category, "wearables/smartwatches");
        assert_eq!(item.quantity, 3);
        assert_eq!(
            item.availability_description,
            proto::AvailabilityDescription::AVAILABLE
        );
    }
}

#[test]
fn stock_return_product_by_store_show_unavailable() {
    let mut stock = StockImpl::new(log::new());

    stock.register(
        Product::new("Laptop air", "AbCorp", "computers/laptops"),
        proto::Store::BERLIN_DE,
        8,
    );
    stock.register(
        Product::new(
            "Laptop UltraPower",
            "GNUFoundation",
            "computers/desktop-computers",
        ),
        proto::Store::BERLIN_DE,
        0,
    );
    stock.register(
        Product::new(
            "SantosDumond watch Active4",
            "EvilCorp",
            "wearables/smartwatches",
        ),
        proto::Store::VENEZA_IT,
        0,
    );

    assert_eq!(stock.get(proto::Store::BERLIN_DE, true).len(), 2);
    let berlin = stock.get(proto::Store::VENEZA_IT, true);
    assert_eq!(berlin.len(), 1);

    for item in berlin {
        assert_eq!(item.name, "SantosDumond watch Active4");
        assert_eq!(item.brand, "EvilCorp");
        assert_eq!(item.category, "wearables/smartwatches");
        assert_eq!(item.quantity, 0);
        assert_eq!(
            item.availability_description,
            proto::AvailabilityDescription::UNAVAILABLE
        );
    }
}

#[test]
fn stock_does_not_return_unavailable_items() {
    let mut stock = StockImpl::new(log::new());

    stock.register(
        Product::new("Laptop air", "AbCorp", "computers/laptops"),
        proto::Store::BERLIN_DE,
        0,
    );
    stock.register(
        Product::new(
            "Laptop UltraPower",
            "GNUFoundation",
            "computers/desktop-computers",
        ),
        proto::Store::BERLIN_DE,
        0,
    );

    assert_eq!(stock.get(proto::Store::BERLIN_DE, false).len(), 0);
    assert_eq!(stock.get(proto::Store::VENEZA_IT, false).len(), 0);
}

#[test]
fn stock_return_empty_store() {
    let mut stock = StockImpl::new(log::new());

    stock.register(
        Product::new("Laptop air", "AbCorp", "computers/laptops"),
        proto::Store::BERLIN_DE,
        8,
    );
    stock.register(
        Product::new(
            "Laptop UltraPower",
            "GNUFoundation",
            "computers/desktop-computers",
        ),
        proto::Store::BERLIN_DE,
        4,
    );

    assert_eq!(stock.get(proto::Store::BERLIN_DE, true).len(), 2);
    assert_eq!(stock.get(proto::Store::VENEZA_IT, true).len(), 0);
}

#[test]
fn stock_return_empty_store_with_unavailable() {
    let mut stock = StockImpl::new(log::new());

    stock.register(
        Product::new("Laptop air", "AbCorp", "computers/laptops"),
        proto::Store::BERLIN_DE,
        8,
    );
    stock.register(
        Product::new(
            "Laptop UltraPower",
            "GNUFoundation",
            "computers/desktop-computers",
        ),
        proto::Store::BERLIN_DE,
        4,
    );
    stock.register(
        Product::new("Model 3", "Tesla", "cars"),
        proto::Store::VENEZA_IT,
        0,
    );

    assert_eq!(stock.get(proto::Store::BERLIN_DE, false).len(), 2);
    assert_eq!(stock.get(proto::Store::VENEZA_IT, false).len(), 0);
}
