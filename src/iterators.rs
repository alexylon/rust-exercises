async fn list_objects(client: &Client, bucket: &str) -> Result<(), aws_sdk_s3::Error> {
    const SUFFIX: &str = "/";
    const ROOTPATH: &str = ".vpms/modules";
    static PMS_EXTENSION: &str = ".PMS";
    static VPM_EXTENSION: &str = ".VPM";

    let resp = client.list_objects_v2().bucket(bucket).send().await.unwrap();
    let objects = resp.contents().unwrap_or_default();

    let mut pms_files: Vec<String> = Vec::new();
    let mut vpm_files: Vec<String> = Vec::new();
    objects.iter()
        .filter_map(|object| object.key())
        .for_each(|str| {
            if str.to_uppercase().ends_with(&PMS_EXTENSION) {
                let filename = str.substring(6, str.len());
                pms_files.push(s.to_string());
            }
            if str.to_uppercase().ends_with(&VPM_EXTENSION) {
                let s = str.substring(6, str.len());
                vpm_files.push(s.to_string());
            }
        });

    for object in objects {
        if let Some(str) = &object.key {
            if str.to_uppercase().ends_with(&PMS_EXTENSION)
                || str.to_uppercase().ends_with(&VPM_EXTENSION) {
                let last_index = str.rfind('/').unwrap();
                let dir_path = str.substring(6, last_index + 1);
                let file_path = str.substring(6, str.len());
                println!("Downloading: {}", file_path);
                download_object(client, bucket, str, dir_path, file_path).await.unwrap();
                // pms_files.push(s.to_string());
            }
            // if str.to_uppercase().ends_with(&VPM_EXTENSION) {
            //     let s = str.substring(6, str.len());
            //     vpm_files.push(s.to_string());
            // }
        }
    }


// let keys: Vec<&str> = objects.iter()
//     .filter_map(|o| o.key())
//     .collect();
// println!("Keys: {:#?}", keys);

// println!("PMS Files: {:#?}", &pms_files);
// println!();
// println!("VPM Files: {:#?}", &vpm_files);
    Ok(())
}