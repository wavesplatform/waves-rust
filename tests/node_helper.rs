use std::str::FromStr;
use testcontainers::core::WaitFor;
use testcontainers::images::generic::GenericImage;
use testcontainers::{clients, Container};
use url::Url;
use waves_rust::api::Node;

// static NODE: Lazy<()> = Lazy::new(|| {
//     //let _ = pretty_env_logger::try_init();
//     let docker = clients::Cli::default();
//
//     let wait_for = WaitFor::message_on_stdout("New height: 10");
//     let image = GenericImage::new("wavesplatform/waves-private-node", "v1.4.8")
//         .with_wait_for(wait_for.clone());
//     let container = docker.run(image.clone());
//     container.;
//     // let host_port = node_container.get_host_port_ipv4(6869);
//     // let url = format!("http://127.0.0.1:{}", host_port);
//     // println!("{}", url);
//     // Node::from_url(Url::from_str(&url).unwrap(), b'R')
// });
//
// pub struct IntegrationTest {
//     container: Container<'static GenericImage>,
// }

#[ignore]
#[tokio::test]
async fn hm() {
    // let node = Lazy::force(&NODE);
    // let addresses = node.get_addresses().await.unwrap();
    // println!("{:?}", addresses);
    let docker = clients::Cli::default();

    let wait_for = WaitFor::message_on_stdout("New height: 10");
    let image = GenericImage::new("wavesplatform/waves-private-node", "v1.4.8")
        .with_wait_for(wait_for.clone());
    let node_container = docker.run(image.clone());
    let host_port = node_container.get_host_port_ipv4(6869);
    let url = format!("http://127.0.0.1:{}", host_port);
    println!("{}", url);
    let node = Node::from_url(Url::from_str(&url).unwrap(), b'R');
    let addresses = node.get_addresses().await.unwrap();
    println!("{:?}", addresses);
}

#[test]
fn cmd() {
    use std::process::Command;

    let mut list_dir = Command::new("docker")
        .arg("ps")
        .arg("-a")
        .arg("--format")
        .arg(r#""{{.ID}}""#)
        .output()
        .expect("faielr");
    let vec = String::from_utf8(list_dir.stdout);
    println!("ls: {:?}", vec);
}
