use data_shapes::RepoCommands;
use tonic::Request;
use vps_rpc::{rpc::service::service_action_client::ServiceActionClient, RPC_ADDR};

pub async fn handle_repo_arg(arg: &RepoCommands) {
    let mut service_client = ServiceActionClient::connect(RPC_ADDR).await.unwrap();

    match arg {
        RepoCommands::List => {
            let list = service_client
                .list(Request::new(()))
                .await
                .unwrap()
                .into_inner()
                .services
                .iter()
                .map(|e| e.service_name.to_owned())
                .collect::<Vec<String>>();
            println!("{:?}", list);
        }
        RepoCommands::Clone => {
            // clone_all();
        }
        RepoCommands::Pull => {
            // TODO: unique urls
            // let unique_paths: Vec<String> = list
            //     .into_iter()
            //     .map(|e| e.relative_root)
            //     .collect::<HashSet<_>>()
            //     .into_iter()
            //     .collect();
            // for path in unique_paths {
            // pull_single(&path);
            // }
            println!("Pulling done");
        }
        RepoCommands::Build(e) => {
            // handle_build(e);
        }
    }
}
