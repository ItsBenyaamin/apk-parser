pub mod parser {
    use std::io::Read;
    use std::path::PathBuf;
    use rocket::form::validate::Contains;
    use xml::EventReader;
    use xml::reader::XmlEvent;

    use crate::app::android_xml::axml;
    use crate::app::apk_info::ApkParsedInfo;


    pub async fn parse(path: &PathBuf) -> Option<ApkParsedInfo> {
        let file = std::fs::File::open(path).unwrap();
        let mut file_content: Vec<u8> = Vec::new();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        for i in 0..archive.len() {
            let mut inner_file = archive.by_index(i).unwrap();
            if inner_file.name() == "AndroidManifest.xml" {
                inner_file.read_to_end(&mut file_content).unwrap();
                break;
            }
        }

        let xml = axml::extract_xml(file_content);
        parse_to_info(xml)
    }

    fn parse_to_info(content: String) -> Option<ApkParsedInfo> {
        let mut apk_info = ApkParsedInfo::new();

        let reader = EventReader::from_str(content.as_str());
        for e in reader {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, namespace: _ }) => {
                    match name.local_name.as_str() {
                        "manifest" => {
                            for attribute in attributes {
                                let attr = attribute.name.to_string();

                                if attr.contains("versionCode") {
                                    apk_info.version_code = attribute.value;
                                } else if attr.contains("versionName") {
                                    apk_info.version_name = attribute.value;
                                } else if attr.contains("compileSdkVersionCodename") {
                                    apk_info.compile_sdk_version_code_name = attribute.value;
                                } else if attr.contains("compileSdkVersion") {
                                    apk_info.compile_sdk_version = attribute.value;
                                } else if attr.contains("package") {
                                    apk_info.package_name = attribute.value;
                                }
                            }
                        }

                        "uses-sdk" => {
                            for attribute in attributes {
                                let attr = attribute.name.to_string();

                                if attr.contains("minSdkVersion") {
                                    apk_info.min_sdk_version = attribute.value;
                                } else if attr.contains("targetSdkVersion") {
                                    apk_info.target_sdk_version = attribute.value;
                                }
                            }
                        }

                        "uses-permission" => {
                            for attribute in attributes {
                                if attribute.name.to_string().contains("name") {
                                    apk_info.permissions.push(attribute.value)
                                }
                            }
                        }

                        _ => {}
                    }
                
                }

                Err(_) => {
                    return None;
                }

                _ => {}
            }
        }

        Some(apk_info)
    }

}

