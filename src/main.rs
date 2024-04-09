use dotenv::dotenv;
use s3::{creds::Credentials, Bucket, BucketConfiguration, Region};
use std::{env, error::Error, sync::RwLockReadGuard};

#[allow(unused)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    //  1. create s3 bucket client instance
    let bucket_name = "rusty-bucket";
    let region = Region::Custom {
        region: "".to_owned(),
        endpoint: "http://127.0.0.1:9000".to_owned(),
    };
    let credentials = Credentials {
        access_key: Some(env::var("MINIO_ACCESS_KEY")?.to_owned()),
        secret_key: Some(env::var("MINIO_SECRET_KEY")?.to_owned()),
        security_token: None,
        session_token: None,
        expiration: None,
    };
    let mut bucket = Bucket::new(bucket_name, region, credentials)?.with_path_style();

    //  2. create bucket if doesn't exist in minio db yet (on first run)
    if let Err(e) = bucket.head_object("/").await {
        // handle error - error indicates the bucket does not exist yet
        println!("An error occurred: {:?}", e);
        // obtain read access
        let creds_read_guard: RwLockReadGuard<Credentials> = bucket.credentials.read().unwrap();
        // to use `creds_read_guard` as `Credentials`, you can dereference it
        let actual_credentials: &Credentials = &*creds_read_guard;
        // create bucket in s3
        let create_result = Bucket::create_with_path_style(
            bucket_name,
            bucket.region.clone(),
            actual_credentials.clone(),
            BucketConfiguration::default(),
        )
        .await?;

        println!(
            "Bucket created\n{} - {} - {}",
            bucket_name, create_result.response_code, create_result.response_text
        );
    }

    //  3. Create an object and put it in bucket (text/plain)
    let key = "test_file";
    let data = "Rusty Bucket Testing";
    println!("###### Putting content in s3 bucket");
    let resp = bucket
        .put_object_with_content_type(key, data.as_bytes(), "text/plain")
        .await?;
    println!("response: {}", resp);

    //  4. List existing bucket content
    println!("###### Listing content in s3 bucket");
    let results = bucket.list("".to_owned(), Some("/".to_owned())).await?;
    for result in results {
        for item in result.contents {
            println!("key: {}", item.key);
        }
    }

    //  5. Get object from bucket
    println!("###### Getting content from s3 bucket");
    let resp = bucket.get_object(key).await?;
    let data = std::str::from_utf8(&resp.as_slice()).expect("Wrong data returned");
    println!("data: {}", data);

    //  6. Delete object from bucket
    println!("###### Deleting content in s3 bucket");
    let resp = bucket.delete_object(key).await?;
    println!("resp: {}", resp);

    Ok(())
}
