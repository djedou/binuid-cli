use binuid_shared::syn::{visit::{Visit, visit_item}, File, ItemUse, Item, 
    ItemEnum, ItemStruct, ItemFn, ItemImpl, ItemMacro, parse_file, Result
};
//use binuid_shared_wasm::vm::Component;


#[derive(Debug)]
pub struct BinuidFile {
    pub item_uses: Vec<ItemUse>,
    pub item_enums: Vec<ItemEnum>,
    pub item_structs: Vec<ItemStruct>,
    pub item_funtions: Vec<ItemFn>,
    pub item_impls: Vec<ItemImpl>,
    pub item_macros: Vec<ItemMacro>

    /*
    Const(ItemConst),
    ExternCrate(ItemExternCrate),
    ForeignMod(ItemForeignMod),
    Mod(ItemMod),
    Static(ItemStatic),
    Trait(ItemTrait),
    TraitAlias(ItemTraitAlias),
    Type(ItemType),
    Union(ItemUnion),
    Verbatim(TokenStream),
    */
}

impl BinuidFile {
    pub fn new() -> Self {
        BinuidFile {
            item_uses: vec![],
            item_enums: vec![],
            item_structs: vec![],
            item_funtions: vec![],
            item_impls: vec![],
            item_macros: vec![]
        }
    }

    pub fn load(&mut self, file: &File) {
        self.visit_file(&file);

        println!("ast: {:#?}", self);
    }
}


impl<'ast> Visit<'ast> for BinuidFile {
    fn visit_file(&mut self, file: &'ast File) {
        file.items.iter().for_each(|item| {
            match item {
                Item::Enum(i) => {
                    self.item_enums.push(i.clone());
                },
                Item::Fn(i) => {
                    self.item_funtions.push(i.clone());
                },
                Item::Impl(i) => {
                    self.item_impls.push(i.clone());
                },
                Item::Macro(i) => {
                    self.item_macros.push(i.clone())
                },
                Item::Struct(i) => {
                    self.item_structs.push(i.clone());
                },
                Item::Use(i) => {
                    self.item_uses.push(i.clone());
                },
                /*
                Const(ItemConst),
                ExternCrate(ItemExternCrate),
                ForeignMod(ItemForeignMod),
                Mod(ItemMod),
                Static(ItemStatic),
                Trait(ItemTrait),
                TraitAlias(ItemTraitAlias),
                Type(ItemType),
                Union(ItemUnion),
                Verbatim(TokenStream),
                */
                _ => {}
            }
        })
    }
}