The `.rost`file is a Rust Template with the following syntax:

```
// Comment indicating parent object
{
    #title1 {
        ##subtitle1: "value1",
        common-text1: "value2"
    },

    enum1 |
        opt1,
        opt2: "desc1",
        opt3
    |,

    // rust types accepted: u64, i64, f64, str ...
    list1 [
        obj1: type,
        obj2: type,
        // admits static values
        obj-list: [ 34, "hello", 35.2 ]
    ],

    varname = func1 ( param1: type, param2: type ) -> return
}
```

This will be translated to:

```rust
struct Template {
    path: PathBuf::from("/whatever"),
    data: Vec<...>, // bytes of encoded dumped parent object
    fields: Vec<...> // bytes of encoded fields
}
```

The fields will look like:

```
titles = (title1 : 1),
subtitles = (subtitle1 = value1 : 2 ),
common-texts = (common-text1 = value2 : 3 ),
enums = (enum1 = { opt1, opt2 = desc1, opt3 } : [6-10] ),
lists = (list1 = { obj1 = type, obj2 = type, obj-list = list } : [13-18] ), (obj-list = { 34, "hello", 35.2 } : 17 )
vars = (varname = func1 )
funcs = (func1 { param1 = type, param2 = type } = return )
```
