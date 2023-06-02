// use wasm_bindgen::prelude::JsCast;
// use web_sys::HtmlMetaElement;
//
// pub struct MetadataParams<'a> {
//     pub title: Option<&'a str>,
//     pub description: Option<&'a str>,
//     pub keywords: Option<&'a str>,
//     pub image_url: Option<&'a str>,
// }
//
// pub fn insert_metadata(params: MetadataParams) {
//     let document = web_sys::window().unwrap().document().unwrap();
//     let head = document.head().unwrap();
//     let metadata_list = head.query_selector_all("meta").unwrap();
//
//     let title = match params.title {
//         Some(title) => title.to_string(),
//         None => "Masahiro's tech note".to_string(),
//     };
//     let description = match params.description {
//         Some(description) => description.to_string(),
//         None => "名古屋のソフトウェアエンジニア。SaaSやマッチングサービス、AR/VR等の開発を経て現在は独立して名古屋で開発やITコンサルしています。サービス開発の所感や、ハマった際の解決方法を記載しております。".to_string()
//     };
//     let keywords = match params.keywords {
//         Some(keywords) => keywords.to_string(),
//         None => "ITエンジニア, ITコンサル, IT顧問, システム開発, Rust, wasm".to_string(),
//     };
//     let image = match params.image_url {
//         Some(image) => image.to_string(),
//         None => "/images/kyuri.png".to_string(),
//     };
//
//     document.set_title(&title);
//
//     for i in 0..metadata_list.length() {
//         if let Some(meta) = metadata_list.item(i) {
//             if let Ok(meta_element) = meta.dyn_into::<HtmlMetaElement>() {
//                 let meta_name = meta_element.name();
//                 if meta_name == "twitter:title" {
//                     meta_element.set_content(&title);
//                 }
//                 if meta_name == "description" {
//                     meta_element.set_content(&description);
//                 }
//                 if meta_name == "twitter:description" {
//                     meta_element.set_content(&description);
//                 }
//                 if meta_name == "keywords" {
//                     meta_element.set_content(&keywords);
//                 }
//                 if meta_name == "twitter:image" {
//                     meta_element.set_content(&image);
//                 }
//             }
//         }
//     }
// }
