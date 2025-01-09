CREATE TABLE brand (
    brand_name TEXT NOT NULL,
    PRIMARY KEY (brand_name)
);
CREATE TABLE tag(tag TEXT NOT NULL, PRIMARY KEY (tag));
CREATE TABLE brand_tag (
    brand_name TEXT NOT NULL,
    tag TEXT NOT NULL,
    FOREIGN KEY (brand_name) REFERENCES brand (brand_name),
    FOREIGN KEY (tag) REFERENCES tag (tag),
    PRIMARY KEY (brand_name, tag)
);
CREATE TABLE product (
    id UUID NOT NULL DEFAULT gen_random_uuid (),
    open_food_fact_id TEXT NOT NULL,
    image_url TEXT NOT NULL,
    product_type TEXT NOT NULL,
    product_quantity TEXT NOT NULL,
    product_quantity_unit TEXT NOT NULL,
    brand TEXT NOT NULL,
    FOREIGN KEY (brand) REFERENCES brand (brand_name),
    PRIMARY KEY (id)
);
CREATE TABLE keyword (
    keyword TEXT NOT NULL,
    PRIMARY KEY (keyword)
);
CREATE TABLE product_keyword (
    keyword TEXT NOT NULL,
    product_id UUID NOT NULL,
    FOREIGN KEY (keyword) REFERENCES keyword (keyword),
    FOREIGN KEY (product_id) REFERENCES product (id),
    PRIMARY KEY (keyword, product_id)
);