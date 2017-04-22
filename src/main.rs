extern crate rls_span as span;
extern crate rls_analysis as analysis;

use std::path::Path;

fn print_doc(host: &analysis::AnalysisHost, id: u32) {
    let def = host.get_def(id).unwrap();
    println!("{:?}", def.kind);
    println!("name: {}", def.name);
    println!("api crate: {}", def.api_crate);
    println!("sig: {:?}", def.sig);

    let mut parents = vec![];
    let mut node_id = def.parent;
    while let Some(pid) = node_id {
        if let Ok(parent_def) = host.get_def(pid) {
            parents.push(parent_def.name.clone());
            node_id = parent_def.parent;
        }
        else {
            println!("Issue with: {}", pid);
            break;
        }
    }
    let breadcrumbs: Vec<&String> = parents.iter().rev().collect();
    println!("breadcrumbs: {:?}", breadcrumbs);

    for line in def.docs.clone().lines() {
        println!("{}", line);
    }
}

fn main() {
    use std::env;
    // Simple program, a somewhat thorough test that we have all the defs and refs we expect.
    let host = analysis::AnalysisHost::new(analysis::Target::Debug);
    host.reload(Path::new("test_data"), Path::new("test_data"), true).unwrap();

    let args: Vec<String> = env::args().collect();

    if let Some(name) = args.get(1) {
        if let Ok(ids) = host.search_for_id(name) {
            println!("ids: {:?}", ids);
            for id in ids {
                print_doc(&host, id);
            }
        }
    }
}
